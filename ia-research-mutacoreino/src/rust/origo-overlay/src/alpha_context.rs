//! AlphaContext - Canonização e Isomorfismo Alfa-Estrutural
//!
//! Implementa a redução alfa (α-conversão) para obter formas canônicas
// de termos lambda, permitindo comparação estrutural independente
//! de nomes de variáveis ligadas.

use std::collections::HashMap;
use crate::{AstNode, AstNodeKind};

/// Contexto para canonização alfa-estrutural
///
/// Mantém um mapa de renomeação de variáveis ligadas para nomes
/// canônicos sequenciais (α_1, α_2, ...), garantindo que
/// termos alfa-equivalentes tenham a mesma representação canônica.
#[derive(Debug, Clone, Default)]
pub struct AlphaContext {
    /// Mapa de nome original -> nome canônico
    rename_map: HashMap<String, String>,
    /// Contador para gerar nomes canônicos únicos
    counter: usize,
    /// Pilha de escopos para suporte a shadowing
    scope_stack: Vec<HashMap<String, String>>,
}

impl AlphaContext {
    /// Cria um novo contexto vazio
    pub fn new() -> Self {
        Self {
            rename_map: HashMap::new(),
            counter: 0,
            scope_stack: Vec::new(),
        }
    }

    /// Cria um contexto com capacidade pré-alocada
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rename_map: HashMap::with_capacity(capacity),
            counter: 0,
            scope_stack: Vec::new(),
        }
    }

    /// Canoniza um nó AST, retornando sua forma alfa-estrutural
    ///
    /// Variáveis ligadas são renomeadas para α_1, α_2, etc. na ordem
    /// em que aparecem (pre-order traversal). Variáveis livres mantêm
    /// seus nomes originais.
    pub fn canonicalize(&mut self, node: &AstNode) -> AstNode {
        self.canonicalize_with_scope(node, &mut Vec::new())
    }

    /// Canoniza com rastreamento de escopo explícito
    fn canonicalize_with_scope(&mut self, node: &AstNode, bound_vars: &mut Vec<String>) -> AstNode {
        match node {
            AstNode::Variable(name) => {
                // Variável ligada: usa nome canônico do escopo atual
                if let Some(pos) = bound_vars.iter().rposition(|v| v == name) {
                    let canonical_name = format!("α_{}", pos + 1);
                    AstNode::Variable(canonical_name)
                } else {
                    // Variável livre: mantém nome original
                    AstNode::Variable(name.clone())
                }
            }
            AstNode::Abstraction(param, body) => {
                // Entra novo escopo
                bound_vars.push(param.clone());

                let canonical_body = self.canonicalize_with_scope(body, bound_vars);

                // Sai do escopo
                bound_vars.pop();

                AstNode::Abstraction(format!("α_{}", bound_vars.len()), Box::new(canonical_body))
            }
            AstNode::Application(fun, arg) => AstNode::Application(
                Box::new(self.canonicalize_with_scope(fun, bound_vars)),
                Box::new(self.canonicalize_with_scope(arg, bound_vars)),
            ),
            AstNode::Constant(val) => AstNode::Constant(*val),
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => AstNode::KrajicekGenerator {
                proof_len: *proof_len,
                threshold: *threshold,
                generator_type: *generator_type,
            },
        }
    }

    /// Canoniza múltiplos nós compartilhando o mesmo contexto
    /// (útil para comparar termos no mesmo ambiente)
    pub fn canonicalize_batch(&mut self, nodes: &[AstNode]) -> Vec<AstNode> {
        nodes.iter().map(|n| self.canonicalize(n)).collect()
    }

    /// Verifica isomorfismo alfa-estrutural entre dois nós
    ///
    /// Dois termos são alfa-isomórficos se suas formas canônicas
    /// são idênticas.
    pub fn is_alpha_isomorphic(&mut self, a: &AstNode, b: &AstNode) -> bool {
        let canon_a = self.canonicalize(a);
        let canon_b = self.canonicalize(b);
        canon_a == canon_b
    }

    /// Obtém o hash estrutural de um nó (após canonização)
    pub fn structural_hash(&mut self, node: &AstNode) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let canon = self.canonicalize(node);
        let mut hasher = DefaultHasher::new();
        canon.hash(&mut hasher);
        hasher.finish()
    }

    /// Reseta o contador (útil para nova sessão de canonização)
    pub fn reset(&mut self) {
        self.rename_map.clear();
        self.counter = 0;
        self.scope_stack.clear();
    }

    /// Retorna o número de variáveis ligadas processadas
    pub fn bound_count(&self) -> usize {
        self.counter
    }

    /// Clona o estado atual do mapa de renomeação
    pub fn snapshot(&self) -> HashMap<String, String> {
        self.rename_map.clone()
    }

    /// Restaura um snapshot anterior
    pub fn restore(&mut self, snapshot: HashMap<String, String>) {
        self.rename_map = snapshot;
        self.counter = self.rename_map.len();
    }
}

/// Contexto alfa com cache de hash para performance
#[derive(Debug, Clone)]
pub struct CachedAlphaContext {
    inner: AlphaContext,
    hash_cache: HashMap<u64, u64>, // hash do nó original -> hash canônico
}

impl CachedAlphaContext {
    pub fn new() -> Self {
        Self {
            inner: AlphaContext::new(),
            hash_cache: HashMap::new(),
        }
    }

    pub fn canonicalize(&mut self, node: &AstNode) -> AstNode {
        self.inner.canonicalize(node)
    }

    pub fn structural_hash(&mut self, node: &AstNode) -> u64 {
        // Usa hash do ponteiro/identidade como chave de cache
        // Nota: em produção, usar uma chave mais robusta
        let node_ptr = node as *const AstNode as u64;

        if let Some(&cached) = self.hash_cache.get(&node_ptr) {
            return cached;
        }

        let hash = self.inner.structural_hash(node);
        self.hash_cache.insert(node_ptr, hash);
        hash
    }

    pub fn is_alpha_isomorphic(&mut self, a: &AstNode, b: &AstNode) -> bool {
        self.inner.is_alpha_isomorphic(a, b)
    }

    pub fn reset(&mut self) {
        self.inner.reset();
        self.hash_cache.clear();
    }
}

impl Default for CachedAlphaContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AstNode;

    #[test]
    fn test_basic_canonicalization() {
        let mut ctx = AlphaContext::new();

        // λx.x → λα_1.α_1
        let ast = AstNode::abstraction("x", AstNode::variable("x"));
        let canon = ctx.canonicalize(&ast);
        assert_eq!(canon.to_string(), "λα_1.α_1");
    }

    #[test]
    fn test_alpha_equivalence() {
        let mut ctx = AlphaContext::new();

        // λx.x ≡ λy.y
        let ast1 = AstNode::abstraction("x", AstNode::variable("x"));
        let ast2 = AstNode::abstraction("y", AstNode::variable("y"));

        assert!(ctx.is_alpha_isomorphic(&ast1, &ast2));
    }

    #[test]
    fn test_nested_abstractions() {
        let mut ctx = AlphaContext::new();

        // λx.λy.x y
        let ast = AstNode::abstraction(
            "x",
            AstNode::abstraction(
                "y",
                AstNode::application(
                    AstNode::variable("x"),
                    AstNode::variable("y"),
                ),
            ),
        );

        let canon = ctx.canonicalize(&ast);
        // Deve ser: λα_1.λα_2.(α_1 α_2)
        assert!(canon.to_string().contains("α_1"));
        assert!(canon.to_string().contains("α_2"));
    }

    #[test]
 fn test_free_variables_preserved() {
        let mut ctx = AlphaContext::new();

        // λx.y (y é livre)
        let ast = AstNode::abstraction("x", AstNode::variable("y"));
        let canon = ctx.canonicalize(&ast);

        // y deve permanecer como y (livre)
        assert!(canon.to_string().contains("y"));
        assert!(!canon.to_string().contains("α_"));
    }

    #[test]
    fn test_shadowing() {
        let mut ctx = AlphaContext::new();

        // λx.λx.x (shadowing: x interno ≠ x externo)
        let ast = AstNode::abstraction(
            "x",
            AstNode::abstraction("x", AstNode::variable("x")),
        );

        let canon = ctx.canonicalize(&ast);
        // Deve distinguir os dois x's
        assert_eq!(canon.to_string(), "λα_1.λα_1.α_1"); // x interno é α_1
    }

    #[test]
    fn test_complex_alpha_equivalence() {
        let mut ctx = AlphaContext::new();

        // (λx.λy.x) ≡ (λa.λb.a)
        let ast1 = AstNode::abstraction(
            "x",
            AstNode::abstraction("y", AstNode::variable("x")),
        );
        let ast2 = AstNode::abstraction(
            "a",
            AstNode::abstraction("b", AstNode::variable("a")),
        );

        assert!(ctx.is_alpha_isomorphic(&ast1, &ast2));
    }

    #[test]
    fn test_non_equivalent() {
        let mut ctx = AlphaContext::new();

        // λx.λy.x ≢ λx.λy.y
        let ast1 = AstNode::abstraction(
            "x",
            AstNode::abstraction("y", AstNode::variable("x")),
        );
        let ast2 = AstNode::abstraction(
            "x",
            AstNode::abstraction("y", AstNode::variable("y")),
        );

        assert!(!ctx.is_alpha_isomorphic(&ast1, &ast2));
    }

    #[test]
    fn test_krajicek_generator_preserved() {
        let mut ctx = AlphaContext::new();

        let ast = AstNode::krajicek_generator(100, 200, crate::KrajicekGeneratorType::RhoT);
        let canon = ctx.canonicalize(&ast);

        assert!(matches!(canon, AstNode::KrajicekGenerator { .. }));
    }

    #[test]
    fn test_cached_context() {
        let mut ctx = CachedAlphaContext::new();

        let ast = AstNode::abstraction("x", AstNode::variable("x"));
        let hash1 = ctx.structural_hash(&ast);
        let hash2 = ctx.structural_hash(&ast);

        assert_eq!(hash1, hash2); // Cache hit
    }
}
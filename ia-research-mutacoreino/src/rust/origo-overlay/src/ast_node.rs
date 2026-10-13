//! Definições dos Nós da AST (Árvore de Sintaxe Abstrata)
//!
//! Representação alfa-estrutural de termos lambda com suporte a
//! geradores de Krajicek para monitoramento de complexidade de prova.

use std::fmt;
use serde::{Deserialize, Serialize};

/// Tipo de nó na AST para pattern matching rápido
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AstNodeKind {
    Variable,
    Abstraction,
    Application,
    Constant,
    KrajicekGenerator,
}

/// Nó da AST alfa-estrutural
///
/// A estrutura é recursiva e suporta:
/// - Variáveis com nomes (canonizadas via AlphaContext)
/// - Abstrações (λ-param.body) com binding lexical
/// - Aplicações (fun arg)
/// - Constantes numéricas
/// - Geradores de Krajicek para rastreamento de complexidade de prova
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstNode {
    Variable(String),
    Abstraction(String, Box<AstNode>),
    Application(Box<AstNode>, Box<AstNode>),
    Constant(u64),
    KrajicekGenerator {
        proof_len: usize,
        threshold: usize,
        generator_type: KrajicekGeneratorType,
    },
}

/// Tipo de gerador de Krajicek
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KrajicekGeneratorType {
    /// ρ_T - Limiar de colagem de prova
    RhoT,
    /// B_T - Limiar de complexidade base
    BT,
    /// C_T - Limiar de contração
    CT,
    /// R_T - Limiar de reflexão
    RT,
}

impl AstNode {
    /// Cria uma variável
    pub fn variable(name: impl Into<String>) -> Self {
        AstNode::Variable(name.into())
    }

    /// Cria uma abstração λ-param.body
    pub fn abstraction(param: impl Into<String>, body: AstNode) -> Self {
        AstNode::Abstraction(param.into(), Box::new(body))
    }

    /// Cria uma aplicação (fun arg)
    pub fn application(fun: AstNode, arg: AstNode) -> Self {
        AstNode::Application(Box::new(fun), Box::new(arg))
    }

    /// Cria uma constante
    pub fn constant(value: u64) -> Self {
        AstNode::Constant(value)
    }

    /// Cria um gerador de Krajicek
    pub fn krajicek_generator(
        proof_len: usize,
        threshold: usize,
        generator_type: KrajicekGeneratorType,
    ) -> Self {
        AstNode::KrajicekGenerator {
            proof_len,
            threshold,
            generator_type,
        }
    }

    /// Retorna o tipo do nó
    pub fn kind(&self) -> AstNodeKind {
        match self {
            AstNode::Variable(_) => AstNodeKind::Variable,
            AstNode::Abstraction(_, _) => AstNodeKind::Abstraction,
            AstNode::Application(_, _) => AstNodeKind::Application,
            AstNode::Constant(_) => AstNodeKind::Constant,
            AstNode::KrajicekGenerator { .. } => AstNodeKind::KrajicekGenerator,
        }
    }

    /// Verifica se é um gerador de Krajicek
    pub fn is_krajicek_generator(&self) -> bool {
        matches!(self, AstNode::KrajicekGenerator { .. })
    }

    /// Obtém informações do gerador se for um
    pub fn krajicek_info(&self) -> Option<(usize, usize, KrajicekGeneratorType)> {
        match self {
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => Some((*proof_len, *threshold, *generator_type)),
            _ => None,
        }
    }

    /// Conta o número total de nós na árvore
    pub fn node_count(&self) -> usize {
        match self {
            AstNode::Variable(_) | AstNode::Constant(_) => 1,
            AstNode::KrajicekGenerator { .. } => 1,
            AstNode::Abstraction(_, body) => 1 + body.node_count(),
            AstNode::Application(fun, arg) => 1 + fun.node_count() + arg.node_count(),
        }
    }

    /// Calcula a profundidade máxima da árvore
    pub fn depth(&self) -> usize {
        match self {
            AstNode::Variable(_) | AstNode::Constant(_) | AstNode::KrajicekGenerator { .. } => 1,
            AstNode::Abstraction(_, body) => 1 + body.depth(),
            AstNode::Application(fun, arg) => 1 + fun.depth().max(arg.depth()),
        }
    }

    /// Coleta todas as variáveis livres
    pub fn free_variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_free_vars(&mut vars, &mut Vec::new());
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_free_vars(&self, free: &mut Vec<String>, bound: &mut Vec<String>) {
        match self {
            AstNode::Variable(name) => {
                if !bound.contains(name) {
                    free.push(name.clone());
                }
            }
            AstNode::Abstraction(param, body) => {
                bound.push(param.clone());
                body.collect_free_vars(free, bound);
                bound.pop();
            }
            AstNode::Application(fun, arg) => {
                fun.collect_free_vars(free, bound);
                arg.collect_free_vars(free, bound);
            }
            AstNode::Constant(_) | AstNode::KrajicekGenerator { .. } => {}
        }
    }

    /// Substitui uma variável livre por outro termo
    pub fn substitute(&self, var: &str, replacement: &AstNode) -> AstNode {
        match self {
            AstNode::Variable(name) if name == var => replacement.clone(),
            AstNode::Variable(name) => AstNode::Variable(name.clone()),
            AstNode::Abstraction(param, body) => {
                if param == var {
                    // Variável ligada, não substitui no corpo
                    AstNode::Abstraction(param.clone(), body.clone())
                } else {
                    // Evita captura de variáveis
                    let new_body = body.substitute(var, replacement);
                    AstNode::Abstraction(param.clone(), Box::new(new_body))
                }
            }
            AstNode::Application(fun, arg) => AstNode::Application(
                Box::new(fun.substitute(var, replacement)),
                Box::new(arg.substitute(var, replacement)),
            ),
            AstNode::Constant(_) | AstNode::KrajicekGenerator { .. } => self.clone(),
        }
    }

    /// Aplica β-redução se possível (uma etapa)
    pub fn beta_reduce_once(&self) -> Option<AstNode> {
        match self {
            AstNode::Application(fun, arg) => match fun.as_ref() {
                AstNode::Abstraction(param, body) => Some(body.substitute(param, arg)),
                _ => None,
            },
            _ => None,
        }
    }

    /// Normalização β completa (com limite de passos para evitar loops)
    pub fn beta_normalize(&self, max_steps: usize) -> AstNode {
        let mut current = self.clone();
        for _ in 0..max_steps {
            if let Some(reduced) = current.beta_reduce_once() {
                current = reduced;
            } else {
                break;
            }
        }
        current
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Variable(name) => write!(f, "{}", name),
            AstNode::Abstraction(param, body) => write!(f, "λ{}.{}", param, body),
            AstNode::Application(fun, arg) => write!(f, "({} {})", fun, arg),
            AstNode::Constant(val) => write!(f, "{}", val),
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => write!(
                f,
                "⟨K:{:?} proof_len={} threshold={}⟩",
                generator_type, proof_len, threshold
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_construction() {
        let ast = AstNode::abstraction("x", AstNode::variable("x"));
        assert_eq!(ast.kind(), AstNodeKind::Abstraction);
        assert_eq!(ast.to_string(), "λx.x");
    }

    #[test]
    fn test_application() {
        let ast = AstNode::application(
            AstNode::variable("f"),
            AstNode::variable("x"),
        );
        assert_eq!(ast.to_string(), "(f x)");
    }

    #[test]
    fn test_nested_abstraction() {
        let ast = AstNode::abstraction(
            "x",
            AstNode::abstraction("y", AstNode::variable("x")),
        );
        assert_eq!(ast.to_string(), "λx.λy.x");
    }

    #[test]
    fn test_free_variables() {
        let ast = AstNode::abstraction(
            "x",
            AstNode::application(
                AstNode::variable("x"),
                AstNode::variable("y"),
            ),
        );
        let free = ast.free_variables();
        assert_eq!(free, vec!["y"]);
    }

    #[test]
    fn test_substitution() {
        let ast = AstNode::abstraction("x", AstNode::variable("y"));
        let replaced = ast.substitute("y", &AstNode::constant(42));
        assert_eq!(replaced.to_string(), "λx.42");
    }

    #[test]
    fn test_beta_reduction() {
        // (λx.x) y → y
        let ast = AstNode::application(
            AstNode::abstraction("x", AstNode::variable("x")),
            AstNode::variable("y"),
        );
        let reduced = ast.beta_reduce_once().unwrap();
        assert_eq!(reduced, AstNode::variable("y"));
    }

    #[test]
    fn test_krajicek_generator() {
        let ast = AstNode::krajicek_generator(100, 200, KrajicekGeneratorType::RhoT);
        assert!(ast.is_krajicek_generator());
        assert_eq!(ast.krajicek_info(), Some((100, 200, KrajicekGeneratorType::RhoT)));
    }

    #[test]
    fn test_node_count_and_depth() {
        // λx.λy.(x y)
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
        assert_eq!(ast.node_count(), 5); // 2 abstractions + 1 app + 2 vars
        assert_eq!(ast.depth(), 4);
    }
}
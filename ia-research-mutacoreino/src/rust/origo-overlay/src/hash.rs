//! Structural Hashing - Hash Alfa-Estrutural para ASTs
//!
//! Implementa hashing que é invariante sob renomeação alfa,
//! permitindo que termos alfa-equivalentes tenham o mesmo hash.
//!
//! Usa técnica de hash baseada em De Bruijn indices internamente
//! para canonização estrutural eficiente.

use crate::{AstNode, AstNodeKind};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

/// Hash alfa-estrutural (64 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlphaHash(pub u64);

impl AlphaHash {
    /// Hash zero (vazio)
    pub const ZERO: Self = AlphaHash(0);

    /// Cria a partir de u64
    pub fn new(value: u64) -> Self {
        AlphaHash(value)
    }

    /// Valor interno
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Combina dois hashes
    pub fn combine(self, other: AlphaHash) -> AlphaHash {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        other.0.hash(&mut hasher);
        AlphaHash(hasher.finish())
    }

    /// Combina com salt
    pub fn combine_with_salt(self, salt: u64) -> AlphaHash {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        salt.hash(&mut hasher);
        AlphaHash(hasher.finish())
    }
}

impl std::fmt::Display for AlphaHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl From<u64> for AlphaHash {
    fn from(v: u64) -> Self {
        AlphaHash(v)
    }
}

impl From<AlphaHash> for u64 {
    fn from(h: AlphaHash) -> Self {
        h.0
    }
}

/// Hasher estrutural para ASTs
pub struct StructuralHasher;

impl StructuralHasher {
    /// Computa hash alfa-estrutural de um nó AST
    pub fn hash(node: &AstNode) -> u64 {
        let mut hasher = StructuralHasherInner::new();
        node.hash_structural(&mut hasher);
        hasher.finish()
    }

    /// Computa hash com seed personalizado
    pub fn hash_with_seed(node: &AstNode, seed: u64) -> u64 {
        let mut hasher = StructuralHasherInner::with_seed(seed);
        node.hash_structural(&mut hasher);
        hasher.finish()
    }

    /// Computa hash de múltiplos nós (ordem importa)
    pub fn hash_sequence(nodes: &[AstNode]) -> u64 {
        let mut hasher = StructuralHasherInner::new();
        for node in nodes {
            node.hash_structural(&mut hasher);
        }
        hasher.finish()
    }

    /// Verifica igualdade estrutural via hash
    pub fn structurally_equal(a: &AstNode, b: &AstNode) -> bool {
        Self::hash(a) == Self::hash(b)
    }
}

/// Hasher interno com estado para índices De Bruijn
struct StructuralHasherInner {
    hasher: DefaultHasher,
    /// Pilha de variáveis ligadas para conversão De Bruijn
    bound_vars: Vec<String>,
    /// Contador de profundidade
    depth: usize,
}

impl StructuralHasherInner {
    fn new() -> Self {
        Self {
            hasher: DefaultHasher::new(),
            bound_vars: Vec::new(),
            depth: 0,
        }
    }

    fn with_seed(seed: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        Self {
            hasher,
            bound_vars: Vec::new(),
            depth: 0,
        }
    }

    fn finish(self) -> u64 {
        self.hasher.finish()
    }

    /// Escreve tag do tipo de nó
    fn write_node_kind(&mut self, kind: AstNodeKind) {
        (kind as u8).hash(&mut self.hasher);
    }

    /// Processa variável (converte para índice De Bruijn)
    fn process_variable(&mut self, name: &str) {
        // Procura da mais recente para mais antiga (shadowing)
        if let Some(idx) = self.bound_vars.iter().rposition(|v| v == name) {
            // Variável ligada: usa índice De Bruijn (distância do topo)
            let de_bruijn_idx = self.bound_vars.len() - 1 - idx;
            self.write_node_kind(AstNodeKind::Variable);
            de_bruijn_idx.hash(&mut self.hasher);
        } else {
            // Variável livre: usa nome (hash do nome)
            self.write_node_kind(AstNodeKind::Variable);
            name.hash(&mut self.hasher);
        }
    }

    /// Entra em abstração (novo binding)
    fn enter_abstraction(&mut self, param: &str) {
        self.bound_vars.push(param.to_string());
        self.depth += 1;
    }

    /// Sai de abstração
    fn exit_abstraction(&mut self) {
        self.bound_vars.pop();
        self.depth -= 1;
    }
}

/// Trait para hash estrutural de AstNode
trait HashStructural {
    fn hash_structural(&self, hasher: &mut StructuralHasherInner);
}

impl HashStructural for AstNode {
    fn hash_structural(&self, hasher: &mut StructuralHasherInner) {
        match self {
            AstNode::Variable(name) => {
                hasher.process_variable(name);
            }
            AstNode::Abstraction(param, body) => {
                hasher.write_node_kind(AstNodeKind::Abstraction);
                hasher.enter_abstraction(param);
                body.hash_structural(hasher);
                hasher.exit_abstraction();
            }
            AstNode::Application(fun, arg) => {
                hasher.write_node_kind(AstNodeKind::Application);
                fun.hash_structural(hasher);
                arg.hash_structural(hasher);
            }
            AstNode::Constant(val) => {
                hasher.write_node_kind(AstNodeKind::Constant);
                val.hash(&mut hasher.hasher);
            }
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => {
                hasher.write_node_kind(AstNodeKind::KrajicekGenerator);
                proof_len.hash(&mut hasher.hasher);
                threshold.hash(&mut hasher.hasher);
                (*generator_type as u8).hash(&mut hasher.hasher);
            }
        }
    }
}

/// Hash de Merkle Tree para ASTs grandes
pub struct MerkleHasher {
    leaf_hashes: Vec<u64>,
}

impl MerkleHasher {
    pub fn new() -> Self {
        Self { leaf_hashes: Vec::new() }
    }

    /// Adiciona folha (hash de sub-árvore)
    pub fn add_leaf(&mut self, hash: u64) {
        self.leaf_hashes.push(hash);
    }

    /// Computa raiz da árvore de Merkle
    pub fn root_hash(&self) -> u64 {
        if self.leaf_hashes.is_empty() {
            return 0;
        }

        let mut current_level = self.leaf_hashes.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    Self::combine_hashes(chunk[0], chunk[1])
                } else {
                    chunk[0]
                };
                next_level.push(combined);
            }
            current_level = next_level;
        }

        current_level[0]
    }

    fn combine_hashes(left: u64, right: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        left.hash(&mut hasher);
        right.hash(&mut hasher);
        hasher.finish()
    }

    /// Prova de inclusão (Merkle proof)
    pub fn generate_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if leaf_index >= self.leaf_hashes.len() {
            return None;
        }

        let mut proof = Vec::new();
        let mut current_level = self.leaf_hashes.clone();
        let mut index = leaf_index;

        while current_level.len() > 1 {
            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };

            if sibling_index < current_level.len() {
                proof.push(MerkleProofStep {
                    hash: current_level[sibling_index],
                    is_left: sibling_index < index,
                });
            }

            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    Self::combine_hashes(chunk[0], chunk[1])
                } else {
                    chunk[0]
                };
                next_level.push(combined);
            }
            current_level = next_level;
            index /= 2;
        }

        Some(MerkleProof { root: current_level[0], steps: proof })
    }

    /// Verifica prova de Merkle
    pub fn verify_proof(leaf_hash: u64, proof: &MerkleProof) -> bool {
        let mut current = leaf_hash;
        for step in &proof.steps {
            current = if step.is_left {
                Self::combine_hashes(step.hash, current)
            } else {
                Self::combine_hashes(current, step.hash)
            };
        }
        current == proof.root
    }
}

impl Default for MerkleHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Passo da prova de Merkle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofStep {
    pub hash: u64,
    pub is_left: bool,
}

/// Prova de Merkle completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub root: u64,
    pub steps: Vec<MerkleProofStep>,
}

/// Hash incrementais para streaming
pub struct IncrementalHasher {
    hasher: DefaultHasher,
    node_count: usize,
}

impl IncrementalHasher {
    pub fn new() -> Self {
        Self {
            hasher: DefaultHasher::new(),
            node_count: 0,
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        Self { hasher, node_count: 0 }
    }

    pub fn add_node(&mut self, node: &AstNode) {
        let hash = StructuralHasher::hash(node);
        hash.hash(&mut self.hasher);
        self.node_count += 1;
    }

    pub fn add_hash(&mut self, hash: u64) {
        hash.hash(&mut self.hasher);
        self.node_count += 1;
    }

    pub fn finalize(self) -> AlphaHash {
        AlphaHash(self.hasher.finish())
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }
}

impl Default for IncrementalHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash de consenso para validação distribuída
pub fn consensus_hash(nodes: &[&AstNode]) -> AlphaHash {
    let mut hashes: Vec<u64> = nodes.iter().map(|n| StructuralHasher::hash(n)).collect();
    hashes.sort_unstable();

    let mut hasher = DefaultHasher::new();
    for h in hashes {
        h.hash(&mut hasher);
    }
    AlphaHash(hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AstNode;

    #[test]
    fn test_alpha_equivalent_hash() {
        // λx.x ≡ λy.y
        let ast1 = AstNode::abstraction("x", AstNode::variable("x"));
        let ast2 = AstNode::abstraction("y", AstNode::variable("y"));

        let hash1 = StructuralHasher::hash(&ast1);
        let hash2 = StructuralHasher::hash(&ast2);

        assert_eq!(hash1, hash2, "Termos alfa-equivalentes devem ter mesmo hash");
    }

    #[test]
    fn test_non_equivalent_different_hash() {
        // λx.λy.x ≢ λx.λy.y
        let ast1 = AstNode::abstraction("x", AstNode::abstraction("y", AstNode::variable("x")));
        let ast2 = AstNode::abstraction("x", AstNode::abstraction("y", AstNode::variable("y")));

        let hash1 = StructuralHasher::hash(&ast1);
        let hash2 = StructuralHasher::hash(&ast2);

        assert_ne!(hash1, hash2, "Termos não-equivalentes devem ter hashes diferentes");
    }

    #[test]
    fn test_complex_alpha_equivalence() {
        // (λx.λy.x y) ≡ (λa.λb.a b)
        let ast1 = AstNode::abstraction(
            "x",
            AstNode::abstraction("y", AstNode::application(AstNode::variable("x"), AstNode::variable("y"))),
        );
        let ast2 = AstNode::abstraction(
            "a",
            AstNode::abstraction("b", AstNode::application(AstNode::variable("a"), AstNode::variable("b"))),
        );

        assert_eq!(StructuralHasher::hash(&ast1), StructuralHasher::hash(&ast2));
    }

    #[test]
    fn test_free_variables_affect_hash() {
        // λx.y ≢ λx.z (y e z são livres)
        let ast1 = AstNode::abstraction("x", AstNode::variable("y"));
        let ast2 = AstNode::abstraction("x", AstNode::variable("z"));

        assert_ne!(StructuralHasher::hash(&ast1), StructuralHasher::hash(&ast2));
    }

    #[test]
    fn test_shadowing() {
        // λx.λx.x (x interno shadowea externo)
        let ast = AstNode::abstraction("x", AstNode::abstraction("x", AstNode::variable("x")));
        let hash = StructuralHasher::hash(&ast);

        // Deve ser diferente de λx.λy.x
        let ast2 = AstNode::abstraction("x", AstNode::abstraction("y", AstNode::variable("x")));
        assert_ne!(hash, StructuralHasher::hash(&ast2));
    }

    #[test]
    fn test_krajicek_generator_hash() {
        let ast1 = AstNode::krajicek_generator(100, 200, crate::KrajicekGeneratorType::RhoT);
        let ast2 = AstNode::krajicek_generator(100, 200, crate::KrajicekGeneratorType::RhoT);
        let ast3 = AstNode::krajicek_generator(100, 200, crate::KrajicekGeneratorType::BT);

        assert_eq!(StructuralHasher::hash(&ast1), StructuralHasher::hash(&ast2));
        assert_ne!(StructuralHasher::hash(&ast1), StructuralHasher::hash(&ast3));
    }

    #[test]
    fn test_merkle_tree() {
        let mut merkle = MerkleHasher::new();
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("x")));
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("y")));
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("z")));

        let root = merkle.root_hash();
        assert_ne!(root, 0);

        let proof = merkle.generate_proof(1).unwrap();
        assert!(MerkleHasher::verify_proof(merkle.leaf_hashes[1], &proof));
    }

    #[test]
    fn test_incremental_hasher() {
        let mut inc = IncrementalHasher::new();
        inc.add_node(&AstNode::variable("x"));
        inc.add_node(&AstNode::variable("y"));
        let hash1 = inc.finalize();

        let mut inc2 = IncrementalHasher::new();
        inc2.add_node(&AstNode::variable("x"));
        inc2.add_node(&AstNode::variable("y"));
        let hash2 = inc2.finalize();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_consensus_hash() {
        let ast1 = AstNode::variable("x");
        let ast2 = AstNode::variable("y");
        let ast3 = AstNode::variable("z");

        let hash1 = consensus_hash(&[&ast1, &ast2, &ast3]);
        let hash2 = consensus_hash(&[&ast3, &ast1, &ast2]); // Ordem diferente

        assert_eq!(hash1, hash2, "Consensus hash deve ser invariante à ordem");
    }

    #[test]
    fn test_hash_with_seed() {
        let ast = AstNode::variable("x");
        let hash1 = StructuralHasher::hash_with_seed(&ast, 42);
        let hash2 = StructuralHasher::hash_with_seed(&ast, 42);
        let hash3 = StructuralHasher::hash_with_seed(&ast, 43);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
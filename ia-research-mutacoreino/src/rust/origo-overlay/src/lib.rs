//! Origo Overlay - Overlays Declarativos Seguros com ASTs Alfa-Estruturais
//!
//! Este crate implementa a Camada 2 (Transporte, Comunicação e Distribuição) do MutaCore.
//! Fornece:
//! - **AlphaContext**: Canonização e isomorfismo alfa-estrutural de ASTs
//! - **OrigoAstValidator**: Validação de limiares de complexidade de prova (Krajicek)
//! - **KrajicekThresholds**: Gerenciamento do espectro ρ_T, B_T, C_T, R_T
//! - **QuantumLocalPacking**: Otimização de localidade e distribuição em nós de rede

pub mod alpha_context;
pub mod ast_validator;
pub mod ast_node;
pub mod krajicek_thresholds;
pub mod quantum_packing;
pub mod error;
pub mod hash;

pub use alpha_context::AlphaContext;
pub use ast_validator::{OrigoAstValidator, ValidationResult};
pub use ast_node::{AstNode, AstNodeKind};
pub use krajicek_thresholds::{KrajicekThresholds, ThresholdConfig};
pub use quantum_packing::{QuantumLocalPacker, PackingConfig, NodeInfo, WorkItem, WorkAssignment};
pub use error::{OrigoError, Result};
pub use hash::{AlphaHash, StructuralHasher, MerkleHasher, MerkleProof, MerkleProofStep, IncrementalHasher, consensus_hash};

/// Versão do crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuração padrão para validação
pub const DEFAULT_MAX_THRESHOLD: usize = 10_000;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_validation_pipeline() {
        let mut ctx = AlphaContext::new();
        let thresholds = KrajicekThresholds::default();
        let validator = OrigoAstValidator::new(thresholds);

        // Cria uma AST simples: λx. λy. x y
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

        // Canonização alfa
        let canonical = ctx.canonicalize(&ast);
        assert_eq!(canonical, ast); // Já está em forma canônica

        // Validação
        let result = validator.validate(&canonical);
        assert!(result.is_ok());
    }

    #[test]
    fn test_alpha_isomorphism() {
        let mut ctx = AlphaContext::new();

        // Dois termos alfa-equivalentes: λx.x e λy.y
        let ast1 = AstNode::abstraction("x", AstNode::variable("x"));
        let ast2 = AstNode::abstraction("y", AstNode::variable("y"));

        let canon1 = ctx.canonicalize(&ast1);
        let canon2 = ctx.canonicalize(&ast2);

        // Devem ter o mesmo hash estrutural
        let hash1 = StructuralHasher::hash(&canon1);
        let hash2 = StructuralHasher::hash(&canon2);
        assert_eq!(hash1, hash2);
    }
}

#[cfg(test)]
mod tests;
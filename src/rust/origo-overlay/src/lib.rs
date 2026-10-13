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
pub mod tests;

#[cfg(feature = "pyo3")]
pub mod pyo3_bindings;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;

#[cfg(feature = "ricemotions")]
pub mod ricemotions;

pub use alpha_context::AlphaContext;
pub use ast_validator::{OrigoAstValidator, ValidationResult};
pub use ast_node::{AstNode, AstNodeKind};
pub use krajicek_thresholds::{KrajicekThresholds, ThresholdConfig};
pub use quantum_packing::{QuantumLocalPacker, PackingConfig, NodeInfo, WorkItem, WorkAssignment};
pub use error::{OrigoError, Result};
pub use hash::{AlphaHash, StructuralHasher, MerkleHasher, MerkleProof, MerkleProofStep, IncrementalHasher, consensus_hash};

#[cfg(feature = "ricemotions")]
pub use ricemotions::{
    PixelState, RicCode, TeoaObject, LandauerMonitor, RicemotionsPipeline,
    EmotionalState, PIXEL_BITS, RIC_BITS, TEOA_BITS,
    pixel_encode, ric_compress, teoa_map,
    landauer_loss, metabolic_to_emotional, run_ricemotions_cycle, visualize_emotion,
};

/// Versão do crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuração padrão para validação
pub const DEFAULT_MAX_THRESHOLD: usize = 10_000;
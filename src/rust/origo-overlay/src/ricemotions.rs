//! Ricemotions Pipeline - Metabolismo Emocional Computacional
//!
//! Implementa o pipeline PIXEL → RIC → TEOA com monitoramento de Landauer
//! Target: < 100ms por ciclo metabólico

#[cfg(feature = "ricemotions")]
use libm::{log, exp};
#[cfg(feature = "ricemotions")]
use crate::{AstNode, StructuralHasher};

#[cfg(feature = "ricemotions")]
pub const K_B: f64 = 1.380649e-23;
#[cfg(feature = "ricemotions")]
pub const LN2: f64 = 0.6931471805599453;

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EmotionalState {
    Homeostasis,
    Exploration,
    Anxiety,
    Curiosity,
    Satisfaction,
}

#[cfg(feature = "ricemotions")]
impl std::fmt::Display for EmotionalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmotionalState::Homeostasis => write!(f, "HOMEOSTASIS"),
            EmotionalState::Exploration => write!(f, "EXPLORATION"),
            EmotionalState::Anxiety => write!(f, "ANXIETY"),
            EmotionalState::Curiosity => write!(f, "CURIOSITY"),
            EmotionalState::Satisfaction => write!(f, "SATISFACTION"),
        }
    }
}

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PixelState {
    pub tensor: Vec<f32>,           // 64x64 = 4096 elements
    pub timestamp: u64,
    pub entropy: f64,
    pub emotional_signature: String,
    pub metadata: std::collections::HashMap<String, String>,
}

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RicCode {
    pub compressed_ast: String,
    pub compression_ratio: f64,
    pub information_loss: f64,
    pub semantic_hash: String,
    pub metadata: std::collections::HashMap<String, String>,
}

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeoaObject {
    pub concept_id: String,
    pub semantic_vector: Vec<f32>,  // 512-dim
    pub epistemic_confidence: f64,
    pub abstract_relations: Vec<(String, String)>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LandauerMonitor {
    pub pixel_to_ric_loss: f64,
    pub ric_to_teoa_loss: f64,
    pub total_loss: f64,
    pub threshold: f64,
}

#[cfg(feature = "ricemotions")]
impl LandauerMonitor {
    pub fn check_threshold(&self) -> bool {
        self.total_loss > self.threshold
    }
}

#[cfg(feature = "ricemotions")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RicemotionsPipeline {
    pub pixel_input: PixelState,
    pub ric_intermediate: RicCode,
    pub teoa_output: TeoaObject,
    pub landauer_monitor: LandauerMonitor,
    pub metabolic_state: EmotionalState,
    pub cycle_time_ms: u64,
    pub timestamp: u64,
}

#[cfg(feature = "ricemotions")]
pub const PIXEL_BITS: f64 = 64.0 * 64.0 * 32.0;   // 64x64 float32
#[cfg(feature = "ricemotions")]
pub const RIC_BITS: f64 = 1024.0 * 8.0;            // 1KB compressed
#[cfg(feature = "ricemotions")]
pub const TEOA_BITS: f64 = 512.0 * 32.0;           // 512-dim float32

#[cfg(feature = "ricemotions")]
pub fn landauer_loss(input_bits: f64, output_bits: f64, temperature: f64) -> f64 {
    if input_bits <= output_bits {
        return 0.0;
    }
    (input_bits - output_bits) * K_B * temperature * LN2
}

#[cfg(feature = "ricemotions")]
pub fn pixel_encode(organism_state: &str, exogenous_data: &str) -> PixelState {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Seed determinístico
    let mut hasher = DefaultHasher::new();
    organism_state.hash(&mut hasher);
    exogenous_data.hash(&mut hasher);
    let seed = hasher.finish();
    
    // Gera tensor 64x64 determinístico
    let mut tensor = Vec::with_capacity(64 * 64);
    let mut x = seed as f64;
    for _ in 0..(64 * 64) {
        // Simple LCG for deterministic generation
        x = (x * 1664525.0 + 1013904223.0) % 4294967296.0;
        tensor.push((x / 4294967296.0 * 2.0 - 1.0) as f32);
    }
    
    // Entropia
    let entropy: f64 = tensor.iter()
        .map(|&v| {
            let av = v.abs() as f64 + 1e-10;
            -(v as f64) * libm::log(av)
        })
        .sum();
    
    // Assinatura emocional
    let mut sig_hasher = DefaultHasher::new();
    for v in &tensor {
        (v.to_bits() as u64).hash(&mut sig_hasher);
    }
    let emotional_signature = format!("{:x}", sig_hasher.finish())[..16].to_string();
    
    PixelState {
        tensor,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        entropy,
        emotional_signature,
        metadata: {
            let mut m = std::collections::HashMap::new();
            m.insert("organism_state".to_string(), organism_state.to_string());
            m.insert("exogenous_data".to_string(), exogenous_data.to_string());
            m
        },
    }
}

#[cfg(feature = "ricemotions")]
pub fn ric_compress(pixel: &PixelState, ast: &str) -> RicCode {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let input_bits = PIXEL_BITS;
    
    // Hash semântico combinado
    let mut hasher = DefaultHasher::new();
    ast.hash(&mut hasher);
    pixel.emotional_signature.hash(&mut hasher);
    let semantic_hash = format!("{:x}", hasher.finish());
    
    let compressed_ast = if ast.len() > 1024 { &ast[..1024] } else { ast }.to_string();
    let output_bits = RIC_BITS;
    
    let loss = landauer_loss(input_bits, output_bits, 300.0);
    let compression_ratio = output_bits / input_bits;
    
    RicCode {
        compressed_ast,
        compression_ratio,
        information_loss: loss,
        semantic_hash,
        metadata: {
            let mut m = std::collections::HashMap::new();
            m.insert("original_ast_len".to_string(), ast.len().to_string());
            m.insert("pixel_entropy".to_string(), pixel.entropy.to_string());
            m
        },
    }
}

#[cfg(feature = "ricemotions")]
pub fn teoa_map(ric: &RicCode) -> TeoaObject {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let input_bits = RIC_BITS;
    
    // Vetor semântico determinístico 512-dim
    let mut hasher = DefaultHasher::new();
    ric.semantic_hash.hash(&mut hasher);
    let seed = hasher.finish();
    
    let mut semantic_vector = Vec::with_capacity(512);
    let mut x = seed as f64;
    for _ in 0..512 {
        x = (x * 1664525.0 + 1013904223.0) % 4294967296.0;
        semantic_vector.push((x / 4294967296.0 * 2.0 - 1.0) as f32);
    }
    
    // Normaliza
    let norm: f32 = semantic_vector.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut semantic_vector {
            *v /= norm;
        }
    }
    
    let output_bits = TEOA_BITS;
    let _loss = landauer_loss(input_bits, output_bits, 300.0);
    
    let concept_id = format!("concept_{}", &ric.semantic_hash[..8]);
    let epistemic_confidence = (1.0 - ric.compression_ratio).max(0.5);
    
    TeoaObject {
        concept_id,
        semantic_vector,
        epistemic_confidence,
        abstract_relations: Vec::new(),
        metadata: {
            let mut m = std::collections::HashMap::new();
            m.insert("ric_hash".to_string(), ric.semantic_hash.clone());
            m.insert("ric_loss".to_string(), ric.information_loss.to_string());
            m
        },
    }
}

#[cfg(feature = "ricemotions")]
pub fn metabolic_to_emotional(landauer_residue: f64, delta_t_magnitude: f64) -> EmotionalState {
    if landauer_residue > 1e-18 || delta_t_magnitude > 10000.0 {
        EmotionalState::Anxiety
    } else if delta_t_magnitude > 100.0 {
        EmotionalState::Exploration
    } else if delta_t_magnitude > 10.0 {
        EmotionalState::Curiosity
    } else if landauer_residue < 1e-20 && delta_t_magnitude < 10.0 {
        EmotionalState::Satisfaction
    } else {
        EmotionalState::Homeostasis
    }
}

#[cfg(feature = "ricemotions")]
pub fn run_ricemotions_cycle(
    organism_state: &str,
    exogenous_data: &str,
    temperature: f64,
) -> RicemotionsPipeline {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Stage 1: PIXEL
    let pixel = pixel_encode(organism_state, exogenous_data);
    
    // Stage 2: RIC
    let ric = ric_compress(&pixel, organism_state);
    
    // Stage 3: TEOA
    let teoa = teoa_map(&ric);
    
    // Landauer accounting
    let loss_1 = landauer_loss(PIXEL_BITS, RIC_BITS, temperature);
    let loss_2 = landauer_loss(RIC_BITS, TEOA_BITS, temperature);
    let total_loss = loss_1 + loss_2;
    
    let landauer_monitor = LandauerMonitor {
        pixel_to_ric_loss: loss_1,
        ric_to_teoa_loss: loss_2,
        total_loss,
        threshold: 1e-18,
    };
    
    // Estado emocional
    let delta_t_mag: f64 = pixel.tensor.iter().map(|v| (*v as f64).powi(2)).sum();
    let emotional_state = metabolic_to_emotional(total_loss, delta_t_mag);
    
    let cycle_time_ms = start.elapsed().as_millis() as u64;
    
    RicemotionsPipeline {
        pixel_input: pixel,
        ric_intermediate: ric,
        teoa_output: teoa,
        landauer_monitor,
        metabolic_state: emotional_state,
        cycle_time_ms,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

#[cfg(feature = "ricemotions")]
pub fn visualize_emotion(state: EmotionalState) -> (u8, u8, u8, &'static str) {
    match state {
        EmotionalState::Homeostasis => (0, 255, 0, "breathing"),
        EmotionalState::Exploration => (0, 100, 255, "expanding"),
        EmotionalState::Anxiety => (255, 50, 50, "pulsing"),
        EmotionalState::Curiosity => (255, 200, 0, "sparkling"),
        EmotionalState::Satisfaction => (100, 255, 100, "glowing"),
    }
}

#[cfg(feature = "ricemotions")]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ricemotions_cycle() {
        let pipeline = run_ricemotions_cycle("test_organism", "test_exogenous", 300.0);
        
        assert!(pipeline.cycle_time_ms < 1000); // Should be very fast
        assert!(pipeline.landauer_monitor.total_loss >= 0.0);
        assert!(pipeline.pixel_input.tensor.len() == 64 * 64);
        assert!(pipeline.teoa_output.semantic_vector.len() == 512);
    }
    
    #[test]
    fn test_landauer_loss() {
        let loss = landauer_loss(1000.0, 500.0, 300.0);
        assert!(loss > 0.0);
        
        let loss_zero = landauer_loss(500.0, 1000.0, 300.0);
        assert_eq!(loss_zero, 0.0);
    }
    
    #[test]
    fn test_emotional_states() {
        assert_eq!(metabolic_to_emotional(1e-17, 50.0), EmotionalState::Anxiety);
        assert_eq!(metabolic_to_emotional(1e-19, 200.0), EmotionalState::Exploration);
        assert_eq!(metabolic_to_emotional(1e-19, 50.0), EmotionalState::Curiosity);
        assert_eq!(metabolic_to_emotional(1e-21, 5.0), EmotionalState::Satisfaction);
        assert_eq!(metabolic_to_emotional(1e-19, 50.0), EmotionalState::Homeostasis);
    }
    
    #[test]
    fn test_visualize_emotion() {
        let (r, g, b, anim) = visualize_emotion(EmotionalState::Anxiety);
        assert_eq!((r, g, b), (255, 50, 50));
        assert_eq!(anim, "pulsing");
    }
}
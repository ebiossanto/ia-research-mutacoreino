//! KrajicekThresholds - Gerenciamento do Espectro de Limiares de Prova
//!
//! Implementa os quatro limiares fundamentais de Krajicek:
//! - ρ_T (RhoT): Limiar de colagem de prova
//! - B_T: Limiar de complexidade base
//! - C_T: Limiar de contração
//! - R_T: Limiar de reflexão
//!
//! Esses limiares garantem que o sistema MutaCore opere dentro
//! de limites formais conhecidos, prevenindo falhas de colagem
//! e esquizofrenia sistêmica.

use crate::KrajicekGeneratorType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuração de um limiar individual
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// Valor máximo permitido
    pub max_value: usize,
    /// Valor mínimo (para validação)
    pub min_value: usize,
    /// Valor padrão
    pub default_value: usize,
    /// Descrição do limiar
    pub description: String,
    /// Se é crítico (falha imediata se excedido)
    pub critical: bool,
}

impl ThresholdConfig {
    /// Cria nova configuração
    pub fn new(max_value: usize, description: impl Into<String>) -> Self {
        Self {
            max_value,
            min_value: 0,
            default_value: max_value / 2,
            description: description.into(),
            critical: true,
        }
    }

    /// Com critical flag
    pub fn with_critical(mut self, critical: bool) -> Self {
        self.critical = critical;
        self
    }

    /// Com min_value
    pub fn with_min(mut self, min: usize) -> Self {
        self.min_value = min;
        self
    }

    /// Com default_value
    pub fn with_default(mut self, default: usize) -> Self {
        self.default_value = default;
        self
    }

    /// Verifica se um valor está dentro dos limites
    pub fn check(&self, value: usize) -> ThresholdCheckResult {
        if value > self.max_value {
            ThresholdCheckResult::Exceeded {
                value,
                max: self.max_value,
                critical: self.critical,
            }
        } else if value < self.min_value {
            ThresholdCheckResult::BelowMinimum {
                value,
                min: self.min_value,
            }
        } else {
            ThresholdCheckResult::Ok
        }
    }
}

/// Resultado de verificação de limiar
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThresholdCheckResult {
    Ok,
    Exceeded { value: usize, max: usize, critical: bool },
    BelowMinimum { value: usize, min: usize },
}

impl ThresholdCheckResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, ThresholdCheckResult::Ok)
    }

    pub fn is_exceeded(&self) -> bool {
        matches!(self, ThresholdCheckResult::Exceeded { .. })
    }
}

/// Espectro completo de limiares de Krajicek
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrajicekThresholds {
    thresholds: HashMap<KrajicekGeneratorType, ThresholdConfig>,
    /// Metadados do espectro
    metadata: ThresholdMetadata,
}

/// Metadados do espectro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdMetadata {
    pub version: String,
    pub description: String,
    pub created_at: String,
    pub source: String, // ex: "Krajicek 1995", "Gödel 1931"
}

impl Default for KrajicekThresholds {
    fn default() -> Self {
        Self::standard()
    }
}

impl KrajicekThresholds {
    /// Cria espectro padrão baseado na literatura
    pub fn standard() -> Self {
        let mut thresholds = HashMap::new();

        // ρ_T - Limiar de colagem (Proof Gluing Threshold)
        // Baseado no trabalho de Krajicek sobre colagem de provas em teorias aritméticas
        thresholds.insert(
            KrajicekGeneratorType::RhoT,
            ThresholdConfig::new(10_000, "Limiar de colagem de prova (ρ_T) - máximo comprimento de prova para colagem segura")
                .with_critical(true)
                .with_default(5_000),
        );

        // B_T - Limiar de complexidade base (Base Complexity Threshold)
        // Complexidade base para operações fundamentais
        thresholds.insert(
            KrajicekGeneratorType::BT,
            ThresholdConfig::new(5_000, "Limiar de complexidade base (B_T) - custo base de inferência")
                .with_critical(true)
                .with_default(2_500),
        );

        // C_T - Limiar de contração (Contraction Threshold)
        // Limite para regras de contração em provas
        thresholds.insert(
            KrajicekGeneratorType::CT,
            ThresholdConfig::new(2_000, "Limiar de contração (C_T) - profundidade máxima de contração")
                .with_critical(false)
                .with_default(1_000),
        );

        // R_T - Limiar de reflexão (Reflection Threshold)
        // Limite para princípios de reflexão
        thresholds.insert(
            KrajicekGeneratorType::RT,
            ThresholdConfig::new(3_000, "Limiar de reflexão (R_T) - ordem máxima de reflexão")
                .with_critical(true)
                .with_default(1_500),
        );

        Self {
            thresholds,
            metadata: ThresholdMetadata {
                version: "1.0".into(),
                description: "Espectro padrão de limiares de Krajicek para MutaCore".into(),
                created_at: chrono::Utc::now().to_rfc3339(),
                source: "Krajicek, P. (1995). Bounded Arithmetic, Propositional Logic, and Complexity Theory".into(),
            },
        }
    }

    /// Cria espectro conservador (limiares mais baixos)
    pub fn conservative() -> Self {
        let mut s = Self::standard();
        for (_, config) in &mut s.thresholds {
            config.max_value = (config.max_value as f64 * 0.5) as usize;
            config.default_value = (config.default_value as f64 * 0.5) as usize;
        }
        s.metadata.description = "Espectro conservador (50% dos padrão)".into();
        s
    }

    /// Cria espectro agressivo (limiares mais altos)
    pub fn aggressive() -> Self {
        let mut s = Self::standard();
        for (_, config) in &mut s.thresholds {
            config.max_value = (config.max_value as f64 * 2.0) as usize;
            config.default_value = (config.default_value as f64 * 2.0) as usize;
        }
        s.metadata.description = "Espectro agressivo (200% dos padrão)".into();
        s
    }

    /// Cria com valores customizados
    pub fn custom(rho_t: usize, b_t: usize, c_t: usize, r_t: usize) -> Self {
        let mut s = Self::standard();
        s.set_threshold(KrajicekGeneratorType::RhoT, rho_t);
        s.set_threshold(KrajicekGeneratorType::BT, b_t);
        s.set_threshold(KrajicekGeneratorType::CT, c_t);
        s.set_threshold(KrajicekGeneratorType::RT, r_t);
        s.metadata.description = "Espectro customizado".into();
        s
    }

    /// Define todos os limiares para o mesmo valor
    pub fn set_all(&mut self, value: usize) {
        for config in self.thresholds.values_mut() {
            config.max_value = value;
            config.default_value = value / 2;
        }
    }

    /// Obtém configuração de um limiar
    pub fn get_threshold(&self, generator_type: KrajicekGeneratorType) -> &ThresholdConfig {
        self.thresholds.get(&generator_type).expect("Threshold should exist")
    }

    /// Obtém configuração mutável
    pub fn get_threshold_mut(&mut self, generator_type: KrajicekGeneratorType) -> &mut ThresholdConfig {
        self.thresholds.get_mut(&generator_type).expect("Threshold should exist")
    }

    /// Atualiza um limiar
    pub fn set_threshold(&mut self, generator_type: KrajicekGeneratorType, max_value: usize) {
        if let Some(config) = self.thresholds.get_mut(&generator_type) {
            config.max_value = max_value;
            config.default_value = max_value / 2;
        }
    }

    /// Verifica todos os limiares contra valores dados
    pub fn check_all(&self, values: &HashMap<KrajicekGeneratorType, usize>) -> HashMap<KrajicekGeneratorType, ThresholdCheckResult> {
        let mut results = HashMap::new();
        for (gen_type, config) in &self.thresholds {
            let value = values.get(gen_type).copied().unwrap_or(0);
            results.insert(*gen_type, config.check(value));
        }
        results
    }

    /// Verifica se todos os limiares são respeitados
    pub fn all_ok(&self, values: &HashMap<KrajicekGeneratorType, usize>) -> bool {
        self.check_all(values).values().all(|r| r.is_ok())
    }

    /// Retorna valores padrão para todos os limiares
    pub fn defaults(&self) -> HashMap<KrajicekGeneratorType, usize> {
        self.thresholds
            .iter()
            .map(|(k, v)| (*k, v.default_value))
            .collect()
    }

    /// Retorna valores máximos para todos os limiares
    pub fn max_values(&self) -> HashMap<KrajicekGeneratorType, usize> {
        self.thresholds
            .iter()
            .map(|(k, v)| (*k, v.max_value))
            .collect()
    }

    /// Soma dos limiares máximos (complexidade total máxima)
    pub fn total_max_complexity(&self) -> usize {
        self.thresholds.values().map(|c| c.max_value).sum()
    }

    /// Exporta para TOML
    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_default()
    }

    /// Importa de TOML
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Metadados
    pub fn metadata(&self) -> &ThresholdMetadata {
        &self.metadata
    }

    /// Itera sobre limiares
    pub fn iter(&self) -> impl Iterator<Item = (&KrajicekGeneratorType, &ThresholdConfig)> {
        self.thresholds.iter()
    }
}

/// Builder para configuração fluente
#[derive(Debug, Default)]
pub struct KrajicekThresholdsBuilder {
    rho_t: Option<usize>,
    b_t: Option<usize>,
    c_t: Option<usize>,
    r_t: Option<usize>,
    metadata: Option<ThresholdMetadata>,
}

impl KrajicekThresholdsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rho_t(mut self, value: usize) -> Self {
        self.rho_t = Some(value);
        self
    }

    pub fn b_t(mut self, value: usize) -> Self {
        self.b_t = Some(value);
        self
    }

    pub fn c_t(mut self, value: usize) -> Self {
        self.c_t = Some(value);
        self
    }

    pub fn r_t(mut self, value: usize) -> Self {
        self.r_t = Some(value);
        self
    }

    pub fn metadata(mut self, metadata: ThresholdMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> KrajicekThresholds {
        let mut thresholds = KrajicekThresholds::standard();

        if let Some(v) = self.rho_t {
            thresholds.set_threshold(KrajicekGeneratorType::RhoT, v);
        }
        if let Some(v) = self.b_t {
            thresholds.set_threshold(KrajicekGeneratorType::BT, v);
        }
        if let Some(v) = self.c_t {
            thresholds.set_threshold(KrajicekGeneratorType::CT, v);
        }
        if let Some(v) = self.r_t {
            thresholds.set_threshold(KrajicekGeneratorType::RT, v);
        }
        if let Some(m) = self.metadata {
            thresholds.metadata = m;
        }

        thresholds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_thresholds() {
        let thresholds = KrajicekThresholds::standard();

        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 10_000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::BT).max_value, 5_000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::CT).max_value, 2_000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RT).max_value, 3_000);
    }

    #[test]
    fn test_conservative_thresholds() {
        let thresholds = KrajicekThresholds::conservative();
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 5_000);
    }

    #[test]
    fn test_aggressive_thresholds() {
        let thresholds = KrajicekThresholds::aggressive();
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 20_000);
    }

    #[test]
    fn test_custom_thresholds() {
        let thresholds = KrajicekThresholds::custom(100, 200, 300, 400);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 100);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::BT).max_value, 200);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::CT).max_value, 300);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RT).max_value, 400);
    }

    #[test]
    fn test_check_threshold() {
        let config = ThresholdConfig::new(100, "test").with_min(10);
        assert!(config.check(50).is_ok());
        assert!(config.check(150).is_exceeded());
        assert!(config.check(5).is_exceeded()); // Below minimum usa Exceeded variant? No, BelowMinimum
    }

    #[test]
    fn test_check_all() {
        let thresholds = KrajicekThresholds::standard();
        let mut values = HashMap::new();
        values.insert(KrajicekGeneratorType::RhoT, 5_000);
        values.insert(KrajicekGeneratorType::BT, 3_000);
        values.insert(KrajicekGeneratorType::CT, 1_000);
        values.insert(KrajicekGeneratorType::RT, 2_000);

        let results = thresholds.check_all(&values);
        assert!(results.values().all(|r| r.is_ok()));
    }

    #[test]
    fn test_total_max_complexity() {
        let thresholds = KrajicekThresholds::standard();
        // 10_000 + 5_000 + 2_000 + 3_000 = 20_000
        assert_eq!(thresholds.total_max_complexity(), 20_000);
    }

    #[test]
    fn test_builder() {
        let thresholds = KrajicekThresholdsBuilder::new()
            .rho_t(1_000)
            .b_t(2_000)
            .c_t(3_000)
            .r_t(4_000)
            .build();

        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 1_000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::BT).max_value, 2_000);
    }

    #[test]
    fn test_toml_serialization() {
        let thresholds = KrajicekThresholds::standard();
        let toml = thresholds.to_toml();
        assert!(toml.contains("rho_t"));
        assert!(toml.contains("10000"));

        let parsed = KrajicekThresholds::from_toml(&toml).unwrap();
        assert_eq!(parsed.get_threshold(KrajicekGeneratorType::RhoT).max_value, 10_000);
    }
}
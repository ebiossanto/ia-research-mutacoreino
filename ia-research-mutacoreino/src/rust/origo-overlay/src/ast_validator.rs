//! OrigoAstValidator - Validador de ASTs com Limiares de Krajicek
//!
//! Implementa a validação de complexidade de prova baseada no espectro
//! de limiares de Krajicek (ρ_T, B_T, C_T, R_T) para prevenir falhas
//! de colagem lógica e garantir terminabilidade.

use crate::{AstNode, AstNodeKind, KrajicekGeneratorType, error::{OrigoError, Result, ValidationResult}, krajicek_thresholds::{KrajicekThresholds, ThresholdConfig}};

/// Validador principal da AST Origo
///
/// Verifica se uma AST mutante respeita os limiares de complexidade
/// de prova estabelecidos pelos geradores de Krajicek. Se uma mutação
/// exceder o limiar de colagem (ρ_T), ela é rejeitada para evitar
/// esquizofrenia sistêmica.
#[derive(Debug, Clone)]
pub struct OrigoAstValidator {
    /// Limiares de Krajicek configurados
    thresholds: KrajicekThresholds,
    /// Configuração de validação
    config: ValidationConfig,
    /// Estatísticas de validação
    stats: ValidationStats,
}

/// Configuração de validação
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Modo estrito: falha em qualquer violação
    pub strict_mode: bool,
    /// Permite warnings para limiares próximos (90%)
    pub warn_near_threshold: bool,
    /// Percentual para warning (0.0-1.0)
    pub warn_threshold_ratio: f64,
    /// Profundidade máxima de recursão
    pub max_recursion_depth: usize,
    /// Limite de nós para validação
    pub max_nodes: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            warn_near_threshold: true,
            warn_threshold_ratio: 0.9,
            max_recursion_depth: 10_000,
            max_nodes: 1_000_000,
        }
    }
}

/// Estatísticas de validação
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    pub total_validated: u64,
    pub passed: u64,
    pub failed: u64,
    pub warnings: u64,
    pub total_complexity: usize,
    pub max_complexity_seen: usize,
}

impl OrigoAstValidator {
    /// Cria um novo validador com limiares padrão
    pub fn new(thresholds: KrajicekThresholds) -> Self {
        Self {
            thresholds,
            config: ValidationConfig::default(),
            stats: ValidationStats::default(),
        }
    }

    /// Cria um validador com configuração customizada
    pub fn with_config(thresholds: KrajicekThresholds, config: ValidationConfig) -> Self {
        Self {
            thresholds,
            config,
            stats: ValidationStats::default(),
        }
    }

    /// Cria validador com limiar máximo simples (compatibilidade)
    pub fn new_simple(max_threshold: usize) -> Self {
        let mut thresholds = KrajicekThresholds::default();
        thresholds.set_all(max_threshold);
        Self::new(thresholds)
    }

    /// Valida um nó AST completo
    pub fn validate(&mut self, node: &AstNode) -> Result<ValidationResult> {
        self.stats.total_validated += 1;

        // Verifica limites estruturais
        let node_count = node.node_count();
        if node_count > self.config.max_nodes {
            return Err(OrigoError::CapacityExceeded {
                resource: "AST nodes".into(),
                current: node_count,
                max: self.config.max_nodes,
            });
        }

        if node.depth() > self.config.max_recursion_depth {
            return Err(OrigoError::CapacityExceeded {
                resource: "AST depth".into(),
                current: node.depth(),
                max: self.config.max_recursion_depth,
            });
        }

        // Validação recursiva
        let mut result = ValidationResult::success(0);
        self.validate_node_recursive(node, 0, &mut result)?;

        self.stats.total_complexity += result.total_complexity;
        self.stats.max_complexity_seen = self.stats.max_complexity_seen.max(result.total_complexity);

        if result.valid {
            self.stats.passed += 1;
        } else {
            self.stats.failed += 1;
        }

        Ok(result)
    }

    /// Validação recursiva interna
    fn validate_node_recursive(
        &self,
        node: &AstNode,
        depth: usize,
        result: &mut ValidationResult,
    ) -> Result<()> {
        if depth > self.config.max_recursion_depth {
            return Err(OrigoError::CapacityExceeded {
                resource: "recursion depth".into(),
                current: depth,
                max: self.config.max_recursion_depth,
            });
        }

        match node {
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => {
                let threshold_config = self.thresholds.get_threshold(*generator_type);
                let effective_threshold = threshold.min(threshold_config.max_value);

                // Verifica se excede o limiar configurado
                if *proof_len > effective_threshold {
                    let failure = format!(
                        "Gerador {:?}: prova_len={} > threshold={} (config={})",
                        generator_type, proof_len, threshold, threshold_config.max_value
                    );
                    result.add_failure(failure.clone());

                    if self.config.strict_mode {
                        return Err(OrigoError::proof_threshold_exceeded(
                            *proof_len,
                            effective_threshold,
                            *generator_type,
                        ));
                    }
                }

                // Warning para limiares próximos
                if self.config.warn_near_threshold {
                    let ratio = *proof_len as f64 / effective_threshold as f64;
                    if ratio >= self.config.warn_threshold_ratio {
                        result.add_failure(format!(
                            "AVISO: Gerador {:?} próximo do limiar ({:.1}%)",
                            generator_type,
                            ratio * 100.0
                        ));
                        self.stats.warnings += 1;
                    }
                }

                // Marca como verificado
                match generator_type {
                    KrajicekGeneratorType::RhoT => result.rho_t_checked = true,
                    KrajicekGeneratorType::BT => result.b_t_checked = true,
                    KrajicekGeneratorType::CT => result.c_t_checked = true,
                    KrajicekGeneratorType::RT => result.r_t_checked = true,
                }

                result.total_complexity += *proof_len;
            }
            AstNode::Abstraction(_, body) => {
                self.validate_node_recursive(body, depth + 1, result)?;
            }
            AstNode::Application(fun, arg) => {
                self.validate_node_recursive(fun, depth + 1, result)?;
                self.validate_node_recursive(arg, depth + 1, result)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Valida apenas os geradores de Krajicek (verificação rápida)
    pub fn validate_krajicek_only(&mut self, node: &AstNode) -> Result<ValidationResult> {
        self.stats.total_validated += 1;
        let mut result = ValidationResult::success(0);
        self.collect_krajicek_generators(node, &mut result)?;

        if result.valid {
            self.stats.passed += 1;
        } else {
            self.stats.failed += 1;
        }

        Ok(result)
    }

    /// Coleta todos os geradores de Krajicek na AST
    fn collect_krajicek_generators(
        &self,
        node: &AstNode,
        result: &mut ValidationResult,
    ) -> Result<()> {
        match node {
            AstNode::KrajicekGenerator {
                proof_len,
                threshold,
                generator_type,
            } => {
                let threshold_config = self.thresholds.get_threshold(*generator_type);
                let effective_threshold = threshold.min(threshold_config.max_value);

                if *proof_len > effective_threshold {
                    result.add_failure(format!(
                        "{:?}: {} > {}",
                        generator_type, proof_len, effective_threshold
                    ));
                }

                match generator_type {
                    KrajicekGeneratorType::RhoT => result.rho_t_checked = true,
                    KrajicekGeneratorType::BT => result.b_t_checked = true,
                    KrajicekGeneratorType::CT => result.c_t_checked = true,
                    KrajicekGeneratorType::RT => result.r_t_checked = true,
                }

                result.total_complexity += *proof_len;
            }
            AstNode::Abstraction(_, body) => self.collect_krajicek_generators(body, result)?,
            AstNode::Application(fun, arg) => {
                self.collect_krajicek_generators(fun, result)?;
                self.collect_krajicek_generators(arg, result)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Verifica colagem lógica (gluing) entre duas ASTs
    ///
    /// Testa se a composição de duas ASTs mutantes mantém a
    /// consistência lógica sem exceder ρ_T (limiar de colagem).
    pub fn validate_gluing(&mut self, ast1: &AstNode, ast2: &AstNode) -> Result<ValidationResult> {
        let mut result = ValidationResult::success(0);

        // Complexidade combinada
        let complexity1 = self.compute_krajicek_complexity(ast1);
        let complexity2 = self.compute_krajicek_complexity(ast2);
        let combined = complexity1 + complexity2;

        let rho_t_config = self.thresholds.get_threshold(KrajicekGeneratorType::RhoT);

        if combined > rho_t_config.max_value {
            result.add_failure(format!(
                "Falha de colagem: complexidade combinada {} > ρ_T={}",
                combined, rho_t_config.max_value
            ));
            return Err(OrigoError::gluing_failure(
                "Complexidade combinada excede limiar de colagem ρ_T",
                rho_t_config.max_value,
                self.thresholds.get_threshold(KrajicekGeneratorType::BT).max_value,
                self.thresholds.get_threshold(KrajicekGeneratorType::CT).max_value,
                self.thresholds.get_threshold(KrajicekGeneratorType::RT).max_value,
            ));
        }

        result.total_complexity = combined;
        Ok(result)
    }

    /// Computa complexidade total de Krajicek em uma AST
    pub fn compute_krajicek_complexity(&self, node: &AstNode) -> usize {
        let mut total = 0;
        self.accumulate_complexity(node, &mut total);
        total
    }

    fn accumulate_complexity(&self, node: &AstNode, total: &mut usize) {
        match node {
            AstNode::KrajicekGenerator { proof_len, .. } => *total += *proof_len,
            AstNode::Abstraction(_, body) => self.accumulate_complexity(body, total),
            AstNode::Application(fun, arg) => {
                self.accumulate_complexity(fun, total);
                self.accumulate_complexity(arg, total);
            }
            _ => {}
        }
    }

    /// Obtém estatísticas de validação
    pub fn stats(&self) -> &ValidationStats {
        &self.stats
    }

    /// Reseta estatísticas
    pub fn reset_stats(&mut self) {
        self.stats = ValidationStats::default();
    }

    /// Obtém configuração atual
    pub fn config(&self) -> &ValidationConfig {
        &self.config
    }

    /// Atualiza configuração
    pub fn set_config(&mut self, config: ValidationConfig) {
        self.config = config;
    }

    /// Atualiza limiares
    pub fn set_thresholds(&mut self, thresholds: KrajicekThresholds) {
        self.thresholds = thresholds;
    }

    /// Obtém limiares atuais
    pub fn thresholds(&self) -> &KrajicekThresholds {
        &self.thresholds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AstNode, KrajicekGeneratorType};

    #[test]
    fn test_validator_creation() {
        let validator = OrigoAstValidator::new_simple(1000);
        assert_eq!(validator.thresholds().get_threshold(KrajicekGeneratorType::RhoT).max_value, 1000);
    }

    #[test]
    fn test_validate_simple_ast() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast = AstNode::abstraction("x", AstNode::variable("x"));
        let result = validator.validate(&ast).unwrap();
        assert!(result.valid);
    }

    #[test]
    fn test_validate_krajicek_within_threshold() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast = AstNode::krajicek_generator(500, 1000, KrajicekGeneratorType::RhoT);
        let result = validator.validate(&ast).unwrap();
        assert!(result.valid);
        assert!(result.rho_t_checked);
    }

    #[test]
    fn test_validate_krajicek_exceeds_threshold() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast = AstNode::krajicek_generator(1500, 1000, KrajicekGeneratorType::RhoT);
        let result = validator.validate(&ast);
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_krajicek_generators() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast = AstNode::application(
            AstNode::krajicek_generator(300, 1000, KrajicekGeneratorType::RhoT),
            AstNode::krajicek_generator(400, 1000, KrajicekGeneratorType::BT),
        );
        let result = validator.validate(&ast).unwrap();
        assert!(result.valid);
        assert_eq!(result.total_complexity, 700);
    }

    #[test]
    fn test_gluing_validation_pass() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast1 = AstNode::krajicek_generator(300, 1000, KrajicekGeneratorType::RhoT);
        let ast2 = AstNode::krajicek_generator(400, 1000, KrajicekGeneratorType::RhoT);
        let result = validator.validate_gluing(&ast1, &ast2).unwrap();
        assert!(result.valid);
        assert_eq!(result.total_complexity, 700);
    }

    #[test]
    fn test_gluing_validation_fail() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast1 = AstNode::krajicek_generator(600, 1000, KrajicekGeneratorType::RhoT);
        let ast2 = AstNode::krajicek_generator(600, 1000, KrajicekGeneratorType::RhoT);
        let result = validator.validate_gluing(&ast1, &ast2);
        assert!(result.is_err());
    }

    #[test]
    fn test_warning_near_threshold() {
        let config = ValidationConfig {
            warn_near_threshold: true,
            warn_threshold_ratio: 0.9,
            ..Default::default()
        };
        let mut validator = OrigoAstValidator::with_config(
            KrajicekThresholds::default().with_all(1000),
            config,
        );
        let ast = AstNode::krajicek_generator(950, 1000, KrajicekGeneratorType::RhoT); // 95%
        let result = validator.validate(&ast).unwrap();
        assert!(result.valid);
        assert!(validator.stats().warnings > 0);
    }

    #[test]
    fn test_compute_complexity() {
        let validator = OrigoAstValidator::new_simple(1000);
        let ast = AstNode::application(
            AstNode::krajicek_generator(100, 1000, KrajicekGeneratorType::RhoT),
            AstNode::krajicek_generator(200, 1000, KrajicekGeneratorType::BT),
        );
        assert_eq!(validator.compute_krajicek_complexity(&ast), 300);
    }

    #[test]
    fn test_stats_tracking() {
        let mut validator = OrigoAstValidator::new_simple(1000);
        let ast_ok = AstNode::krajicek_generator(100, 1000, KrajicekGeneratorType::RhoT);
        let ast_fail = AstNode::krajicek_generator(1500, 1000, KrajicekGeneratorType::RhoT);

        validator.validate(&ast_ok).unwrap();
        let _ = validator.validate(&ast_fail); // Ignora erro

        let stats = validator.stats();
        assert_eq!(stats.total_validated, 2);
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 1);
    }
}
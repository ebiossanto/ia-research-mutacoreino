//! Tipos de erro para o Origo Overlay

use std::fmt;
use thiserror::Error;

/// Resultado padronizado do Origo
pub type Result<T> = std::result::Result<T, OrigoError>;

/// Erros principais do sistema Origo Overlay
#[derive(Debug, Error)]
pub enum OrigoError {
    /// Limiar de prova de Krajicek excedido
    #[error("Limiar de prova excedido: prova_len={actual} > max={max} (tipo={generator_type:?})")]
    ProofThresholdExceeded {
        actual: usize,
        max: usize,
        generator_type: crate::KrajicekGeneratorType,
    },

    /// Falha no isomorfismo alfa-estrutural
    #[error("Falha no isomorfismo alfa-estrutural: {details}")]
    AlphaIsomorphismFailure { details: String },

    /// Falha de colagem (gluing failure) - incompletude
    #[error("Falha de colagem lógica: {reason} (ρ_T={rho_t}, B_T={b_t}, C_T={c_t}, R_T={r_t})")]
    GluingFailure {
        reason: String,
        rho_t: usize,
        b_t: usize,
        c_t: usize,
        r_t: usize,
    },

    /// Erro de serialização/desserialização
    #[error("Erro de serialização: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Erro de I/O
    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),

    /// Erro de criptografia
    #[error("Erro criptográfico: {0}")]
    CryptoError(String),

    /// Erro de configuração inválida
    #[error("Configuração inválida: {0}")]
    InvalidConfig(String),

    /// Erro de packing quântico
    #[error("Erro no Quantum Local Packing: {0}")]
    QuantumPackingError(String),

    /// Nó não encontrado na rede
    #[error("Nó não encontrado: {node_id}")]
    NodeNotFound { node_id: String },

    /// Capacidade excedida
    #[error("Capacidade excedida: {resource} (atual={current}, max={max})")]
    CapacityExceeded {
        resource: String,
        current: usize,
        max: usize,
    },

    /// Timeout de operação
    #[error("Timeout: {operation} após {timeout_ms}ms")]
    Timeout { operation: String, timeout_ms: u64 },

    /// Erro interno inesperado
    #[error("Erro interno: {0}")]
    Internal(String),
}

impl OrigoError {
    /// Cria erro de limiar excedido
    pub fn proof_threshold_exceeded(
        actual: usize,
        max: usize,
        generator_type: crate::KrajicekGeneratorType,
    ) -> Self {
        Self::ProofThresholdExceeded {
            actual,
            max,
            generator_type,
        }
    }

    /// Cria erro de falha de isomorfismo alfa
    pub fn alpha_isomorphism_failure(details: impl Into<String>) -> Self {
        Self::AlphaIsomorphismFailure {
            details: details.into(),
        }
    }

    /// Cria erro de falha de colagem
    pub fn gluing_failure(
        reason: impl Into<String>,
        rho_t: usize,
        b_t: usize,
        c_t: usize,
        r_t: usize,
    ) -> Self {
        Self::GluingFailure {
            reason: reason.into(),
            rho_t,
            b_t,
            c_t,
            r_t,
        }
    }

    /// Verifica se é um erro recuperável (pode tentar novamente)
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            OrigoError::Timeout { .. } | OrigoError::CapacityExceeded { .. }
        )
    }

    /// Verifica se é erro de validação (dados inválidos)
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            OrigoError::ProofThresholdExceeded { .. }
                | OrigoError::AlphaIsomorphismFailure { .. }
                | OrigoError::GluingFailure { .. }
                | OrigoError::InvalidConfig(_)
        )
    }
}

/// Resultado de validação com detalhes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    /// Se a validação passou
    pub valid: bool,
    /// Limiar ρ_T verificado
    pub rho_t_checked: bool,
    /// Limiar B_T verificado
    pub b_t_checked: bool,
    /// Limiar C_T verificado
    pub c_t_checked: bool,
    /// Limiar R_T verificado
    pub r_t_checked: bool,
    /// Detalhes de falhas (se houver)
    pub failures: Vec<String>,
    /// Complexidade total computada
    pub total_complexity: usize,
}

impl ValidationResult {
    /// Resultado de sucesso
    pub fn success(total_complexity: usize) -> Self {
        Self {
            valid: true,
            rho_t_checked: true,
            b_t_checked: true,
            c_t_checked: true,
            r_t_checked: true,
            failures: Vec::new(),
            total_complexity,
        }
    }

    /// Resultado de falha
    pub fn failure(failures: Vec<String>, total_complexity: usize) -> Self {
        Self {
            valid: false,
            rho_t_checked: false,
            b_t_checked: false,
            c_t_checked: false,
            r_t_checked: false,
            failures,
            total_complexity,
        }
    }

    /// Adiciona uma falha
    pub fn add_failure(&mut self, failure: String) {
        self.valid = false;
        self.failures.push(failure);
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.valid {
            write!(f, "✓ Validação OK (complexidade={})", self.total_complexity)
        } else {
            write!(
                f,
                "✗ Validação FALHOU (complexidade={}): {}",
                self.total_complexity,
                self.failures.join("; ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = OrigoError::proof_threshold_exceeded(1500, 1000, crate::KrajicekGeneratorType::RhoT);
        assert!(err.to_string().contains("1500"));
        assert!(err.to_string().contains("1000"));
    }

    #[test]
    fn test_validation_result() {
        let success = ValidationResult::success(100);
        assert!(success.valid);
        assert_eq!(success.total_complexity, 100);

        let failure = ValidationResult::failure(vec!["Erro 1".into(), "Erro 2".into()], 200);
        assert!(!failure.valid);
        assert_eq!(failure.failures.len(), 2);
    }

    #[test]
    fn test_error_classification() {
        let validation_err = OrigoError::proof_threshold_exceeded(100, 50, crate::KrajicekGeneratorType::RhoT);
        assert!(validation_err.is_validation_error());
        assert!(!validation_err.is_retryable());

        let timeout_err = OrigoError::Timeout {
            operation: "test".into(),
            timeout_ms: 1000,
        };
        assert!(timeout_err.is_retryable());
        assert!(!timeout_err.is_validation_error());
    }
}
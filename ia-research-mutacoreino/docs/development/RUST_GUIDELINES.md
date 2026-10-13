# Diretrizes Rust — MutaCore

> **Objetivo:** Código seguro, performático, formalmente verificável

---

## Princípios Fundamentais

1. **Safety First** — Zero `unsafe` no core. `unsafe` só em FFI/boundary com justificativa documentada.
2. **Formal Verification Ready** — Estruturas que o validador pode analisar estaticamente.
3. **Zero-Cost Abstractions** — Generics, traits, iterators sobre trait objects onde possível.
4. **Explicit Error Handling** — `Result<T, OrigoError>` em toda API pública.
5. **Determinism** — Mesma entrada → mesma saída (crítico para hash alfa-estrutural).

---

## Estilo de Código

### Formatação

```bash
# rustfmt padrão (rustfmt.toml no workspace)
cargo fmt --all
```

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
hard_tabs = false
brace_style = "SameLineWhere"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
```

### Clippy

```bash
# Deny warnings em CI
cargo clippy --workspace -- -D warnings -D clippy::all -D clippy::pedantic -A clippy::module_name_repetitions
```

---

## Padrões de API

### Construtores

```rust
// Sempre new() para construtor padrão
impl OrigoAstValidator {
    pub fn new(thresholds: KrajicekThresholds) -> Self { ... }
    pub fn with_config(thresholds: KrajicekThresholds, config: ValidationConfig) -> Self { ... }
}

// Builder para configuração complexa
impl KrajicekThresholds {
    pub fn builder() -> KrajicekThresholdsBuilder { ... }
}
```

### Error Types

```rust
// Use thiserror para derive
#[derive(Debug, Error)]
pub enum OrigoError {
    #[error("Proof threshold exceeded: proof_len={actual} > max={max}")]
    ProofThresholdExceeded { actual: usize, max: usize },
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// SEMPRE impl From para erros externos que você expõe
```

### Generics e Traits

```rust
// Prefira trait bounds a trait objects
pub fn validate<T: HashStructural>(&self, node: &T) -> Result<ValidationResult>

// Evite Box<dyn Trait> em hot paths
// Use enum se conjunto fechado de tipos
```

---

## Validação e Segurança

### Regra de Ouro: Valide na Fronteira

```rust
// ✓ CORRETO: Valida entrada ANTES de processar
pub fn process_mutation(&mut self, ast: AstNode) -> Result<()> {
    self.validator.validate(&ast)?;  // Gate obrigatório
    self.execute_safe(ast)
}

// ✗ ERRADO: Processa sem validar
pub fn process_mutation(&mut self, ast: AstNode) -> Result<()> {
    self.execute_safe(ast)  // PERIGOSO!
}
```

### Limiares de Krajicek

```rust
// SEMPRE use thresholds configurados, nunca hardcode
let validator = OrigoAstValidator::new(thresholds);  // ✓
let validator = OrigoAstValidator::new_simple(10000); // ✗ Apenas para testes
```

### Alpha-Structural Hash

```rust
// Para comparar ASTs semanticamente:
let hash1 = StructuralHasher::hash(&ast1);
let hash2 = StructuralHasher::hash(&ast2);
if hash1 == hash2 { /* semanticamente iguais */ }

// NÃO use PartialEq direto em AstNode (sensível a nomes de variáveis)
```

---

## Performance

### Allocation Strategy

```rust
// Reutilize buffers
let mut buffer = Vec::with_capacity(expected_size);

// Evite clonagens desnecessárias
fn process(&self, node: &AstNode) -> Result<AstNode> {  // &AstNode, não AstNode
    // ...
}

// Cow para copy-on-write
use std::borrow::Cow;
fn maybe_modify(&self, node: &AstNode) -> Cow<AstNode> { ... }
```

### Hot Paths

```rust
// Marque funções críticas
#[inline(always)]
fn hash_node(node: &AstNode) -> u64 { ... }

// Use iteradores em vez de loops manuais
let sum: usize = nodes.iter().map(|n| n.complexity()).sum();
```

---

## Testes

### Unit Tests (no mesmo arquivo)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_rejects_over_threshold() {
        let mut validator = OrigoAstValidator::new_simple(100);
        let ast = AstNode::krajicek_generator(150, 100, RhoT);
        assert!(validator.validate(&ast).is_err());
    }
}
```

### Property-Based Tests (proptest)

```rust
// tests/proptest_ast.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn alpha_canonicalization_idempotent(ast in arb_ast()) {
        let mut ctx = AlphaContext::new();
        let canon1 = ctx.canonicalize(&ast);
        let canon2 = ctx.canonicalize(&canon1);
        prop_assert_eq!(canon1, canon2);
    }
}
```

### Integration Tests

```rust
// tests/integration_validator.rs
use origo_overlay::{OrigoAstValidator, AstNode, KrajicekThresholds};

#[test]
fn full_validation_pipeline() {
    let thresholds = KrajicekThresholds::standard();
    let mut validator = OrigoAstValidator::new(thresholds);
    
    // AST complexa mas válida
    let ast = build_complex_valid_ast();
    let result = validator.validate(&ast).unwrap();
    assert!(result.valid);
}
```

---

## Documentação

### Doc Comments

```rust
/// Valida uma AST contra os limiares de Krajicek configurados.
///
/// # Arguments
/// * `node` - AST a ser validada (já canonizada preferivelmente)
///
/// # Returns
/// `ValidationResult` com detalhes de quais limiares passaram/falharam.
///
/// # Errors
/// Retorna `OrigoError::ProofThresholdExceeded` se modo estrito e limiar excedido.
///
/// # Example
/// ```
/// let mut validator = OrigoAstValidator::new_simple(1000);
/// let ast = AstNode::abstraction("x", AstNode::variable("x"));
/// let result = validator.validate(&ast).unwrap();
/// assert!(result.valid);
/// ```
pub fn validate(&mut self, node: &AstNode) -> Result<ValidationResult> { ... }
```

---

## Dependências

### Permitidas (workspace deps)

```toml
# Cargo.toml workspace
thiserror = "1.0"      # Error derive
serde = { version = "1.0", features = ["derive"] }  # Serialização
tokio = { version = "1", features = ["full"] }      # Async runtime
tracing = "0.1"        # Logging estruturado
rand = "0.8"           # RNG para quantum packing
nalgebra = "0.32"      # Álgebra linear (se necessário)
```

### Proibidas / Cuidado

| Crate | Motivo | Alternativa |
|-------|--------|-------------|
| `anyhow` | Error handling muito flexível | `thiserror` + `OrigoError` |
| `serde_json` em hot path | Allocation | `postcard` / `bincode` |
| `regex` | Compilação lenta | `aho-corasick` ou manual |
| `async-trait` | Overhead de boxing | `async fn` nativo (Rust 1.75+) |

---

## FFI / WASM (Futuro)

```rust
// Para WASM: compile com --target wasm32-unknown-unknown
// Use wasm-bindgen para JS bindings
// Mantenha core puro Rust (std) separado de platform-specific

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;
    // ...
}
```

---

## Checklist de PR Rust

- [ ] `cargo fmt --check` passa
- [ ] `cargo clippy -- -D warnings` passa
- [ ] `cargo test --workspace` passa
- [ ] `cargo doc --no-deps` gera sem warnings
- [ ] Novos `pub` items têm doc comments com exemplos
- [ ] Error types usam `thiserror` e `From` impls
- [ ] Nenhum `unwrap()`/`expect()` em código de produção
- [ ] `unsafe` justificado e documentado (se houver)
- [ ] Benchmarks atualizados se mudança em hot path
# Estratégia de Testes — MutaCore

> **Versão:** 4.0.26  
> **Objetivo:** Garantir correção formal, performance e estabilidade do organismo cibernético

---

## Pirâmide de Testes

```
                    ┌─────────────────┐
                    │  E2E / System   │  ◄── 5-10 testes, críticos
                    │  (Integração    │       Rust + Python + LaTeX
                    │   completa)     │
                   ┌┴─────────────────┴┐
                   │   Component       │  ◄── 20-50 testes
                   │   (Módulos isol.) │       Validator, Packing, Hodge
                  ┌┴───────────────────┴┐
                  │      Unit           │  ◄── 100+ testes
                  │   (Funções puras)   │       AST, Hash, Thresholds
                 ┌┴─────────────────────┴┐
                 │   Property-Based      │  ◄── Infinitos (proptest/Hypothesis)
                 │   (Invariantes)       │       α-equivalence, Bijectivity
                ┌┴───────────────────────┴┐
```

---

## Categorias de Teste

### 1. Unit Tests (Rust: `#[cfg(test)]`, Python: `test_*.py`)

**Objetivo:** Verificar lógica de funções puras isoladamente

| Módulo | Testes Críticos | Cobertura Alvo |
|--------|-----------------|----------------|
| `ast_node.rs` | `beta_reduction`, `substitution`, `free_variables`, `krajicek_generator` | 100% |
| `alpha_context.rs` | `alpha_equivalence`, `shadowing`, `canonicalization_idempotent` | 100% |
| `ast_validator.rs` | `gluing_validation`, `threshold_exceeded`, `warning_near_threshold` | 95% |
| `krajicek_thresholds.rs` | `check_all`, `hierarchy`, `builder`, `toml_roundtrip` | 100% |
| `hash.rs` | `alpha_equivalent_hash`, `merkle_proof`, `consensus_hash` | 100% |
| `quantum_packing.rs` | `packing`, `specialization_matching`, `network_stats` | 90% |
| `landauer_simulation.py` | `step`, `trajectory`, `metabolic_classification` | 90% |
| `hodge_matrices.py` | `boundary_matrix`, `hodge_laplacian`, `equivariance_score` | 85% |
| `spectrum1024_tests.py` | `bijectivity`, `avalanche`, `round_trip`, `differential` | 100% |

**Execução:**
```bash
# Rust
cargo test --workspace --lib

# Python
cd src/python && pytest tests/ -v --cov=src --cov-report=term-missing
```

---

### 2. Property-Based Tests

**Objetivo:** Verificar invariantes matemáticas com milhares de casos aleatórios

#### Rust (proptest)

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

    #[test]
    fn structural_hash_alpha_invariant(ast in arb_ast()) {
        let mut ctx1 = AlphaContext::new();
        let mut ctx2 = AlphaContext::new();
        let h1 = ctx1.structural_hash(&ast);
        let h2 = ctx2.structural_hash(&ast);
        prop_assert_eq!(h1, h2);
    }

    #[test]
    fn validator_rejects_over_rho_t(ast in arb_ast_with_krajicek()) {
        let mut validator = OrigoAstValidator::new_simple(100);
        // Injeta complexidade > 100
        let result = validator.validate(&ast);
        if ast.complexity() > 100 {
            prop_assert!(result.is_err());
        }
    }
}

// Estratégias customizadas
fn arb_ast() -> impl Strategy<Value = AstNode> { ... }
fn arb_ast_with_krajicek() -> impl Strategy<Value = AstNode> { ... }
```

#### Python (Hypothesis)

```python
# tests/test_property_based.py
from hypothesis import given, strategies as st, settings
import numpy as np

@settings(max_examples=1000, deadline=5000)
@given(st.lists(st.floats(-1000, 1000), min_size=1, max_size=10))
def test_delta_t_magnitude_nonnegative(components):
    magnitude = sum(c**2 for c in components)
    assert magnitude >= 0

@given(st.integers(2, 10))
def test_hodge_betti_numbers(n):
    hodge = compute_hypercube_hodge(n, max_eigenvalues_per_block=5)
    # Q_n é contrátil: β_0=1, β_k=0 para k>0
    assert hodge.blocks[0].betti_number == 1
    for k in range(1, n+1):
        assert hodge.blocks[k].betti_number == 0

@given(st.integers(1, 1000))
def test_spectrum1024_bijectivity_single_block(n):
    key = secrets.token_bytes(128)
    plaintext = secrets.token_bytes(128)
    ciphertext = spectrum1024_encrypt_block(plaintext, key)
    decrypted = spectrum1024_decrypt_block(ciphertext, key)
    assert decrypted == plaintext
```

---

### 3. Component Tests (Integração de Módulos)

**Objetivo:** Verificar interação entre módulos adjacentes

| Cenário | Módulos Envolvidos | Teste |
|---------|-------------------|-------|
| Validação completa | `alpha_context` + `ast_validator` + `hash` | `test_full_validation_pipeline` |
| Packing com validação | `quantum_packing` + `ast_validator` | `test_pack_then_validate` |
| Termodinâmica → Validador | `landauer_simulation` + `ast_validator` | `test_metabolic_feedback` |
| Hodge → Equivariância | `hodge_matrices` + `delta_t_topology` | `test_equivariance_monitoring` |

**Exemplo Rust:**
```rust
// tests/integration_full_pipeline.rs
#[test]
fn full_mutation_pipeline() {
    // 1. Recebe AST mutante
    let mut ctx = AlphaContext::new();
    let ast = generate_mutant_ast();
    
    // 2. Canonização alfa
    let canonical = ctx.canonicalize(&ast);
    
    // 3. Validação Krajicek
    let mut validator = OrigoAstValidator::new(KrajicekThresholds::standard());
    let result = validator.validate(&canonical);
    assert!(result.is_ok());
    
    // 4. Hash estrutural para packing
    let hash = StructuralHasher::hash(&canonical);
    
    // 5. Quantum packing
    let mut packer = QuantumLocalPacker::new(config, nodes).unwrap();
    let work = WorkItem::from_ast("test".into(), canonical);
    let assignments = packer.pack(vec![work]).unwrap();
    assert_eq!(assignments.len(), 1);
}
```

---

### 4. E2E / System Tests

**Objetivo:** Validar fluxo completo do organismo

```python
# tests/test_e2e_metabolic_cycle.py
def test_complete_metabolic_cycle():
    """Testa ciclo completo: Homeostase → Estímulo → Mutação → Validação → Encapsulamento"""
    
    # Setup
    thermo = MutaCoreThermodynamics()
    mutator = MutationEngine()
    validator = OrigoAstValidatorPy()  # PyO3 binding
    packer = QuantumLocalPackerPy()
    
    # 1. HOMEOSTASIS: Estado inicial estável
    state = thermo.step([5, 5, 5, 5])
    assert state.metabolic_state == "HOMEOSTASIS"
    
    # 2. ESTÍMULO: Dados exógenos chegam
    exogenous_data = generate_complex_input()
    delta_t = compute_delta_t(organism_state, exogenous_data)
    
    # 3. MUTAÇÃO: Gera AST candidata
    candidate_ast = mutator.propose_mutation(delta_t)
    
    # 4. FILTRO: Validação Krajicek
    val_result = validator.validate(candidate_ast)
    if not val_result.valid:
        # Rejeitado → organismo permanece em homeostase
        assert thermo.step([5,5,5,5]).metabolic_state == "HOMEOSTASIS"
        return
    
    # 5. ENCAPSULAMENTO: dNFT + Packing
    dnft = mint_dnft(candidate_ast)
    assignments = packer.pack([WorkItem.from_ast("mut-1", candidate_ast)])
    
    # 6. Verifica estado metabólico pós-mutação
    new_state = thermo.step(delta_t.components)
    # Se mutação resolveu gap: EXPLORATION ou HOMEOSTASIS
    assert new_state.metabolic_state in ["HOMEOSTASIS", "EXPLORATION"]
```

---

### 5. Formal Verification Tests

**Objetivo:** Validar que formalização LaTeX corresponde à implementação

```bash
# Verifica consistência LaTeX ↔ Código
# 1. Extrai equações do LaTeX
# 2. Compara com implementação Python/Rust
# 3. Testa valores conhecidos

# Exemplo: Equação fundamental ℛₗ
# LaTeX: ℛₗ = (Σ ΔTᵢ²) · exp((ρ_T - B_T)/(C_T·R_T)) · k_B T ln 2
# Python: compute_landauer_residue()
# Teste: ambos produzem mesmo resultado para inputs idênticos
```

---

## Métricas de Qualidade

### Cobertura Mínima

| Camada | Linha | Branch | Função |
|--------|-------|--------|--------|
| Rust Core | 90% | 85% | 95% |
| Python Simulations | 80% | 75% | 85% |
| SPECTRUM-1024 | 100% | 100% | 100% |

### Performance Benchmarks

```bash
# Rust: criterion
cargo bench --workspace

# Python: pytest-benchmark
pytest tests/ --benchmark-only

# Metas:
# - validate() < 1ms para AST 1000 nós
# - pack() < 100ms para 100 work items, 10 nós
# - hodge_laplacian Q_8 < 5s
# - spectrum1024 encrypt < 10ms/bloco
```

### Regression Tests

```yaml
# .github/workflows/regression.yml
# Guarda baselines de:
# - Hash outputs para ASTs conhecidas
# - Métricas de performance
# - Distribuições estatísticas (avalanche, chi²)
# Falha se desvio > 5% do baseline
```

---

## Testes de Segurança

### Fuzzing

```bash
# cargo-fuzz para Rust
cargo install cargo-fuzz
cargo fuzz run ast_parser  # Fuzzing de parser AST

# atheris para Python
pip install atheris
python -m atheris fuzz_ast_validator.py
```

### Adversarial Tests

```python
# tests/test_adversarial.py
def test_malformed_ast_rejected():
    """ASTs malformados não devem causar panic/crash."""
    malformed = [
        AstNode::Abstraction("", Box::new(AstNode::Variable("x"))),  # Var vazia
        AstNode::Application(Box::new(AstNode::Variable("x")), Box::new(AstNode::Variable("y"))),  # OK
        # ... casos de borda
    ]
    for ast in malformed {
        let result = validator.validate(&ast);
        // Deve retornar Err, não panic
        assert!(result.is_err() || result.unwrap().valid == false);
    }
```

---

## CI/CD Pipeline

### Stages

```mermaid
graph LR
    A[Push/PR] --> B[Format/Lint]
    B --> C[Unit Tests]
    C --> D[Property Tests]
    D --> E[Component Tests]
    E --> F[E2E Tests]
    F --> G[Benchmarks]
    G --> H[Security Scan]
    H --> I[LaTeX Compile]
    I --> J[Deploy Docs]
```

### Gates Obrigatórios

| Gate | Comando | Falha = Block |
|------|---------|---------------|
| Formatting | `cargo fmt --check && black --check` | ✓ |
| Linting | `cargo clippy -D warnings && ruff check` | ✓ |
| Type Check | `cargo check && mypy` | ✓ |
| Unit Tests | `cargo test --lib && pytest -m "not slow"` | ✓ |
| Property Tests | `cargo test proptest && pytest -m property` | ✓ |
| Integration | `cargo test --test integration` | ✓ |
| Benchmarks | `cargo bench && pytest --benchmark` | Compare baseline |
| Security | `cargo audit && pip-audit` | ✓ |
| LaTeX | `latexmk -pdf *.tex` | ✓ |

---

## Test Data Management

### Fixtures

```rust
// tests/fixtures/mod.rs
pub mod ast_fixtures {
    use crate::AstNode;
    
    pub fn identity() -> AstNode {
        AstNode::abstraction("x", AstNode::variable("x"))
    }
    
    pub fn church_numeral(n: u32) -> AstNode {
        // λf.λx.f^n(x)
        let mut body = AstNode::variable("x");
        for _ in 0..n {
            body = AstNode::application(AstNode::variable("f"), body);
        }
        AstNode::abstraction("f", AstNode::abstraction("x", body))
    }
}
```

### Golden Files

```bash
# tests/golden/
# golden/ast_identity.json      # Hash esperado
# golden/hodge_Q6.json          # Autovalores esperados
# golden/spectrum1024_vectors.txt # Test vectors conhecidos

# Atualização controlada:
# cargo test -- --accept  # Apenas maintainers
```

---

## Troubleshooting de Testes

| Sintoma | Causa | Ação |
|---------|-------|------|
| `AlphaIsomorphismFailure` flaky | Context não resetado | `ctx.reset()` entre testes |
| ARPACK `ArpackNoConvergence` | Matriz mal condicionada | Aumentar `max_eigenvalues` ou `tol` |
| `GluingFailure` em teste válido | Thresholds muito baixos | Usar `KrajicekThresholds::aggressive()` no teste |
| Hypothesis `Flaky` | Stateful RNG | Usar `@settings(derandomize=True)` |
| Benchmark regression | Ruído de sistema | Rodar 3x, usar mediana, isolar CPU |

---

## Documentação de Testes

Cada teste deve documentar:
1. **O que testa** (invariante/propriedade)
2. **Por que é crítico** (ligação à arquitetura)
3. **Como reproduzir falha** (se aplicável)

```rust
/// Testa que a validação rejeita mutações que excedem ρ_T (limiar de colagem).
/// 
/// CRÍTICO: Este é o "sistema imunológico" do MutaCore — previne esquizofrenia sistêmica.
/// 
/// Falha esperada: OrigoError::ProofThresholdExceeded com generator_type=RhoT.
#[test]
fn test_gluing_failure_prevents_schizophrenia() { ... }
```
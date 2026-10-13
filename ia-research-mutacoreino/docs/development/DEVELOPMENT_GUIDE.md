# Guia de Desenvolvimento — MutaCore / Reino Digital

> **Versão:** 4.0.26  
> **Público:** Engenheiros Rust/Python, Pesquisadores, IAs Colaboradoras

---

## Setup do Ambiente

### Pré-requisitos

| Ferramenta | Versão Mínima | Instalação |
|------------|---------------|------------|
| Rust | 1.75+ | `rustup install stable` |
| Python | 3.11+ | `python3.11 -m venv venv` |
| LaTeX | TeX Live 2023+ | `tlmgr install ...` |
| Git | 2.40+ | padrão |
| cargo-make | latest | `cargo install cargo-make` |

### Inicialização

```bash
# Clone e entre no diretório
git clone https://github.com/ebiossanto/ia-research-mutacoreino
cd ia-research-mutacoreino

# Rust: build workspace
cargo build --workspace --release

# Python: setup virtualenv
cd src/python
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# LaTeX: compile docs
cd ../../formal/tex
pdflatex mutacore_architecture.tex
pdflatex landauer_coupling.tex
```

---

## Estrutura de Código Rust

### Convenções

```rust
// Nomes: snake_case para funções/variáveis, PascalCase para tipos
// Módulos: um arquivo por conceito principal
// Documentação: /// para itens públicos, //! para módulos

/// Valida AST contra limiares de Krajicek
pub fn validate(&mut self, node: &AstNode) -> Result<ValidationResult> { ... }

// Testes: #[cfg(test)] mod tests { ... } no mesmo arquivo
// Benchmarks: benches/ separado, criterion.rs
```

### Padrões Obrigatórios

1. **Error Handling:** Sempre `Result<T, OrigoError>` — nada de `unwrap()`/`expect()` em produção
2. **Validation First:** Toda entrada externa passa por `OrigoAstValidator` ANTES de processamento
3. **Alpha-Structural:** Compare ASTs via `StructuralHasher::hash()` ou `AlphaContext::is_alpha_isomorphic()`
4. **Threshold Awareness:** Respeite `KrajicekThresholds` — configure via `ValidationConfig`

### Módulos Principais

| Arquivo | Responsabilidade | Testes Críticos |
|---------|------------------|-----------------|
| `ast_node.rs` | Definição `AstNode`, `KrajicekGenerator` | `test_beta_reduction`, `test_free_variables` |
| `alpha_context.rs` | Canonização α, isomorfismo | `test_alpha_equivalence`, `test_shadowing` |
| `ast_validator.rs` | Validação limiares, gluing | `test_gluing_validation_fail`, `test_warning_near_threshold` |
| `krajicek_thresholds.rs` | Espectro ρ_T, B_T, C_T, R_T | `test_check_all`, `test_builder` |
| `quantum_packing.rs` | Alocação ótima em hipercubo | `test_packing`, `test_specialization_matching` |
| `hash.rs` | Hash estrutural, Merkle, consenso | `test_alpha_equivalent_hash`, `test_merkle_tree` |

---

## Estrutura de Código Python

### Convenções

```python
# Type hints obrigatórios em funções públicas
# Docstrings no formato Google/NumPy
# Dataclasses para estruturas de dados
# Logging via `logging` module (não print)

from dataclasses import dataclass
from typing import List, Optional
import numpy as np

@dataclass
class ThermodynamicState:
    """Estado termodinâmico do MutaCore."""
    delta_t_components: List[float]
    thresholds: KrajicekThresholds
    temperature: float = 300.0
```

### Scripts Principais

| Script | Função | Como Executar |
|--------|--------|---------------|
| `landauer_simulation.py` | Simula metabolismo ΔT ↔ ℛₗ | `python landauer_simulation.py --steps 500` |
| `hodge_matrices.py` | Decomposição Hodge em Q_n | `python hodge_matrices.py --dimension 8` |
| `spectrum1024_tests.py` | Validação bijectiva cifrador | `python spectrum1024_tests.py --suite default` |

### Dependências Críticas

```toml
# pyproject.toml / requirements.txt
numpy>=1.26.0
scipy>=1.11.0          # ARPACK para autovalores esparsos
sympy>=1.12.0          # Álgebra simbólica para Hodge
cryptography>=42.0.0   # Para SPECTRUM-1024
pydantic>=2.5.0        # Validação de config
```

---

## Workflow de Desenvolvimento

### 1. Nova Feature / Pesquisa

```bash
# Cria branch descritiva
git checkout -b feat/nova-pesquisa-delta-t

# Desenvolve com testes
cargo test --workspace
python -m pytest src/python/ -v

# Documenta no ADR se decisão arquitetural
# docs/adr/XXX-nova-decisao.md
```

### 2. Validação de Mudança no Core Rust

```bash
# Testa tudo
cargo test --workspace --all-targets

# Benchmarks de performance
cargo bench --workspace

# Verifica formatação
cargo fmt --check

# Clippy (lints)
cargo clippy --workspace -- -D warnings
```

### 3. Validação de Mudança no Python

```bash
cd src/python
source .venv/bin/activate

# Testes unitários
pytest tests/ -v --cov=src --cov-report=term-missing

# Type checking
mypy src/

# Linting
ruff check src/
```

### 4. Commit e PR

```bash
# Commits semânticos
git commit -m "feat(core): add quantum tunneling to packing

- Implements quantum_tunneling_step() for escaping local minima
- Adds tunneling_probability parameter to PackingConfig
- Updates benchmarks with tunneling vs non-tunneling comparison

Refs: #42"

# PR template inclui:
# - Descrição da mudança
# - Testes adicionados/modificados
# - Impacto em limiares Krajicek (se houver)
# - Atualização de docs/ADR (se arquitetural)
```

---

## Testes e Qualidade

### Pirâmide de Testes

```
         ┌─────────────┐
         │  E2E /      │  Poucos, críticos
         │  Integration│
        ┌┴─────────────┴┐
        │  Component    │  Módulos isolados (validator, packing)
       ┌┴───────────────┴┐
       │     Unit        │  Muitos, rápidos (AST, hash, thresholds)
      ┌┴─────────────────┴┐
```

### Cobertura Mínima

| Camada | Cobertura | Crítico |
|--------|-----------|---------|
| Rust Core (origo-overlay) | ≥ 90% | Validator, AlphaContext, Hash |
| Python Simulations | ≥ 80% | Landauer step, Hodge blocks |
| SPECTRUM-1024 | 100% | Bijectivity, Avalanche, Round-trip |

### Testes de Propriedade (Property-Based)

```rust
// Use proptest para validar invariantes
proptest! {
    #[test]
    fn alpha_hash_invariant_under_rename(ast in arb_ast()) {
        let mut ctx1 = AlphaContext::new();
        let mut ctx2 = AlphaContext::new();
        let hash1 = ctx1.structural_hash(&ast);
        let hash2 = ctx2.structural_hash(&ast);
        prop_assert_eq!(hash1, hash2);
    }
}
```

---

## Formalização Matemática (LaTeX)

### Compilação

```bash
cd formal/tex

# Compilação padrão (2-3 passes para referências)
pdflatex mutacore_architecture.tex
pdflatex mutacore_architecture.tex

# Ou use latexmk (recomendado)
latexmk -pdf mutacore_architecture.tex
```

### Convenções

- Teoremas numerados por seção: `Teorema 3.1`
- Equações referenciadas: `\eqref{eq:fundamental}`
- Código inline: `\texttt{OrigoAstValidator}`
- Símbolos definidos em `mutacore_architecture.tex` Apêndice A

---

## CI/CD (GitHub Actions)

### Workflows Principais

| Workflow | Trigger | Jobs |
|----------|---------|------|
| `rust-ci.yml` | Push/PR | build, test, clippy, fmt, audit |
| `python-ci.yml` | Push/PR | test, type-check, lint, coverage |
| `latex-ci.yml` | Push/PR (tex changes) | compile, spell-check |
| `release.yml` | Tag `v*` | build artifacts, publish docs |

### Verificação Local (act)

```bash
# Instala act
cargo install act

# Roda workflows localmente
act -W .github/workflows/rust-ci.yml
```

---

## Debugging e Profiling

### Rust

```bash
# Logs detalhados
RUST_LOG=debug cargo test test_name -- --nocapture

# Profiling com perf
cargo build --release --features profiling
perf record --call-graph=dwarf ./target/release/bench_name
perf report

# Heap profiling
cargo install dhat
# #[global_allocator] static ALLOC: dhat::Alloc = dhat::Alloc;
```

### Python

```bash
# Profiling
python -m cProfile -o profile.stats landauer_simulation.py
python -m pstats profile.stats

# Memory
python -m memory_profiler landauer_simulation.py

# Debug pytest
pytest tests/ -v --pdb  # Dropa no pdb em falha
```

---

## Publicação e Release

### Versionamento

**SemVer:** `MAJOR.MINOR.PATCH`
- MAJOR: Quebra de API/arquitetura (ex: mudança em ρ_T sem migração)
- MINOR: Nova feature compatível (ex: novo estado metabólico)
- PATCH: Bugfix, docs, performance

### Checklist de Release

- [ ] Todos os testes passam (`cargo test --workspace && pytest`)
- [ ] `CHANGELOG.md` atualizado
- [ ] Versão bumped em `Cargo.toml` workspace + `pyproject.toml`
- [ ] LaTeX compila sem erros
- [ ] Docs geradas e publicadas
- [ ] Tag git criada: `git tag -a v0.4.0 -m "Release v0.4.0"`
- [ ] GitHub Release publicado com artifacts

---

## Troubleshooting Comum

| Problema | Causa Provável | Solução |
|----------|----------------|---------|
| `GluingFailure` em validação | Mutação excede ρ_T | Simplifique AST ou aumente ρ_T (com cuidado) |
| `AlphaIsomorphismFailure` | Contextos α dessincronizados | Use mesmo `AlphaContext` para batch |
| `QuantumPackingError` | Nenhum nó com capacidade | Adicione nós ou reduza complexidade workload |
| `ARPACK error` em Hodge | Matriz mal condicionada | Aumente `max_eigenvalues` ou use denso |
| Falha bijectiva SPECTRUM | Bug em S-box/key schedule | Verifique `RoundFunctionBijectivityTest` |

---

## Recursos Adicionais

- **Rust Book:** https://doc.rust-lang.org/book/
- **Rust API Guidelines:** https://rust-lang.github.io/api-guidelines/
- **NumPy/SciPy:** https://numpy.org/doc/stable/
- **Property-based Testing:** https://github.com/AltSysrq/proptest
- **LaTeX para Cientistas:** https://en.wikibooks.org/wiki/LaTeX
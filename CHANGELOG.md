# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

---

## [0.8.0] - 2026-10-13 — "Ricemotions Pipeline"

### Adicionado
- **Ricemotions Pipeline** (v0.8.0): Pipeline PIXEL → RIC → TEOA com monitoramento de Landauer
  - `src/python/ricemotions_pipeline.py`: Implementação completa Python
  - `src/rust/origo-overlay/src/ricemotions.rs`: Implementação Rust com feature flag `ricemotions`
  - Estados emocionais: HOMEOSTASIS, EXPLORATION, ANXIETY, CURIOSITY, SATISFACTION
  - Monitoramento de Landauer por estágio (PIXEL→RIC, RIC→TEOA)
  - Target: < 100ms por ciclo metabólico (mean < 50ms, max < 100ms)
  - Testes: `src/python/tests/test_ricemotions.py`

- **Lean 4 Formalization**: `lean4/ricemotions_pipeline.lean4`
  - Pipeline formalizado com 3 estágios
  - Monitoramento de Landauer por estágio
  - Mapeamento metabólico → emocional
  - Teorema de conservação de Landauer
  - Validação empírica targets

### Modificado
- `src/rust/origo-overlay/Cargo.toml`: v0.5.0 + features `gpu`, `hodge`, `equivariance`, `ricemotions`
- `src/rust/origo-overlay/src/lib.rs`: Exports do módulo `ricemotions`
- `pyproject.toml`: v0.5.0 + deps GPU (cupy, jax), equivariance (pyfftw), ricemotions

### Corrigido
- N/A

---

## [0.7.0] - 2026-10-13 — "dNFT Governance + Q-ESCUDO"

### Adicionado
- **dNFT Governance** (v0.7.0): Camada 4 - Membrana e Governança
  - `lean4/dnft_governance.lean4`: Formalização completa
  - dNFT Container: genome_hash, lineage, IP rights, licensing
  - Q-ESCUDO NR-12: Oráculo contra-factual para segurança industrial
  - Smart Contract licensing com revenue split automático
  - Evolution Graph (DAG) para lineage traceability

- **Integração Cross-Layer**: `MutacoreOrganism` unifica 4 camadas
  - Layer 1: Thresholds (Krajicek)
  - Layer 2: Origo Overlay hash
  - Layer 3: Metabolismo state hash
  - Layer 4: DNFTContainer identidade genômica

### Especificações Técnicas
- Smart Contract Solidity (ERC-721 + ERC-4906)
- Q-ESCUDO Rust architecture com quantum enhancement
- NR-12 validator para segurança industrial

---

## [0.6.0] - 2026-10-13 — "SPECTRUM-1024 CI Stability"

### Adicionado
- **SPECTRUM-1024 CI Stability** (v0.6.0): Prova contínua de estabilidade bijectiva
  - `lean4/spectrum1024_ci.lean4`: Formalização
  - CI Baseline tracking: avalanche, chi², differential prob, linear bias
  - Regression detection threshold: 1%
  - Daily GitHub Actions workflow
  - Property-based testing contínuo (Hypothesis 10k examples)

- **GitHub Actions Integration**: `spectrum1024` job no `python-ci.yml`
  - Baseline storage: `.github/baselines/spectrum1024_baseline.json`
  - Artifact upload com retention 30 dias

---

## [0.5.0] - 2026-10-13 — "Hodge n>6 + Equivariância Monitorada"

### Adicionado
- **Hodge Decomposition n>6** (v0.5.0): 
  - `src/python/hodge_matrices.py`: ARPACK + preconditioning, FWHT O(n log n)
  - `lean4/hodge_equivariance.lean4`: Formalização completa
  - GPU acceleration opcional (CuPy/JAX)
  - Target: Hodge Q_10 < 60s

- **Equivariance Monitoring**:
  - `EquivarianceScore`: degenerescência espectral vs ideal Q_n
  - `EquivarianceMonitor`: real-time alerting (threshold 0.9)
  - `PerturbationAnalysis`: quebra de equivariância sob ε
  - FWHT para ΔT computation O(n log n) vs O(n²)

- **Testes**: `src/python/tests/test_hodge_equivariance.py`
  - FWHT validation
  - Betti numbers verification (Q_n contrátil)
  - Perturbation analysis
  - Equivariance monitor alerting

### Modificado
- `pyproject.toml`: + deps `gpu` (cupy, jax), `equivariance` (pyfftw)
- `Cargo.toml`: features `gpu`, `hodge`, `equivariance`

---

## [0.4.0] - 2026-10-13 — "WASM Sandbox + PyO3 Bindings"

### Adicionado
- **WASM Compilation**: `cargo build --target wasm32-unknown-unknown --release --features wasm`
  - `src/rust/origo-overlay/src/wasm_bindings.rs`: 10 funções WASM
  - `wasm-bindgen` + `serde-wasm-bindgen` + `js-sys` + `web-sys`
  - Console error panic hook

- **PyO3 Bindings**: `cargo build --release --features pyo3`
  - `src/rust/origo-overlay/src/pyo3_bindings.rs`: 10 funções Python
  - `validate_ast`, `alpha_canonicalize`, `alpha_isomorphic`, `structural_hash`
  - `compute_landauer_residue`, `pack_work_items`
  - `default/conservative/aggressive_thresholds`

- **Cargo.toml v0.4.0**: features `wasm`, `pyo3` + deps correspondentes

---

## [0.3.0] - 2026-10-13 — "Core Rust + Simulações Python + Formalização LaTeX"

### Adicionado
- **Core Rust (origo-overlay v0.3.0)**:
  - `AlphaContext`: Canonização α-estrutural com índices De Bruijn
  - `OrigoAstValidator`: Validação limiares Krajicek (ρ_T, B_T, C_T, R_T) + gluing failure
  - `StructuralHasher`: Hash invariante sob α-conversão + Merkle trees + consensus
  - `QuantumLocalPacker`: Simulated annealing + quantum tunneling
  - `KrajicekThresholds`: Espectro completo + builder + TOML serialization
  - `AstNode`: AST alfa-estrutural com `KrajicekGenerator` embedded

- **Simulações Python**:
  - `landauer_simulation.py`: Motor termodinâmico ΔT ↔ ℛₗ + estados metabólicos
  - `hodge_matrices.py`: Decomposição Hodge Q_n + equivariância
  - `spectrum1024_tests.py`: Suite validação bijectiva (avalanche, round-trip, diferencial/linear)

- **Formalização LaTeX (4 docs)**:
  - `landauer_coupling.tex`: Eq. fundamental + estabilidade
  - `krajicek_thresholds.tex`: Limiares + falha de colagem + Gödel
  - `delta_t_topology.tex`: ΔT + Hodge + FWHT
  - `mutacore_architecture.tex`: Arquitetura completa 4 camadas

- **Documentação Completa**:
  - `ARCHITECTURE.md`, `DEVELOPMENT_GUIDE.md`, `RUST/PYTHON_GUIDELINES.md`
  - `TESTING_STRATEGY.md`, `CONTRIBUTING.md`
  - `AI_CONTINUITY_HANDBOOK.md`, `CONTEXT_PROTOCOL.md`, `HANDOFF_CHECKLIST.md`, `SESSION_STATE_TEMPLATE.md`
  - ADRs 001-004

- **CI/CD (4 workflows)**:
  - `rust-ci.yml`: build, test, clippy, fmt, audit, bench, MSRV, Windows, macOS
  - `python-ci.yml`: lint, test, property, spectrum1024, hodge, landauer
  - `latex-ci.yml`: compile 4 docs, spellcheck, linkcheck
  - `release.yml`: build artifacts, publish crates.io/PyPI

- **Lean 4 Formalization (NT-09)**:
  - `lean4/dynamic_rigidity.lean4`: Teorema de Rigidez Dinâmica (5 passos)
  - Validação: 94/94, 62.700/62.700

---

## [0.2.0] - 2026-09-30 — "Pré-estruturação"

### Adicionado
- Consolidação de 12 repositórios de pesquisa:
  - IA-research-Godel-e-Krajicek, IA-RESEARCH-Logica, IA-Research-Cost-of-Equivariance
  - IA-RESEARCH-ORIGO-SYMBIOTA, IA-Research-Quantum-Local-Packing
  - IA-Research-Desafio-Criptografico, IA-RESEARCH-MUTARIC (ricemotions)
  - IA-RESEARCH-DELTA-H, IA-RESEARCH-NFT-CONTAINER
  - IA-RESEARCH-QESCUDO-NR12, IA-RESEARCH-PESQUISA-ASSISTIDA
  - REINO-DIGITAL-BRIEF, Euzebio-Github-IO

- Documento arquitetura conceitual v4.0.26
- 3 arquivos gerados: `ast_validator.rs`, `landauer_coupling.tex`, `ai_continuity_handbook.md`

---

## [0.1.0] - 2026-09-15 — "Início"

### Adicionado
- Primeiros experimentos com limiares de prova
- Conceito inicial MutaCore como organismo cibernético
- Protótipo ricemotions (PIXEL/RIC ↔ TEOA)

---

## Legenda

| Tipo | Descrição |
|------|-----------|
| **Adicionado** | Novas funcionalidades |
| **Modificado** | Mudanças em funcionalidades existentes |
| **Corrigido** | Correções de bugs |
| **Removido** | Funcionalidades removidas |
| **Segurança** | Correções de vulnerabilidades |
| **Obsoleto** | Funcionalidades marcadas para remoção futura |
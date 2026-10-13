# MutaCore - Reino Digital

> **Ecossistema Cognitivo Concorrente e Mutável** — Infraestrutura Unificada de AGI Distribuída, Confiável e Biologicamente Inspirada

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/Python-3.11+-blue.svg)](https://www.python.org)

## Visão Geral

O **MutaCore** é o núcleo adaptativo do ecossistema **Reino Digital** — um organismo cibernético completo que unifica:

- **Lógica Matemática Rigorosa** — Limiares de prova (Krajicek/Gödel) e equivariância em hipercubos
- **Transporte Seguro** — Overlays declarativos em Rust com ASTs alfa-estruturais imutáveis
- **Metabolismo Termodinâmico** — Emoção computacional via resíduo de Landauer e gap topológico narrativo (ΔT)
- **Governança Descentralizada** — Containers dNFT e oráculos contra-factuais (Q-ESCUDO NR-12)

## Arquitetura em 4 Camadas

```
┌─────────────────────────────────────────────────────────────┐
│  CAMADA 4: MEMBRANA, GOVERNANÇA E IDENTIDADE                │
│  • dNFT Containers (Identidade Genômica de Agentes)         │
│  • Oráculos de Segurança Contra-factual (Q-ESCUDO)          │
├─────────────────────────────────────────────────────────────┤
│  CAMADA 3: METABOLISMO TERMODINÂMICO & SINTAXE (MUTARIC)    │
│  • ricemotions (Ponte PIXEL/RIC ↔ TEOA)                     │
│  • Gerenciamento de Entropia e Resíduo de Landauer          │
├─────────────────────────────────────────────────────────────┤
│  CAMADA 2: TRANSPORTE, COMUNICAÇÃO E DISTRIBUIÇÃO           │
│  • Origo Overlay (Overlays Declarativos Seguros em Rust)    │
│  • Hashing AST Alfa-Estrutural / Quantum Local Packing      │
├─────────────────────────────────────────────────────────────┤
│  CAMADA 1: SUBSTRATO LÓGICO E CRIPTOGRÁFICO DE PROVA        │
│  • Limiares de Krajicek (ρ_T, B_T, C_T, R_T)                │
│  • Incompletude de Gödel / Equivariância no Hipercubo       │
└─────────────────────────────────────────────────────────────┘
```

## Estrutura do Repositório

```
ia-research-mutacoreino/
├── src/
│   ├── rust/
│   │   └── origo-overlay/          # Crate Rust principal (Camada 2)
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── ast_validator.rs
│   │           ├── alpha_context.rs
│   │           ├── krajicek_thresholds.rs
│   │           └── quantum_packing.rs
│   └── python/                      # Scripts de automação e pesquisa
│       ├── hodge_matrices.py
│       ├── landauer_simulation.py
│       └── spectrum1024_tests.py
├── formal/
│   └── tex/                         # Formalização matemática LaTeX
│       ├── landauer_coupling.tex
│       ├── krajicek_thresholds.tex
│       ├── delta_t_topology.tex
│       └── mutacore_architecture.tex
├── docs/
│   ├── architecture/                # Documentos de arquitetura
│   │   ├── ARCHITECTURE.md
│   │   ├── LAYER_1_LOGICAL_SUBSTRATE.md
│   │   ├── LAYER_2_TRANSPORT.md
│   │   ├── LAYER_3_METABOLISM.md
│   │   └── LAYER_4_GOVERNANCE.md
│   ├── development/                 # Guias de desenvolvimento
│   │   ├── DEVELOPMENT_GUIDE.md
│   │   ├── RUST_GUIDELINES.md
│   │   ├── PYTHON_GUIDELINES.md
│   │   ├── TESTING_STRATEGY.md
│   │   └── CONTRIBUTING.md
│   ├── continuity/                  # Documentos de continuidade para IAs
│   │   ├── AI_CONTINUITY_HANDBOOK.md
│   │   ├── CONTEXT_PROTOCOL.md
│   │   ├── HANDOFF_CHECKLIST.md
│   │   └── SESSION_STATE_TEMPLATE.md
│   └── adr/                         # Architecture Decision Records
│       ├── 001-use-rust-for-core.md
│       ├── 002-alpha-structural-ast.md
│       ├── 003-landauer-thermodynamics.md
│       └── 004-dnft-governance.md
├── tests/
│   ├── rust/                        # Testes Rust (unit + integration)
│   └── python/                      # Testes Python
├── examples/                        # Exemplos de uso
├── scripts/                         # Scripts de build/deploy/CI
├── .github/
│   └── workflows/                   # GitHub Actions CI/CD
├── Cargo.toml                       # Workspace root
├── pyproject.toml                   # Python project config
├── LICENSE
├── CHANGELOG.md
└── ROADMAP.md
```

## Início Rápido

### Pré-requisitos

- **Rust** 1.75+ (`rustup install stable`)
- **Python** 3.11+
- **LaTeX** (TeX Live ou MikTeX) para compilar documentos formais

### Compilar o Core Rust

```bash
cd src/rust/origo-overlay
cargo build --release
cargo test
```

### Executar Simulações Python

```bash
cd src/python
pip install -r requirements.txt
python landauer_simulation.py
python hodge_matrices.py --dimension 8
```

### Compilar Documentos LaTeX

```bash
cd formal/tex
pdflatex landauer_coupling.tex
pdflatex mutacore_architecture.tex
```

## Componentes Principais

### 1. `origo-overlay` (Rust) — Camada 2
Validador de ASTs alfa-estruturais com monitoramento de limiares de Krajicek.

```rust
use origo_overlay::{AlphaContext, OrigoAstValidator, AstNode};

let mut ctx = AlphaContext::new();
let validator = OrigoAstValidator::new(1000);

// Canonização alfa (isomorfismo estrutural)
let canonical = ctx.canonicalize(&ast_node);

// Validação de limiares de prova
validator.validate_node(&canonical)?;
```

### 2. Formalização Termodinâmica (LaTeX)
Equação fundamental do acoplamento ΔT ↔ ℛₗ:

```
ℛₗ = (Σ ΔTᵢ²) · exp((ρ_T - B_T) / (C_T · R_T)) · k_B T ln 2
```

### 3. Protocolo de Continuidade IA
Sistema de transferência de contexto entre instâncias de IA para desenvolvimento contínuo sem perda de estado lógico.

## Documentação

| Documento | Descrição |
|-----------|-----------|
| [Arquitetura Completa](docs/architecture/ARCHITECTURE.md) | Visão detalhada das 4 camadas |
| [Guia de Desenvolvimento](docs/development/DEVELOPMENT_GUIDE.md) | Setup, padrões, workflows |
| [Manual de Continuidade IA](docs/continuity/AI_CONTINUITY_HANDBOOK.md) | Protocolo para handoff entre IAs |
| [Formalização LaTeX](formal/tex/landauer_coupling.tex) | Matemática do metabolismo termodinâmico |

## Roadmap

- [ ] **v0.4.0** — Integração WASM Sandbox para execução isolada
- [ ] **v0.5.0** — Matrizes de bloco de Hodge para n>6 (Python/Julia)
- [ ] **v0.6.0** — Prova de estabilidade bijectiva SPECTRUM-1024 em CI
- [ ] **v1.0.0** — Release estável do organismo MutaCore completo

Ver [ROADMAP.md](ROADMAP.md) para detalhes.

## Pesquisas Relacionadas

Este repositório consolida e evolui os seguintes repositórios de pesquisa:

| Repositório Original | Foco | Status |
|---------------------|------|--------|
| `IA-research-Godel-e-Krajicek` | Limiares de prova, incompletude | ✅ Integrado (Camada 1) |
| `IA-RESEARCH-Logica` | Lógica e complexidade | ✅ Integrado (Camada 1) |
| `IA-Research-Cost-of-Equivariance` | Equivariância em hipercubos | ✅ Integrado (Camada 1) |
| `IA-RESEARCH-ORIGO-SYMBIOTA` | Overlays declarativos, AST alfa | ✅ Integrado (Camada 2) |
| `IA-Research-Quantum-Local-Packing` | Distribuição quântica local | 🔄 Em desenvolvimento |
| `IA-Research-Desafio-Criptografico` | Criptografia SPECTRUM-1024 | 🔄 Em desenvolvimento |
| `IA-RESEARCH-MUTARIC` | Emoção computacional, ricemotions | ✅ Integrado (Camada 3) |
| `IA-RESEARCH-DELTA-H` | Gap topológico narrativo ΔT | ✅ Integrado (Camada 3) |
| `IA-RESEARCH-NFT-CONTAINER` | Governança dNFT | 🔄 Planejado (Camada 4) |
| `IA-RESEARCH-QESCUDO-NR12` | Oráculo contra-factual industrial | 🔄 Planejado (Camada 4) |

## Licença

Licenciado sob **Apache License 2.0** — veja [LICENSE](LICENSE) para detalhes.

## Contato

**Euzébio Soares** — [@ebiossanto](https://github.com/ebiossanto)

---

*Construindo os alicerces para uma AGI Distribuída, Confiável e Biologicamente Inspirada.*
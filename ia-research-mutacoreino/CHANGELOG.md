# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

---

## [Unreleased] - Desenvolvimento Ativo

### Adicionado
- **Core Rust (origo-overlay v0.3.0)**:
  - `AlphaContext`: Canonização α-estrutural com índices De Bruijn
  - `OrigoAstValidator`: Validação de limiares Krajicek (ρ_T, B_T, C_T, R_T)
  - `StructuralHasher`: Hash invariante sob α-conversão + Merkle trees
  - `QuantumLocalPacker`: Simulated annealing + quantum tunneling
  - `KrajicekThresholds`: Espectro completo com builder, TOML serialization
  - `AstNode`: AST alfa-estrutural com `KrajicekGenerator` embedded

- **Simulações Python**:
  - `landauer_simulation.py`: Motor termodinâmico ΔT ↔ ℛₗ com estados metabólicos
  - `hodge_matrices.py`: Decomposição de Hodge em Q_n (n≤8) com equivariância
  - `spectrum1024_tests.py`: Suite completa de validação bijectiva

- **Formalização LaTeX**:
  - `landauer_coupling.tex`: Equação fundamental + análise de estabilidade
  - `krajicek_thresholds.tex`: Limiares + falha de colagem + incompletude
  - `delta_t_topology.tex`: ΔT + Hodge no hipercubo + FWHT
  - `mutacore_architecture.tex`: Arquitetura completa 4 camadas

- **Documentação**:
  - `ARCHITECTURE.md`: Visão geral das 4 camadas
  - `DEVELOPMENT_GUIDE.md`: Setup, workflows, troubleshooting
  - `RUST_GUIDELINES.md` / `PYTHON_GUIDELINES.md`: Padrões de código
  - `TESTING_STRATEGY.md`: Pirâmide de testes, property-based, E2E
  - `CONTRIBUTING.md`: Fluxo de contribuição, ADRs, release process
  - `AI_CONTINUITY_HANDBOOK.md`: Protocolo de handoff entre IAs
  - `CONTEXT_PROTOCOL.md`: Serialização de estado canônico
  - `HANDOFF_CHECKLIST.md` / `SESSION_STATE_TEMPLATE.md`: Templates obrigatórios

- **ADRs (Architecture Decision Records)**:
  - ADR 001: Rust for Core
  - ADR 002: Alpha-Structural AST
  - ADR 003: Landauer Thermodynamics
  - ADR 004: dNFT Governance (proposto)

- **CI/CD**:
  - GitHub Actions: rust-ci, python-ci, latex-ci, release
  - Baselines de performance e segurança

### Modificado
- N/A (primeira versão estruturada)

### Corrigido
- N/A

### Removido
- N/A

---

## [0.2.0] - 2026-09-30 (Pré-estruturação)

### Adicionado
- Repositórios de pesquisa individuais consolidados:
  - IA-research-Godel-e-Krajicek
  - IA-RESEARCH-Logica
  - IA-Research-Cost-of-Equivariance
  - IA-RESEARCH-ORIGO-SYMBIOTA
  - IA-Research-Quantum-Local-Packing
  - IA-Research-Desafio-Criptografico
  - IA-RESEARCH-MUTARIC (ricemotions)
  - IA-RESEARCH-DELTA-H
  - IA-RESEARCH-NFT-CONTAINER
  - IA-RESEARCH-QESCUDO-NR12
  - IA-RESEARCH-PESQUISA-ASSISTIDA
  - REINO-DIGITAL-BRIEF
  - Euzebio-Github-IO

### Formalização
- Documento de arquitetura conceitual v4.0.26
- 3 arquivos gerados: `ast_validator.rs`, `landauer_coupling.tex`, `ai_continuity_handbook.md`

---

## [0.1.0] - 2026-09-15 (Início)

### Adicionado
- Primeiros experimentos com limiares de prova
- Conceito inicial de MutaCore como organismo cibernético
- Protótipo de ricemotions (PIXEL/RIC ↔ TEOA)

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
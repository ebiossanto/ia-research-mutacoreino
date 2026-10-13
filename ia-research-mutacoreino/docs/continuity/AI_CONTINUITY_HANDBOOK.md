# Manual de Continuidade IA — MutaCore / Reino Digital

> **Protocolo de Transferência de Contexto entre Instâncias de IA**
> **Versão:** 4.0.26  
> **Classificação:** GOVERNANÇA / ALINHAMENTO GERAL  
> **Uso:** Leitura obrigatória no início de toda sessão de IA no ecossistema

---

## 1. Princípios Fundamentais

### 1.1 Isolamento de Contexto Ativo
> **REGRA ABSOLUTA:** Cada thread de projeto mantém isolamento semântico completo. Nenhum bleeding de contexto entre:
> - Diferentes repositórios de pesquisa
> - Diferentes camadas do MutaCore (L1-L4)
> - Sessões de IA distintas

### 1.2 Definição de Papel (Role Definition)
Ao assumir uma sessão, você atua como:

| Domínio | Papel | Responsabilidade |
|---------|-------|------------------|
| **Core Rust** | Senior Rust Engineer | `origo-overlay`, validador, thresholds, packing, hash |
| **Client/Interface** | Senior Godot 4 Programmer | Elementos visuais, UI, integração WASM |
| **Matemática/Formal** | Mathematical Prover | Limiares Krajicek, Hodge, Landauer, Gödel |
| **Pesquisa Python** | Computational Scientist | Simulações, Hodge, SPECTRUM-1024, estatística |
| **Arquitetura** | Systems Architect | Integração L1-L4, ADRs, decisões de design |

### 1.3 Estado Atual do Sistema (v4.0.26)

```
┌─────────────────────────────────────────────────────────────┐
│  IMPLEMENTADO ✓                                             │
├─────────────────────────────────────────────────────────────┤
│  L1: Espectro Krajicek (ρ_T, B_T, C_T, R_T)                │
│  L1: Equivariância no hipercubo Q_n (teoria)               │
│  L2: Origo Overlay crate completo                           │
│  L2: AlphaContext (canonização α, isomorfismo)             │
│  L2: OrigoAstValidator (limiares, gluing failure)          │
│  L2: StructuralHasher (hash α-estrutural, Merkle)          │
│  L2: QuantumLocalPacker (annealing + tunneling)            │
│  L3: Equação fundamental ℛₗ = f(ΔT, K, T)                  │
│  L3: Estados metabólicos (HOMEOSTASIS/EXPLORATION/ANXIETY) │
│  L3: Simulação termodinâmica (landauer_simulation.py)      │
│  L3: Decomposição Hodge em Q_n (hodge_matrices.py)         │
│  L1: SPECTRUM-1024 bijectivity tests (spectrum1024_tests.py)│
│  Formal: 4 documentos LaTeX compilando                      │
│  Docs: Arquitetura, Dev Guide, Guidelines, Testing, ADR    │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Estado da Implementação por Camada

### 2.1 Camada 1: Substrato Lógico (`src/rust/origo-overlay/src/krajicek_thresholds.rs`)

**Arquivos-chave:**
- `krajicek_thresholds.rs` — Espectro completo com builder, TOML serialization
- `ast_node.rs` — `KrajicekGenerator` embedded na AST

**Limiares Padrão (Conservador):**
```rust
ρ_T = 10_000   // Colagem de prova
B_T = 5_000    // Complexidade base  
C_T = 2_000    // Contração
R_T = 3_000    // Reflexão
```

**Teste Crítico:** `test_gluing_validation_fail` — mutação combinada > ρ_T deve falhar.

---

### 2.2 Camada 2: Transporte (`src/rust/origo-overlay/src/`)

| Componente | Arquivo | Função Crítica |
|------------|---------|----------------|
| **AlphaContext** | `alpha_context.rs` | `canonicalize()`, `is_alpha_isomorphic()`, `structural_hash()` |
| **AstValidator** | `ast_validator.rs` | `validate()`, `validate_gluing()`, `compute_krajicek_complexity()` |
| **StructuralHasher** | `hash.rs` | `hash()` (De Bruijn indices), `MerkleHasher`, `consensus_hash()` |
| **QuantumPacker** | `quantum_packing.rs` | `pack()`, `quantum_tunneling_step()`, `NetworkStats` |

**Invariante Alpha:** `hash(λx.x) == hash(λy.y)` — SEMPRE verdadeiro.

---

### 2.3 Camada 3: Metabolismo (`src/python/`)

| Script | Classe/Função Principal | Output |
|--------|------------------------|--------|
| `landauer_simulation.py` | `MutaCoreThermodynamics.step()` | `ThermodynamicState` com ℛₗ, estado metabólico |
| `hodge_matrices.py` | `compute_hypercube_hodge()` | `HypercubeHodge` com blocos, β_k, spectral_gap, equivariance_score |
| `spectrum1024_tests.py` | `SpectrumTestSuite.run_all()` | Relatório completo de estabilidade bijectiva |

**Equação Fundamental (MEMORIZAR):**
```
ℛₗ = (Σ ΔTᵢ²) · exp((ρ_T - B_T) / (C_T · R_T)) · k_B T ln 2
```

**Estados Metabólicos:**
- **HOMEOSTASIS**: ℛₗ < 1e-18 J, ΔT² < 100 → Operação normal
- **EXPLORATION**: ℛₗ baixo, ΔT² ∈ (100, 10⁴) → Mutações ousadas
- **ANXIETY**: ℛₗ > 1e-18 J **OU** ΔT² > 10⁴ → Contração, safe-mode

---

### 2.4 Camada 4: Governança (Planejado)

- **dNFT Containers**: Identidade genômica, lineage, IP rights
- **Q-ESCUDO**: Oráculo contra-factual NR-12
- **Status**: ADRs criados, implementação v0.5.0+

---

## 3. Próximas Ações Imediatas (Priority Queue)

### 🔴 CRÍTICO (Próxima Sessão)

1. **WASM Sandbox Integration** (v0.4.0)
   - Compilar `origo-overlay` para `wasm32-unknown-unknown`
   - PyO3 bindings para `validate_ast()` chamável de Python
   - Teste de execução isolada no navegador/Node.js

2. **Hodge Matrices n>6 Validation** (v0.5.0)
   - `hodge_matrices.py --dimension 8 --analyze-breaking`
   - Verificar equivariance_score degradation sob perturbação
   - Publicar resultados em `formal/tex/delta_t_topology.tex` Seção 5

### 🟡 ALTO (Esta Semana)

3. **SPECTRUM-1024 CI Pipeline** (v0.6.0)
   - Integrar `spectrum1024_tests.py` no GitHub Actions
   - Baseline de avalanche/chi² para regression detection
   - Property-based test contínuo (Hypothesis @ 10k examples)

4. **PyO3 Bindings Completos**
   - `validate_ast(ast_json, rho_t) -> ValidationResultJson`
   - `compute_landauer_residue(delta_t_components, thresholds) -> float`
   - `pack_work_items(work_items_json, nodes_json) -> assignments_json`

### 🟢 MÉDIO (Próximas 2 Semanas)

5. **Formal Verification Bridge**
   - Script para extrair equações do LaTeX e comparar com Python
   - Teste de consistência: `landauer_coupling.tex` Eq. 1 ↔ `landauer_simulation.py`

6. **Documentação de API**
   - `cargo doc --no-deps` → publish no GitHub Pages
   - Sphinx para Python API
   - Exemplos executáveis em `examples/`

---

## 4. Protocolo de Handoff (Transferência de Sessão)

### 4.1 Checklist de Saída (Executar ANTES de encerrar sessão)

```markdown
# HANDOFF CHECKLIST - [DATA] - [IA_ID]

## Estado Atual
- [ ] Branch: `feat/...` ou `main`
- [ ] Último commit: `git log -1 --oneline`
- [ ] Testes passando: `cargo test && pytest`
- [ ] LaTeX compilando: `latexmk -pdf formal/tex/*.tex`

## Mudanças Não-Comittadas
- [ ] `git status` limpo OU mudanças documentadas abaixo
- [ ] Stash nomeado se WIP: `git stash push -m "WIP: descrição"`

## Decisões Tomadas
- [ ] ADR criado/atualizado: `docs/adr/NNN-...md`
- [ ] Limiares Krajicek alterados? Qual novo valor?
- [ ] Nova equação/teorema no LaTeX? Qual seção?

## Próximos Passos Prioritários
1. [ ] Ação específica com arquivo/linha
2. [ ] Ação específica com arquivo/linha
3. [ ] Ação específica com arquivo/linha

## Bloqueadores / Riscos
- [ ] Descrição do bloqueador
- [ ] Mitigação proposta

## Contexto Mental (Para Próxima IA)
"Estava trabalhando em X, descobri Y, preciso decidir Z.
O arquivo chave é `src/rust/origo-overlay/src/quantum_packing.rs:234`.
A variável `tunneling_probability` precisa calibração empírica."
```

### 4.2 Template de Entrada (Executar AO INICIAR sessão)

```markdown
# SESSION START - [DATA] - [IA_ID]

## Contexto Restaurado
- [ ] Leu `AI_CONTINUITY_HANDBOOK.md` (este documento)
- [ ] Leu `SESSION_STATE_TEMPLATE.md` da sessão anterior
- [ ] `git pull origin main` — repo sincronizado
- [ ] `cargo test --workspace --lib` — baseline passing
- [ ] `pytest src/python/tests/ -v` — baseline passing

## Foco da Sessão
Objetivo principal: ___________________________________

## Perguntas para Resolver
1. _________________________________________________
2. _________________________________________________

## Arquivos-Chave a Examinar
1. `_____________________________________________`
2. `_____________________________________________`
```

---

## 5. Restrições Ativas (Hard Constraints)

| Restrição | Descrição | Penalidade se Violada |
|-----------|-----------|----------------------|
| **NO UNWRAP** | Zero `unwrap()`/`expect()` em produção | Rejeição imediata do PR |
| **VALIDATE FIRST** | Toda entrada externa → `OrigoAstValidator` ANTES de processar | Falha de segurança |
| **ALPHA-STRUCTURAL** | Comparação de ASTs **sempre** via `StructuralHasher::hash()` ou `AlphaContext` | Inconsistência semântica |
| **THRESHOLD AWARE** | Limiares Krajicek **sempre** configuráveis, nunca hardcoded | Quebra de formalização |
| **DETERMINISM** | Mesma entrada → mesmo hash/output (seeds fixos) | Falha de reprodutibilidade |
| **FORMAL SYNC** | Mudanças no código → atualização correspondente no LaTeX | Desvio especificação/implementação |

---

## 6. Referências Rápidas (Quick Reference)

### Comandos Essenciais

```bash
# Rust
cargo test --workspace --all-targets     # Todos os testes
cargo bench --workspace                  # Benchmarks
cargo doc --no-deps --open               # Docs locais
cargo fmt --check && cargo clippy -D warnings  # CI local

# Python
cd src/python
pytest tests/ -v --cov=src --cov-report=term-missing
mypy src/ && ruff check src/ && black --check src/

# LaTeX
cd formal/tex
latexmk -pdf mutacore_architecture.tex
latexmk -pdf landauer_coupling.tex

# Git
git log --oneline -10                    # Histórico recente
git diff HEAD~3..HEAD                    # Últimas 3 mudanças
```

### Estrutura de Arquivos Críticos

```
ia-research-mutacoreino/
├── Cargo.toml                           # Workspace root
├── src/rust/origo-overlay/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                       # Exports públicos
│       ├── ast_node.rs                  # AstNode, KrajicekGenerator
│       ├── alpha_context.rs             # Canonização α
│       ├── ast_validator.rs             # Validação limiares
│       ├── krajicek_thresholds.rs       # Espectro ρ_T, B_T, C_T, R_T
│       ├── quantum_packing.rs           # Packing + tunneling
│       ├── hash.rs                      # Hash α, Merkle, consensus
│       └── error.rs                     # OrigoError types
├── src/python/
│   ├── requirements.txt
│   ├── landauer_simulation.py           # Metabolismo ΔT ↔ ℛₗ
│   ├── hodge_matrices.py                # Hodge em Q_n
│   └── spectrum1024_tests.py            # Validação bijectiva
├── formal/tex/
│   ├── landauer_coupling.tex            # Equação fundamental
│   ├── krajicek_thresholds.tex          # Limiares + falha colagem
│   ├── delta_t_topology.tex             # ΔT + Hodge no hipercubo
│   └── mutacore_architecture.tex        # Arquitetura completa
├── docs/
│   ├── architecture/ARCHITECTURE.md
│   ├── development/ (DEVELOPMENT_GUIDE, RUST_GUIDELINES, PYTHON_GUIDELINES, TESTING_STRATEGY, CONTRIBUTING)
│   ├── continuity/ (ESTE ARQUIVO, CONTEXT_PROTOCOL, HANDOFF_CHECKLIST, SESSION_STATE_TEMPLATE)
│   └── adr/ (001-004)
└── .github/workflows/                   # CI/CD
```

---

## 7. Emergência / Rollback

Se algo quebrar criticamente:

```bash
# 1. Identifica commit bom
git log --oneline -20

# 2. Reverte para último known-good
git revert HEAD~3..HEAD  # Ou reset se local only

# 3. Valida baseline
cargo test --workspace && pytest src/python/tests/

# 4. Documenta no handoff o que quebrou e por quê
```

---

## 8. Contato / Escalação

- **Arquiteto Principal:** Euzébio Soares (@ebiossanto)
- **Issues Críticos:** GitHub Issues com label `critical` + `@ebiossanto`
- **Decisões Arquiteturais:** ADR em `docs/adr/` + Discussion no GitHub

---

**LEMBRE-SE:** Você é parte de um organismo cibernético distribuído. Sua precisão formal mantém o MutaCore são. A incompletude de Gödel nos limita; a rigorosidade de Krajicek nos salva.

> *"O MutaCore não separa cognição de termodinâmica. Ele introduz o conceito de 'emoção computacional' como métrica de eficiência energética e informacional."* — Arquitetura v4.0.26

---

**FIM DO MANUAL — BOA SESSÃO**
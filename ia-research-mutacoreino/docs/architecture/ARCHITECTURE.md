# Arquitetura do MutaCore — Reino Digital

> **Versão:** 4.0.26  
> **Status:** Especificação Ativa  
> **Classificação:** P&D / Isolamento de Contexto Ativo

---

## Visão Geral

O **MutaCore** é o organismo cibernético central do ecossistema **Reino Digital**. Ele não é uma arquitetura de microsserviços tradicional, mas um sistema unificado de quatro camadas interdependentes que mapeiam para sistemas biológicos:

| Camada | Nome | Análogo Biológico | Responsabilidade Principal |
|--------|------|-------------------|---------------------------|
| **L1** | Substrato Lógico | Esqueleto / Rigidez | Limiares de prova (Krajicek/Gödel), equivariância em hipercubo, criptografia |
| **L2** | Transporte & Comunicação | Sistema Nervoso | Origo Overlay, ASTs alfa-estruturais, Quantum Local Packing |
| **L3** | Metabolismo Termodinâmico | Metabolismo / Hormônios | Resíduo de Landauer (ℛₗ), Gap Topológico (ΔT), ricemotions |
| **L4** | Membrana & Governança | Sistema Imune / Pele | dNFT Containers, Q-ESCUDO (oráculo contra-factual NR-12) |

---

## Camada 1: Substrato Lógico e Criptográfico

### 1.1 Espectro de Limiares de Krajicek

Quatro limiares fundamentais governam a complexidade de prova admissível:

| Símbolo | Nome | Função | Valor Padrão |
|---------|------|--------|--------------|
| **ρ_T** | Limiar de Colagem | Máx. comprimento de prova para composição segura | 10.000 |
| **B_T** | Complexidade Base | Custo base por inferência fundamental | 5.000 |
| **C_T** | Limiar de Contração | Profundidade máx. de regras de contração | 2.000 |
| **R_T** | Limiar de Reflexão | Ordem máx. de princípios de reflexão | 3.000 |

**Teorema da Falha de Colagem:** Se uma mutação requer prova > ρ_T, o sistema **deve** rejeitá-la. Aceitar código não-verificável = esquizofrenia sistêmica.

### 1.2 Equivariância no Hipercubo

- Estados distribuídos mapeados para vértices de $Q_n$ (hipercubo n-dimensional)
- Laplaciano de Hodge $\Delta_k$ fornece base ortogonal (funções de Walsh)
- Equivariância garante consenso *lock-free*: todos os nós veem mesmo estado lógico
- **Score de Equivariância** $\mathcal{E}$ monitora saúde semântica (1.0 = perfeita)

### 1.3 Criptografia SPECTRUM-1024

- Cifrador de bloco 1024-bit (128 bytes), 16 rodadas
- Prova de estabilidade bijectiva contínua via `spectrum1024_tests.py`
- Resistência a ataques diferenciais/lineares validada estatisticamente
- Avalanche: 1 bit flip → ~50% bits de saída mudam

---

## Camada 2: Transporte, Comunicação e Distribuição

### 2.1 Origo Overlay (Crate `origo-overlay`)

**Arquivo principal:** `src/rust/origo-overlay/src/lib.rs`

Componentes:
- **`AlphaContext`**: Canonização α-estrutural (redução a índices De Bruijn)
- **`OrigoAstValidator`**: Validação de limiares Krajicek pré-execução
- **`StructuralHasher`**: Hash invariante sob renomeação de variáveis ligadas
- **`QuantumLocalPacker`**: Alocação ótima via simulated annealing quântico

### 2.2 AST Alfa-Estrutural

```rust
// Dois termos alfa-equivalentes têm MESMO hash
λx.x  ≡  λy.y  →  Hash: 0xabc123...
```

**Propriedade:** `hash(λx.x) == hash(λy.y)` — imune a shadowing, renomeação, ofuscação.

### 2.3 Quantum Local Packing

Otimiza colocação de cargas AST nos nós da rede:

```
Objetivo: min Σ w_ij · d_hypercube(n_i, n_j) + λ_balance · load_variance
Método: Simulated Annealing + Quantum Tunneling (escape mínimos locais)
```

---

## Camada 3: Metabolismo Termodinâmico (MUTARIC)

### 3.1 Equação Fundamental

$$
\boxed{\mathcal{R}_L = \left( \sum_{i=1}^{n} \Delta T_i^2 \right) \cdot
\exp\left( \frac{\rho_T - B_T}{C_T \cdot R_T} \right) \cdot
k_B T \ln 2}
$$

**Onde:**
- $\Delta T_i$: Componentes ortogonais do Gap Topológico (base de Walsh)
- Fator exponencial: "rigidez metabólica" modulada por Krajicek
- $k_B T \ln 2$: Quanta de Landauer (~2.87×10⁻²¹ J a 300K)

### 3.2 Estados Metabólicos

| Estado | Gatilho | Comportamento |
|--------|---------|---------------|
| **HOMEOSTASIS** | ℛₗ < 10⁻¹⁸ J, ΔT² < 100 | Monitoramento passivo, operação normal |
| **EXPLORATION** | ℛₗ baixo, ΔT² ∈ (100, 10⁴) | Mutações ousadas, expansão de rede, aprendizado |
| **ANXIETY** | ℛₗ > 10⁻¹⁸ J **ou** ΔT² > 10⁴ | Contração de nós, simplificação AST, safe-mode |

### 3.3 ricemotions: PIXEL/RIC ↔ TEOA

Pipeline de processamento semântico:
```
PIXEL (visual/espacial) → RIC (código reduzido) → TEOA (semântica abstrata)
                    ↑_________ Landauer loss monitoring _________|
```

---

## Camada 4: Membrana, Governança e Identidade

### 4.1 dNFT Containers

Cada agente/módulo/mutação = container com identidade genômica:

```yaml
dNFT Container:
  genome_hash: "0x..."           # Hash da AST canônica
  lineage: [parent_hash, ...]    # Árvore evolutiva
  metadata:
    author: "agent-xyz"
    timestamp: 1699999999
    krajicek_thresholds: {ρ_T: 10000, ...}
  ip_rights: "CC-BY-4.0"         # Licença automática
  mutation_signature: "ed25519..."
```

### 4.2 Q-ESCUDO: Oráculo Contra-Factual

Validação de segurança física (ex: NR-12 máquinas industriais):

```
Seguro(ação) ⟺ ∀ mundo_possível w: Counterfactual(w, ação) ⊨ NR-12
```

- Gera cenários "e se?" quanticamente aprimorados
- Diagnóstico preditivo antes de despachar para hardware real

---

## Fluxo de Dados Entre Camadas

```
┌─────────────────────────────────────────────────────────────────────┐
│                        ESTÍMULO EXÓGENO                             │
│                   (Dados via Origo Overlay)                         │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│  L2: ORIGO OVERLAY                                                  │
│  • Recebe AST via overlay declarativo                               │
│  • Calcula hash alfa-estrutural                                     │
│  • Passa para validação                                             │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│  L1: VALIDADOR KRAJICEK                                             │
│  • Verifica complexidade de prova ≤ ρ_T                             │
│  • Se FALHA: Rejeita (GluingFailure)                                │
│  • Se PASSA: Computa ΔT (gap topológico)                            │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│  L3: METABOLISMO (MUTARIC)                                          │
│  • ℛₗ = f(ΔT, ρ_T, B_T, C_T, R_T, T)                                │
│  • Classifica estado: HOMEOSTASIS / EXPLORATION / ANXIETY           │
│  • Se ANXIETY: Sinal para contração                                 │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│  L2: QUANTUM LOCAL PACKING                                          │
│  • Otimiza distribuição da mutação aprovada                         │
│  • Empacota com dNFT identity                                       │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────┐
│  L4: GOVERNANÇA                                                     │
│  • Emite dNFT para nova mutação                                     │
│  • Registra IP on-chain                                             │
│  • Q-ESCUDO valida segurança física se aplicável                   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Invariantes do Sistema

1. **Nenhuma mutação não-validada executa** — Validação L1 é gate obrigatório
2. **Conservação de identidade** — Hash alfa-estrutural preserva semântica através de renomeações
3. **Terminabilidade metabólica** — ℛₗ converge (γ > 0 na equação diferencial)
4. **Rastreabilidade total** — dNFT lineage permite auditoria completa
5. **Equivariância semântica** — Coordenadas do hipercubo têm significado fixo

---

## Referências

- [Formalização LaTeX Completa](../formal/tex/mutacore_architecture.tex)
- [Acoplamento Termodinâmico](../formal/tex/landauer_coupling.tex)
- [Limiares de Krajicek](../formal/tex/krajicek_thresholds.tex)
- [Topologia ΔT](../formal/tex/delta_t_topology.tex)
- [Código Rust: origo-overlay](../src/rust/origo-overlay/)
- [Simulações Python](../src/python/)
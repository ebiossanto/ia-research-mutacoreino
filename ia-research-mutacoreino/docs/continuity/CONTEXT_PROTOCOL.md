# Protocolo de Contexto — MutaCore / Reino Digital

> **Especificação técnica para transferência de estado lógico entre IAs**
> **Versão:** 4.0.26

---

## 1. Modelo de Estado

O contexto do MutaCore é representado como uma tupla imutável:

```
Context = (Γ, Σ, ℳ, 𝒦, ℋ, 𝒢)
```

Onde:

| Símbolo | Componente | Tipo | Descrição |
|---------|------------|------|-----------|
| **Γ** | Gamma | `AstContext` | Estado de canonização α (rename_map, counter, scope_stack) |
| **Σ** | Sigma | `ValidationState` | Estatísticas do validador (passed, failed, warnings, total_complexity) |
| **ℳ** | Mu | `MetabolicState` | Estado termodinâmico atual (ℛₗ, ΔT, estado metabólico, T) |
| **𝒦** | Kappa | `KrajicekThresholds` | Espectro ativo (ρ_T, B_T, C_T, R_T + metadata) |
| **ℋ** | Eta | `HodgeState` | Decomposição Hodge atual (autovalores, β_k, equivariance_score) |
| **𝒢** | Gamma | `GovernanceState` | dNFTs ativos, lineage, Q-ESCUDO status |

---

## 2. Serialização

### 2.1 Formato Canônico (JSON)

```json
{
  "version": "4.0.26",
  "timestamp": "2026-10-12T15:30:00Z",
  "session_id": "mutacore-20261012-001",
  "gamma": {
    "rename_map": {"x": "α_1", "y": "α_2"},
    "counter": 2,
    "scope_stack": [{"x": "α_1"}, {"y": "α_2"}]
  },
  "sigma": {
    "total_validated": 142,
    "passed": 138,
    "failed": 4,
    "warnings": 7,
    "total_complexity": 847291,
    "max_complexity_seen": 9847
  },
  "mu": {
    "delta_t_components": [12.3, 45.6, 7.8, 23.1],
    "delta_t_magnitude": 2847.23,
    "landauer_residue": 3.14e-18,
    "metabolic_state": "EXPLORATION",
    "temperature": 300.0,
    "energy_cost_per_bit": 2.87e-21
  },
  "kappa": {
    "rho_t": 10000,
    "b_t": 5000,
    "c_t": 2000,
    "r_t": 3000,
    "metadata": {
      "version": "1.0",
      "description": "Espectro padrão",
      "source": "Krajicek 1995"
    }
  },
  "eta": {
    "dimension": 6,
    "blocks": {
      "0": {"betti_number": 1, "spectral_gap": 2.0},
      "1": {"betti_number": 0, "min_eigenvalue": 2.0},
      "2": {"betti_number": 0, "min_eigenvalue": 4.0}
    },
    "total_harmonic_forms": 1,
    "equivariance_score": 0.94
  },
  "governance": {
    "active_dnfts": 47,
    "lineage_depth": 12,
    "qescudo_status": "STANDBY"
  }
}
```

### 2.2 Formato Binário (Postcard/bincode)

Para transferência de alta performance entre processos Rust:

```rust
// Serialização
let bytes = postcard::to_stdvec(&context).unwrap();

// Desserialização
let context: MutacoreContext = postcard::from_bytes(&bytes).unwrap();
```

---

## 3. Protocolo de Transferência

### 3.1 Handshake Inicial

```
IA_ORIGEM                    IA_DESTINO
    |                           |
    |--- CONTEXT_OFFER --------->|
    |   {version, hash, size}   |
    |                           |
    |<-- CONTEXT_ACCEPT ---------|
    |   {accepted_version}      |
    |                           |
    |--- CONTEXT_DATA ---------->|
    |   {serialized_context}    |
    |                           |
    |<-- CONTEXT_ACK ------------|
    |   {verified_hash}         |
    |                           |
```

### 3.2 Verificação de Integridade

```rust
// Hash do contexto para verificação
fn context_hash(ctx: &MutacoreContext) -> AlphaHash {
    let mut hasher = IncrementalHasher::new();
    hasher.add_hash(ctx.gamma_hash());
    hasher.add_hash(ctx.sigma_hash());
    hasher.add_hash(ctx.mu_hash());
    hasher.add_hash(ctx.kappa_hash());
    hasher.add_hash(ctx.eta_hash());
    hasher.add_hash(ctx.governance_hash());
    hasher.finalize()
}
```

### 3.3 Versionamento de Contexto

| Versão | Mudanças | Compatibilidade |
|--------|----------|-----------------|
| 4.0.x | Patch (stats, metadata) | Forward/Backward |
| 4.x.0 | Minor (novos campos opcionais) | Forward only |
| x.0.0 | Major (estrutura alterada) | Incompatível |

---

## 4. Isolamento de Contexto

### 4.1 Princípio de Não-Interferência

```
∀ sessões s1, s2: s1 ≠ s2 ⟹ Context(s1) ∩ Context(s2) = ∅
```

**Implementação:**
- Cada sessão usa diretório de trabalho isolado
- `AlphaContext` **nunca** compartilhado entre sessões
- `KrajicekThresholds` copiado (deep clone) por sessão
- Estado metabólico (`ℛₗ`, `ΔT`) resetado por sessão

### 4.2 Sandboxing de Execução

```rust
// Cada validação em contexto isolado
fn validate_in_sandbox(ast: AstNode, thresholds: KrajicekThresholds) -> ValidationResult {
    let mut validator = OrigoAstValidator::new(thresholds);
    let mut ctx = AlphaContext::new();
    
    // Canonização isolada
    let canonical = ctx.canonicalize(&ast);
    
    // Validação isolada
    validator.validate(&canonical)
}
```

---

## 5. Recuperação de Estado

### 5.1 Checkpoint Automático

```rust
// A cada N validações ou mudança de estado metabólico
const CHECKPOINT_INTERVAL: usize = 10;

fn maybe_checkpoint(&mut self, step: usize) {
    if step % CHECKPOINT_INTERVAL == 0 {
        self.save_checkpoint();
    }
}
```

### 5.2 Restore Procedure

```bash
# 1. Identifica checkpoint válido
ls -la checkpoints/*.ctx.json | tail -5

# 2. Valida integridade
python -c "
import json, sys
with open(sys.argv[1]) as f:
    ctx = json.load(f)
assert ctx['version'] == '4.0.26'
print('OK:', ctx['session_id'])
" checkpoints/latest.ctx.json

# 3. Restaura na nova sessão
# Carrega via CONTEXT_PROTOCOL handshake
```

---

## 6. Métricas de Qualidade do Contexto

| Métrica | Target | Alerta |
|---------|--------|--------|
| **Context Drift** | < 0.1% por 1000 steps | > 1% = investigar |
| **Validation Consistency** | 100% determinístico | Qualquer não-determinismo = bug |
| **Metabolic Continuity** | ℛₗ contínuo (sem saltos > 10x) | Salto = reinicialização |
| **Threshold Stability** | 𝒦 imutável durante sessão | Mudança = novo ADR |

---

## 7. Auditoria

Toda transferência de contexto gera log de auditoria:

```json
{
  "event": "CONTEXT_TRANSFER",
  "timestamp": "2026-10-12T15:30:00Z",
  "from_session": "mutacore-20261012-001",
  "to_session": "mutacore-20261012-002",
  "context_hash": "0xabc123...",
  "size_bytes": 12457,
  "duration_ms": 23,
  "verified": true
}
```

---

## 8. Exceções e Casos Especiais

### 8.1 Contexto Corrompido

```rust
// Detecção
if !verify_context_integrity(&ctx) {
    // Fallback: reconstrói estado mínimo seguro
    return MutacoreContext::safe_default();
}
```

### 8.2 Migração de Versão

```rust
fn migrate_context_v3_to_v4(old: ContextV3) -> ContextV4 {
    ContextV4 {
        // ... campos compatíveis
        eta: HodgeState::default(),  // Novo em v4
        governance: GovernanceState::default(),
        // ...
    }
}
```

---

## 9. Testes de Protocolo

```rust
#[test]
fn context_roundtrip_preserves_state() {
    let original = create_test_context();
    let bytes = postcard::to_stdvec(&original).unwrap();
    let restored: MutacoreContext = postcard::from_bytes(&bytes).unwrap();
    
    assert_eq!(original.gamma, restored.gamma);
    assert_eq!(original.sigma, restored.sigma);
    assert_eq!(original.mu, restored.mu);
    assert_eq!(original.kappa, restored.kappa);
    assert_eq!(original.eta, restored.eta);
    assert_eq!(original.governance, restored.governance);
}

#[test]
fn context_isolation_between_sessions() {
    let mut ctx1 = MutacoreContext::new();
    let mut ctx2 = MutacoreContext::new();
    
    // Modifica ctx1
    ctx1.mu.landauer_residue = 1e-15;
    
    // ctx2 inalterado
    assert_eq!(ctx2.mu.landauer_residue, 0.0);
}
```

---

**FIM DO PROTOCOLO**
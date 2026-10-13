# ADR 004: dNFT Containers for Agent Governance

## Status
Proposto (Planejado para v0.5.0+)

## Contexto
O MutaCore gera agentes, mutações e módulos continuamente. Necessita:
- Identidade única e rastreável para cada entidade
- Registro de propriedade intelectual (authorship, lineage)
- Licenciamento automático e programável
- Monetização descentralizada de capacidades
- Auditoria completa de evolução

Soluções tradicionais (PKI, databases centrais) não oferecem:
- Imutabilidade de lineage
- Composabilidade programável (smart contracts)
- Interoperabilidade cross-chain/ecossistema

## Decisão
Usar **Dynamic NFTs (dNFTs)** como containers de governança:

### Estrutura do dNFT Container
```yaml
dNFT:
  token_id: "mutacore-{genome_hash[:8]}-{timestamp}"
  metadata:
    # Identidade Genômica
    genome_hash: "0x..."              # SHA-256 da AST canônica
    genome_version: 1
    lineage: ["parent_hash", ...]     # Árvore evolutiva completa
    
    # Metadados Imutáveis (IPFS + on-chain)
    author_agent: "agent-uuid"
    created_at: 1699999999
    krajicek_thresholds: {ρ_T: 10000, B_T: 5000, C_T: 2000, R_T: 3000}
    source_code_ipfs: "Qm..."
    
    # Direitos e Licenciamento
    license: "CC-BY-4.0"              # Ou custom via smart contract
    commercial_rights: true
    attribution_required: true
    
    # Estado Dinâmico (atualizável via oracle)
    mutation_count: 0
    deployment_count: 0
    reputation_score: 1.0
    last_audit: 1699999999
```

### Lifecycle
1. **Mint**: Validação Krajicek PASS → mint dNFT com genome_hash
2. **Evolve**: Nova mutação aprovada → novo dNFT filho (lineage append)
3. **Deploy**: Quantum Packing assina assignment com dNFT reference
4. **Audit**: Q-ESCUDO verifica segurança → atualiza reputation_score
5. **Monetize**: Smart contract licencia uso → revenue split automático

## Consequências

### Positivas
- **Rastreabilidade total**: genome_hash → lineage → author
- **IP automático**: timestamp on-chain = prior art proof
- **Composabilidade**: dNFTs como legos (compose agents via lineage)
- **Descentralizado**: não depende de authority central
- **Programável**: licenciamento, royalties, access control via smart contracts

### Negativas
- Complexidade blockchain (gas, finality, oracles)
- Custo de mint/transfer (mitigado: L2/rollups, batch minting)
- Dependência de oracle para reputation/audit (Q-ESCUDO)

### Riscos
- Regulatory uncertainty (NFTs as securities?)
- Oracle manipulation (Q-ESCUDO compromise)
- Metadata rot (IPFS pinning strategy required)

## Referências
- EIP-4906 (ERC-721 Metadata Update)
- EIP-5192 (Soulbound Tokens) — para identidades não-transferíveis
- `formal/tex/mutacore_architecture.tex` Seção 6
- Q-ESCUDO NR-12 specification (pending)
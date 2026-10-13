# ADR 002: Alpha-Structural AST for Transport

## Status
Aceito

## Contexto
O transporte de código entre nós distribuídos requer:
- Imunidade a renomeação de variáveis (α-equivalence)
- Verificação de integridade sem recompilação
- Comparação estrutural eficiente (O(1) via hash)
- Suporte a mutação de código (AST transformadas)

Abordagens alternativas:
1. **Source text transport** — frágil, formatação varia
2. **Bytecode/WASM transport** — perde estrutura semântica, hard to validate
3. **Named AST transport** — sensível a shadowing/renaming
4. **Alpha-structural AST (De Bruijn indices)** — canônico, isomórfico

## Decisão
Implementar **Alpha-Structural AST** com:
- Canonização via `AlphaContext` (conversão para índices De Bruijn)
- Hash estrutural `StructuralHasher` invariante sob α-conversão
- `AstNode` enum com `KrajicekGenerator` embedded para validação inline
- Merkle trees para provas de inclusão em ASTs grandes

## Consequências

### Positivas
- `hash(λx.x) == hash(λy.y)` **sempre** — garantia matemática
- Validação de limiares Krajicek **antes** de execução/transporte
- Diff semântico via hash comparison (não textual)
- Serialização compacta (bincode/postcard)
- Merkle proofs para verificação parcial

### Negativas
- Overhead de canonização (O(n) com cache)
- Complexidade de implementação (shadowing, free vars)
- Debugging menos intuitivo (nomes canônicos α_1, α_2)

### Riscos
- Bugs em canonização = hash collision semântica (mitigado: property tests exaustivos)
- Performance em ASTs gigantes (mitigado: incremental hashing, caching)

## Referências
- [De Bruijn Indices](https://en.wikipedia.org/wiki/De_Bruijn_index)
- [Alpha Equivalence](https://en.wikipedia.org/wiki/Alpha_equivalence)
- `src/rust/origo-overlay/src/alpha_context.rs`
- `src/rust/origo-overlay/src/hash.rs`
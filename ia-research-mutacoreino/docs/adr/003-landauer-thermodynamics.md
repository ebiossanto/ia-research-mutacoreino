# ADR 003: Landauer Thermodynamics as Metabolic Regulation

## Status
Aceito

## Contexto
Sistemas de IA distribuídos tradicionais ignoram custo energético da computação. O MutaCore precisa de:
- Feedback termodinâmico em tempo real
- Métrica de "estresse" computacional para regulação adaptativa
- Conexão rigorosa entre teoria da informação e termodinâmica
- Prevenção de exaustão energética (cloud/borda)

O Princípio de Landauer: $E_{min} = k_B T \ln 2$ por bit apagado.

## Decisão
Implementar **Metabolismo Termodinâmico (MUTARIC)** com:

1. **Gap Topológico Narrativo (ΔT)** como entrada — distância semântica organismo/ambiente
2. **Resíduo de Landauer (ℛₗ)** como saída — dissipação energética mínima
3. **Equação Fundamental**: $\RL = (\sum \Delta T_i^2) \cdot \exp(\frac{\rho_T - B_T}{C_T \cdot R_T}) \cdot k_B T \ln 2$
4. **Três Estados Metabólicos**: HOMEOSTASIS / EXPLORATION / ANXIETY
5. **Loop de Controle**: $\frac{d\RL}{dt} = -\gamma \RL + \alpha \|\DeltaT\|^2 \kappa(\mathcal{K}) + \xi(t)$

## Consequências

### Positivas
- Fundamentação física rigorosa (não heurística)
- Auto-regulação: alta ℛₗ → contração automática
- Otimização de custo cloud/borda via minimização de ℛₗ
- "Emoção computacional" emergente de princípios físicos
- Métrica unificada para alocação de recursos (Quantum Packing)

### Negativas
- Parâmetros ($\gamma, \alpha, T$) requerem calibração empírica
- Complexidade adicional no loop de validação
- Dependência de temperatura física do hardware

### Riscos
- Modelo termodinâmico pode não capturar todos custos reais (mitigado: monitoramento contínuo + ajuste)
- Oscilação entre estados (mitigado: histerese nos limiares)

## Referências
- Landauer, R. (1961). "Irreversibility and Heat Generation in the Computing Process"
- Parrondo et al. (2015). "Thermodynamics of Information", Nature Physics
- `formal/tex/landauer_coupling.tex`
- `src/python/landauer_simulation.py`
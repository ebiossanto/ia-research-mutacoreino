# Checklist de Handoff — MutaCore / Reino Digital

> **Executar ANTES de encerrar QUALQUER sessão de IA**
> **Versão:** 4.0.26

---

## 📋 CHECKLIST OBRIGATÓRIO

### 1. Estado do Repositório
- [ ] `git status` — sem arquivos untracked/modificados não intencionais
- [ ] `git log -1 --oneline` — último commit documentado
- [ ] Branch correta: `git branch --show-current`
- [ ] Push feito: `git push origin <branch>`

### 2. Testes Passando
- [ ] `cargo test --workspace --lib` — Rust unit tests ✓
- [ ] `cargo test --workspace --test integration` — Rust integration ✓
- [ ] `cd src/python && pytest tests/ -v` — Python tests ✓
- [ ] `cargo test --workspace --proptest` — Property tests ✓ (se disponível)

### 3. Formalização LaTeX
- [ ] `cd formal/tex && latexmk -pdf mutacore_architecture.tex` — compila ✓
- [ ] `latexmk -pdf landauer_coupling.tex` — compila ✓
- [ ] `latexmk -pdf krajicek_thresholds.tex` — compila ✓
- [ ] `latexmk -pdf delta_t_topology.tex` — compila ✓
- [ ] Referências cruzadas resolvidas (roda 2x latexmk)

### 4. Documentação Atualizada
- [ ] `docs/architecture/ARCHITECTURE.md` reflete mudanças
- [ ] `docs/development/` guias atualizados se API mudou
- [ ] `docs/continuity/SESSION_STATE_TEMPLATE.md` preenchido
- [ ] ADR criado/atualizado em `docs/adr/` se decisão arquitetural

### 5. Mudanças de Limiares/Parâmetros Críticos
- [ ] `KrajicekThresholds` alterados? **Documente:**
  - Valor anterior → novo valor
  - Justificativa (empírica/teórica)
  - Impacto em validação/metabolismo
- [ ] Equação fundamental ℛₗ modificada? **Documente:**
  - Termo alterado
  - Nova forma matemática
  - Atualização correspondente em `landauer_coupling.tex` E `landauer_simulation.py`

### 6. Novos Arquivos / APIs Públicas
- [ ] Todo `pub fn/struct/enum` tem doc comment com exemplo
- [ ] `cargo doc --no-deps` gera sem warnings
- [ ] Python: type hints + docstrings Google-style em novas funções

### 7. Performance / Benchmarks
- [ ] `cargo bench --workspace` rodou sem regressão > 5%
- [ ] Novos benchmarks adicionados para hot paths
- [ ] Baseline atualizado se melhoria intencional

### 8. Segurança
- [ ] `cargo audit` — sem vulnerabilidades críticas
- [ ] `pip-audit` (Python) — sem vulnerabilidades
- [ ] Zero `unwrap()`/`expect()` em código novo de produção
- [ ] Input validation em todas as fronteiras públicas

### 9. Contexto de Continuidade
- [ ] `SESSION_STATE_TEMPLATE.md` preenchido completamente
- [ ] `HANDOFF_CHECKLIST.md` (este arquivo) preenchido
- [ ] Stash nomeado se WIP: `git stash push -m "WIP: descrição clara"`

### 10. Comunicação
- [ ] PR atualizado com descrição clara
- [ ] Labels corretos no GitHub (feat/fix/docs/refactor/breaking)
- [ ] Reviewers atribuídos se necessário
- [ ] Issue relacionada referenciada (`Refs: #XXX`)

---

## 📝 TEMPLATE DE PREENCHIMENTO

```markdown
# HANDOFF - [YYYY-MM-DD] - [IA_ID/NOME]

## 📍 Localização
- Branch: `__________________________`
- Commit: `__________________________` (`git rev-parse HEAD`)
- Diretório: `__________________________`

## ✅ Status dos Testes
- Rust unit: [ ] PASS / [ ] FAIL (detalhes: __________)
- Rust integration: [ ] PASS / [ ] FAIL
- Python: [ ] PASS / [ ] FAIL
- Property: [ ] PASS / [ ] FAIL
- LaTeX: [ ] PASS / [ ] FAIL

## 🔧 Mudanças Críticas
### Limiares Krajicek
- [ ] Inalterados
- [ ] Alterados: ρ_T: ___ → ___ | B_T: ___ → ___ | C_T: ___ → ___ | R_T: ___ → ___
- Justificativa: ________________________________________________

### Equação Termodinâmica
- [ ] Inalterada
- [ ] Modificada: ________________________________________________

### API Pública
- Novas funções: ________________________________________________
- Funções removidas/depreciadas: _________________________________

## 🎯 Próximos Passos (Prioridade)
1. [ ] ________________________________________________ (Arquivo: `__________________`)
2. [ ] ________________________________________________ (Arquivo: `__________________`)
3. [ ] ________________________________________________ (Arquivo: `__________________`)

## ⚠️ Bloqueadores / Riscos Identificados
- ________________________________________________
- Mitigação sugerida: ________________________________________________

## 🧠 Contexto Mental (Para Próxima IA)
> "Estava trabalhando em ________________________________________________
> Descobri que ________________________________________________
> O arquivo chave é `________________________________________________` linha ____
> A variável/parâmetro __________________________ precisa __________________________
> Decisão pendente: ________________________________________________"

## 📦 Stash / WIP
- [ ] Sem stash
- [ ] Stash criado: `git stash push -m "WIP: __________________________"`

## ✍️ Assinatura
IA: __________________________ | Timestamp: __________________________
```

---

## 🚨 SE NÃO PREENCHER: PRÓXIMA IA DEVE

1. **NÃO ASSUMIR** estado anterior
2. **EXECUTAR** baseline completo: `cargo test && pytest && latexmk`
3. **DOCUMENTAR** discrepâncias encontradas
4. **REPORTAR** no handoff da próxima sessão

---

**ESTE CHECKLIST É PARTE DO PROTOCOLO DE GOVERNANÇA DO MUTACORE. NÃO PULE.**
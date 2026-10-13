# Guia de Contribuição — MutaCore / Reino Digital

> **Bem-vindo!** Este documento orienta como contribuir para o ecossistema MutaCore.

---

## Código de Conduta

Ao participar, você concorda em manter um ambiente respeitoso, inclusivo e profissional. Discriminação, assédio ou comportamento tóxico não são tolerados.

---

## Como Contribuir

### 1. Reporte de Bugs

**Antes de abrir issue:**
- Verifique se já existe issue similar (busque por termos-chave)
- Tente reproduzir na versão `main` mais recente

**Template de Bug Report:**
```markdown
## Descrição
Breve resumo do problema.

## Passos para Reproduzir
1. ...
2. ...
3. ...

## Comportamento Esperado
O que deveria acontecer.

## Comportamento Real
O que acontece (logs, screenshots, stack traces).

## Ambiente
- OS: [ex: Ubuntu 22.04, Windows 11]
- Rust: `rustc --version`
- Python: `python --version`
- Commit: `git rev-parse HEAD`

## Contexto Adicional
Qualquer outra informação relevante.
```

### 2. Sugestão de Features

**Template de Feature Request:**
```markdown
## Problema
Que limitação ou necessidade motiva esta feature?

## Solução Proposta
Descrição da implementação desejada.

## Alternativas Consideradas
Outras abordagens avaliadas.

## Impacto
- Camadas afetadas: [L1/L2/L3/L4]
- Quebra de compatibilidade? [Sim/Não]
- Migração necessária? [Sim/Não]
```

### 3. Pull Requests

#### Fluxo Básico

```bash
# 1. Fork no GitHub
# 2. Clone seu fork
git clone https://github.com/SEU_USUARIO/ia-research-mutacoreino
cd ia-research-mutacoreino

# 3. Adicione upstream
git remote add upstream https://github.com/ebiossanto/ia-research-mutacoreino

# 4. Crie branch descritiva
git checkout -b feat/nova-funcionalidade-xyz

# 5. Desenvolva com commits semânticos
git commit -m "feat(core): adiciona quantum tunneling no packing

- Implementa quantum_tunneling_step() para escapar mínimos locais
- Adiciona tunneling_probability ao PackingConfig
- Atualiza benchmarks comparando com/sem tunneling

Refs: #42"

# 6. Push e abra PR
git push origin feat/nova-funcionalidade-xyz
# Abra PR no GitHub contra upstream/main
```

#### Padrão de Commits (Conventional Commits)

```
<tipo>(<escopo>): <descrição curta>

[corpo opcional]

[rodapé opcional]
```

**Tipos:**
- `feat`: Nova funcionalidade
- `fix`: Correção de bug
- `docs`: Documentação
- `refactor`: Refatoração sem mudança de comportamento
- `perf`: Melhoria de performance
- `test`: Adição/modificação de testes
- `chore`: Manutenção (deps, build, etc.)
- `formal`: Mudanças em formalização LaTeX

**Escopos:** `core`, `transport`, `metabolism`, `governance`, `python`, `docs`, `ci`

**Exemplos:**
```
feat(core): adiciona validação de gluing entre ASTs

fix(transport): corrige race condition no quantum packing

docs(architecture): atualiza diagrama de camadas L3

formal(landauer): corrige fator exponencial na equação fundamental

refactor(validator): extrai validação de Krajicek para trait

test(metabolism): adiciona property-based test para ΔT magnitude

chore(deps): atualiza nalgebra para 0.32
```

---

## Padrões de Desenvolvimento

### Rust

- Siga [RUST_GUIDELINES.md](RUST_GUIDELINES.md)
- `cargo fmt`, `cargo clippy`, `cargo test` devem passar
- Doc comments em todos os `pub` items
- Zero `unwrap()`/`expect()` em código de produção

### Python

- Siga [PYTHON_GUIDELINES.md](PYTHON_GUIDELINES.md)
- Type hints obrigatórios (`mypy --strict`)
- `ruff`, `black`, `pytest` devem passar
- Seeds fixos para reprodutibilidade

### LaTeX (Formalização)

- Compila sem erros: `latexmk -pdf arquivo.tex`
- Referências cruzadas funcionando (`\ref{}`, `\eqref{}`)
- Notação consistente com `mutacore_architecture.tex` (Apêndice A)
- Novos símbolos adicionados ao glossário

### Documentação

- Markdown com formatação consistente
- Diagramas em Mermaid quando aplicável
- Exemplos de código testáveis
- Português (pt-BR) como língua padrão

---

## Revisão de Código (Code Review)

### Checklist do Revisor

- [ ] **Corretude:** Lógica implementa o especificado?
- [ ] **Segurança:** Validação de entrada? Limiares respeitados?
- [ ] **Performance:** Hot paths otimizados? Allocations desnecessárias?
- [ ] **Testes:** Cobertura adequada? Property tests para invariantes?
- [ ] **Documentação:** Doc comments? README/ADR atualizados?
- [ ] **Formal:** LaTeX compila? Notação consistente?
- [ ] **Breaking Changes:** Marcado? Migração documentada?

### Critérios de Aprovação

| Tipo | Aprovações Necessárias |
|------|------------------------|
| Bugfix / Docs / Chore | 1 reviewer |
| Feature / Refactor | 2 reviewers |
| Breaking Change / Arch Decision | 2 reviewers + ADR |
| Formal/Math Changes | 1 reviewer + validação matemática |

---

## Decisões Arquiteturais (ADR)

Para mudanças que afetam a arquitetura global:

1. Crie arquivo em `docs/adr/NNN-titulo-kebab-case.md`
2. Use template ADR (ver abaixo)
3. Inclua no PR como parte da mudança
4. Discuta no PR antes de merge

**Template ADR:**
```markdown
# ADR NNN: Título da Decisão

## Status
[Proposto | Aceito | Rejeitado | Obsoleto | Substituído por ADR XXX]

## Contexto
Qual o problema que motiva esta decisão?

## Decisão
O que foi decidido.

## Consequências
### Positivas
- ...

### Negativas
- ...

### Riscos
- ...

## Alternativas Consideradas
1. ...
2. ...

## Referências
- Links para issues, PRs, papers, etc.
```

---

## Release Process

### Versionamento (SemVer)

```
MAJOR.MINOR.PATCH

MAJOR: Breaking changes (API, arquitetura, limiares Krajicek)
MINOR: Features compatíveis
PATCH: Bugfixes, docs, performance
```

### Checklist de Release

- [ ] Todos os CI passam
- [ ] `CHANGELOG.md` atualizado (formato Keep a Changelog)
- [ ] Versão bumped: `Cargo.toml` (workspace) + `pyproject.toml`
- [ ] LaTeX compila sem warnings
- [ ] Docs geradas: `cargo doc --no-deps`
- [ ] Tag anotada: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] GitHub Release com artifacts (binários, PDFs)
- [ ] Anúncio no Discord/comunidade

---

## Áreas de Contribuição Prioritárias

| Área | Descrição | Habilidades |
|------|-----------|-------------|
| **Core Rust** | Validator, AlphaContext, Hash, Packing | Rust, Formal Methods |
| **Simulações Python** | Landauer, Hodge, SPECTRUM-1024 | Scientific Python, Math |
| **Formalização LaTeX** | Teoremas, provas, notação | LaTeX, Matemática, Lógica |
| **Documentação** | Guias, tutoriais, diagramas | Technical Writing |
| **CI/CD** | Workflows, benchmarks, security | GitHub Actions, DevOps |
| **WASM/Sandbox** | Execução isolada, PyO3 | WebAssembly, FFI |

---

## Comunicação

- **Issues:** GitHub Issues (bugs, features, questions)
- **Discussões:** GitHub Discussions (design, RFCs)
- **Chat:** Discord do Reino Digital (link no perfil GitHub)
- **Email:** ebiossanto@github.com (questões sensíveis)

---

## Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob **Apache License 2.0** (mesma licença do projeto).

---

## Reconhecimento

Contribuidores são listados em:
- `AUTHORS.md` (atualizado a cada release)
- GitHub Contributors graph
- Seção "Agradecimentos" na documentação formal

---

**Obrigado por contribuir para o MutaCore!** 🧬🤖

*Construindo juntos os alicerces de uma AGI Distribuída, Confiável e Biologicamente Inspirada.*
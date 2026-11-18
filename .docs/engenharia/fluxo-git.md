# Fluxo Git — Oak Note

Padrão operacional de branches, commits e integração na `main`.

A branch `main` é protegida por ruleset no GitHub. **Não há commit nem push direto na `main`.** Toda mudança entra por pull request.

## Princípios

1. `main` só recebe mudanças via PR aprovado/mesclado.
2. Trabalho rastreável a issue do GitHub sempre que houver demanda.
3. Commits em Conventional Commits, em inglês, subject ≤ 100 caracteres.
4. Sem `Co-authored-by` (nem trailers de co-autor injetados pela IDE).
5. Código e docs canônicos no checkout vencem wiki, planos Cursor e memória.

## Branches

Criar a partir de `main` atualizada:

```bash
git fetch origin
git switch main
git pull --ff-only origin main
git switch -c <tipo>/<issue>-<slug>
```

### Nome

Formato: `<tipo>/<número-da-issue>-<slug-curto>`

Exemplos:

- `feat/11-h01-reconhecer-markdown`
- `chore/8-t06-rust-gtk-ci`
- `docs/fluxo-git`
- `fix/18-autosave-durabilidade`

Tipos usuais: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `research`.

Sem issue (ex.: docs internas pontuais): `docs/<slug>` ou `chore/<slug>`.

### Regras

- Não reutilizar branch já mesclada.
- Não fazer force-push em `main`.
- Force-push na branch de trabalho só se necessário e nunca após revisão compartilhada, salvo acordo explícito.
- Preferir uma issue por branch/PR. Fatias maiores exigem justificativa no corpo do PR.

## Commits

- Conventional Commits em inglês.
- Subject ≤ 100 caracteres; mensagem só no subject (sem body obrigatório).
- Escopos úteis quando claros: `docs(mvp): …`, `chore(ci): …`.
- Referenciar a issue no subject ou no corpo do PR (`#11`), não inventar números.
- Um commit lógico por intenção; evitar “WIP” na `main` (WIP fica na branch até limpar).

## Pull requests

1. Push da branch: `git push -u origin HEAD`
2. Abrir PR com base `main` (`gh pr create`).
3. Título no mesmo padrão Conventional Commits.
4. Corpo mínimo:
   - objetivo / aceite
   - issue(s) (`Closes #N` quando a PR conclui a issue)
   - como validar
   - limitações
5. Aguardar checks aplicáveis (quando existirem) e revisão.
6. Mesclar só pela interface/CLI do GitHub (merge commit ou squash conforme a política do repositório no momento do merge). Não fazer merge local direto em `main` para “furar” o ruleset.

Após o merge:

```bash
git switch main
git pull --ff-only origin main
git branch -d <branch-local>   # se já mesclada
git push origin --delete <branch-remota>  # se ainda existir
```

## Onde vive o trabalho

| Camada | Caminho | Versionar? |
|---|---|---|
| Docs oficiais | `.docs/` | Sim |
| Trabalho em curso / evidências de demanda | `.work/demands/` | Sim (espelho operacional) |
| Instruções de agentes | `.ai/` (+ loaders `.cursor` via symlink) | Sim (canônico em `.ai/`) |
| Planos locais da IDE | `.cursor/plans/` | Não (ignorado) |

Espelho M1: [`.work/demands/README-m1.md`](../../.work/demands/README-m1.md).

## Proteção da `main`

- Ruleset GitHub: **Protect main** (branch `main`).
- Efeito prático: push/commit direto na `main` bloqueados; integração apenas por PR.
- Exceções administrativas (bypass do ruleset) não fazem parte do fluxo normal e exigem decisão explícita do mantenedor.

## Ponteiros no repositório

- README da raiz e [`.docs/README.md`](../README.md)
- [`.ai/README.md`](../../.ai/README.md) (instruções canônicas de agentes; `AGENTS.md` só aponta para ele)
- [`.ai/rules/repository-standard.md`](../../.ai/rules/repository-standard.md)

## Relação com o backlog

- Issues e milestones no GitHub são a fila oficial do MVP.
- Demandas ativas podem ter pasta em `.work/demands/` com plano e evidências.
- Implementação segue a ordem de dependências das issues (M1 → M2 → …).

## Checklist rápido

- [ ] Branch criada a partir de `main` atualizada
- [ ] Nome `<tipo>/<issue>-<slug>`
- [ ] Commits Conventional Commits, inglês, ≤ 100, sem co-autor
- [ ] PR para `main` com issue e validação
- [ ] Sem push direto na `main`

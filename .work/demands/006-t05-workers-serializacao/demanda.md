# T05 — Validar workers, serialização e cancelamento

- **Issue:** [#6](https://github.com/RafaelCarvalho89/oak-note/issues/6)
- **Milestone:** M1 — Fundação
- **Labels:** research, priority: high, area: platform
- **Status espelho:** planejada (não iniciada)
- **Sprint sugerida:** Sprint M1-B. Depende de T01 (#3).

## Objetivo / critérios de aceitação

- [ ] Validar fila limitada, serialização por nota/recurso, prioridade de autosave, retorno ao contexto GTK e cancelamento cooperativo. Documentar limites observados e reconciliação de resultados incertos sem fixar previamente debounce ou pool.

## Dependências

T01 (#3)

## Plano de trabalho

1. Validar modelo de fila limitada, serialização por nota/recurso e prioridade de autosave (DT-16).
2. Exercitar retorno ao contexto GTK e cancelamento cooperativo.
3. Documentar limites observados e reconciliação de resultados incertos sem fixar debounce/pool a priori.

## Testes / verificação (da issue)

Ver seção correspondente em https://github.com/RafaelCarvalho89/oak-note/issues/6.

## Evidências

Pasta: [`evidencias/`](evidencias/)

Checklist (preencher na implementação):

- [ ] Evidências anexadas ou linkadas
- [ ] Limitações registradas
- [ ] Decisões pendentes listadas (se research)
- [ ] Documentação afetada identificada

## Definition of Done

Conforme a issue #6: critérios atendidos, evidências e limitações documentadas, testes/CI aplicáveis aprovados.

## Referências

- [PREP-01](../../../.docs/mvp/product-specification.md)
- [PREP-02](../../../.docs/mvp/technical-specification.md)
- Issue: https://github.com/RafaelCarvalho89/oak-note/issues/6

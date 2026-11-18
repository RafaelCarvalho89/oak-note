# T07 — Preparar gettext e esquema GSettings v1

- **Issue:** [#7](https://github.com/RafaelCarvalho89/oak-note/issues/7)
- **Milestone:** M1 — Fundação
- **Labels:** chore, priority: high, area: platform
- **Status espelho:** planejada (não iniciada)
- **Sprint sugerida:** Sprint M1-C. Depende de T06 (#8).

## Objetivo / critérios de aceitação

- [ ] Preparar gettext pt-BR/en-US e esquema único GSettings com as dez chaves tipadas, defaults, inicialização de 0 para 1, validação por chave e proteção contra versão futura. Não inventar migração histórica.

## Dependências

T06 (#8)

## Plano de trabalho

1. Definir esquema GSettings v1 com as dez chaves tipadas e defaults.
2. Preparar gettext pt-BR/en-US.
3. Cobrir init 0→1, validação por chave e proteção contra versão futura; sem inventar migração histórica.

## Testes / verificação (da issue)

Ver seção correspondente em https://github.com/RafaelCarvalho89/oak-note/issues/7.

## Evidências

Pasta: [`evidencias/`](evidencias/)

Checklist (preencher na implementação):

- [ ] Evidências anexadas ou linkadas
- [ ] Limitações registradas
- [ ] Decisões pendentes listadas (se research)
- [ ] Documentação afetada identificada

## Definition of Done

Conforme a issue #7: critérios atendidos, evidências e limitações documentadas, testes/CI aplicáveis aprovados.

## Referências

- [PREP-01](../../../.docs/mvp/product-specification.md)
- [PREP-02](../../../.docs/mvp/technical-specification.md)
- Issue: https://github.com/RafaelCarvalho89/oak-note/issues/7

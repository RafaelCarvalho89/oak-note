# T02 — Validar capacidades do Flatpak e do portal

- **Issue:** [#4](https://github.com/RafaelCarvalho89/oak-note/issues/4)
- **Milestone:** M1 — Fundação
- **Labels:** research, priority: critical, area: storage
- **Status espelho:** planejada (não iniciada)
- **Sprint sugerida:** Sprint M1-B. Depende de T01 (#3).

## Objetivo / critérios de aceitação

- [ ] Testar Documentos e pasta alternativa via portal sem overrides amplos; registrar matriz de listagem, leitura, criação, gravação/substituição segura, sync disponível, renomeação, lixeira e persistência/revogação do acesso. Uma capacidade obrigatória não comprovada impede ativação; não ampliar permissões automaticamente.

## Dependências

T01 (#3)

## Plano de trabalho

1. Com base no runtime validado em T01, montar matriz de capacidades Documentos + pasta alternativa via portal.
2. Testar listagem, leitura, criação, gravação/substituição, sync, renomeação, lixeira, persistência/revogação.
3. Registrar o que impede ativação se capacidade obrigatória falhar; sem ampliar permissões automaticamente.

## Testes / verificação (da issue)

Ver seção correspondente em https://github.com/RafaelCarvalho89/oak-note/issues/4.

## Evidências

Pasta: [`evidencias/`](evidencias/)

Checklist (preencher na implementação):

- [ ] Evidências anexadas ou linkadas
- [ ] Limitações registradas
- [ ] Decisões pendentes listadas (se research)
- [ ] Documentação afetada identificada

## Definition of Done

Conforme a issue #4: critérios atendidos, evidências e limitações documentadas, testes/CI aplicáveis aprovados.

## Referências

- [PREP-01](../../../.docs/mvp/product-specification.md)
- [PREP-02](../../../.docs/mvp/technical-specification.md)
- Issue: https://github.com/RafaelCarvalho89/oak-note/issues/4

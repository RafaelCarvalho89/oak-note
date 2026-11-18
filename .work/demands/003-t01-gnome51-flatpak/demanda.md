# T01 — Validar GNOME 51 e Rust stable no Flatpak

- **Issue:** [#3](https://github.com/RafaelCarvalho89/oak-note/issues/3)
- **Milestone:** M1 — Fundação
- **Labels:** research, priority: critical, area: platform
- **Status espelho:** planejada (não iniciada)
- **Sprint sugerida:** Sprint M1-A (sem dependências). Pré-requisito de T02, T05 e T06.

## Objetivo / critérios de aceitação

- [ ] Build, instalação e abertura do Flatpak são exercitados no ambiente-alvo.
- [ ] Versões efetivas de Rust, Cargo, SDK, extensão e dependências relevantes são registradas.
- [ ] Incompatibilidades e limitações são registradas; uma falha impede a entrega dependente. Não presumir builds binariamente reproduzíveis.

## Dependências

Nenhuma.

## Plano de trabalho

1. Instalar org.gnome.Sdk//51, Platform e extensão Rust stable via Flatpak.
2. Exercitar build/install/open mínimo no runtime 51.
3. Registrar versões efetivas (Rust, Cargo, SDK, extensão, GTK/libadwaita).
4. Documentar incompatibilidades e divergência do host (Ubuntu 24.04 / GNOME 46) vs referência Debian/GNOME.

## Testes / verificação (da issue)

Ver seção correspondente em https://github.com/RafaelCarvalho89/oak-note/issues/3.

## Evidências

Pasta: [`evidencias/`](evidencias/)

Checklist (preencher na implementação):

- [ ] Evidências anexadas ou linkadas
- [ ] Limitações registradas
- [ ] Decisões pendentes listadas (se research)
- [ ] Documentação afetada identificada

## Definition of Done

Conforme a issue #3: critérios atendidos, evidências e limitações documentadas, testes/CI aplicáveis aprovados.

## Referências

- [PREP-01](../../../.docs/mvp/product-specification.md)
- [PREP-02](../../../.docs/mvp/technical-specification.md)
- Issue: https://github.com/RafaelCarvalho89/oak-note/issues/3

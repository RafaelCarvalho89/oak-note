# Plano de experimentos

Resultados pendentes não constituem validação. Registre ambiente, versões, data e evidências ao executar cada procedimento.

## POC-00 — Bootstrap

- **Objetivo e hipótese:** validar a toolchain e as dependências; uma aplicação GTK4/libadwaita mínima deve abrir no GNOME/Wayland.
- **Procedimento:** instalar os pré-requisitos do README, executar as verificações documentadas e `cargo run`; editar o texto, redimensionar e fechar a janela em uma sessão GNOME/Wayland.
- **Critérios de aprovação:** verificações sem erros; janela nativa abre, aceita edição, redimensiona e fecha normalmente.
- **Evidências esperadas:** versões da toolchain e bibliotecas, saída dos comandos e registro da sessão gráfica.
- **Resultado observado:** implementação inicial criada. Ambiente de investigação: Ubuntu 24.04.5, Rust 1.100.0 e Cargo 1.100.0; `pkg-config` não encontrou GTK4 nem libadwaita. `cargo fmt --all -- --check` passou; `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` e `cargo run` falharam ao localizar bibliotecas nativas (GTK4 e Graphene). Libadwaita também não foi encontrada pelo `pkg-config`.
- **Limitações e conclusão:** validação gráfica GNOME/Wayland pendente; a presença de variáveis de sessão não comprova a abertura da janela. Conclusão pendente.

## POC-01 — Transparência seletiva

- **Objetivo e hipótese:** investigar GTK CSS/RGBA; pode ser possível tornar o fundo translúcido mantendo texto e controles opacos.
- **Procedimento:** experimentar estilos de fundo RGBA em diferentes níveis e comparar texto e controles em GNOME/Wayland.
- **Critérios de aprovação:** níveis de transparência distinguíveis e texto e controles legíveis e opacos, sem efeitos indesejados relevantes.
- **Evidências esperadas:** versões, configurações CSS, capturas e observações por nível.
- **Resultado observado:** controle experimental e testes de conversão implementados; compilação e testes aprovados em Ubuntu. A execução não abriu o display. Detalhes em [resultados da POC-01](poc-01-results.md).
- **Limitações e conclusão:** hipótese pendente de validação visual no Debian/GNOME/Wayland; o comportamento depende do compositor e da integração com libadwaita.

## POC-02 — Sobreposição manual

- **Objetivo e hipótese:** investigar se o GNOME/Wayland oferece uma forma manual de manter a janela acima das demais.
- **Procedimento:** testar as opções disponíveis no menu da janela e documentar passos, ambiente e comportamento ao trocar de aplicativos.
- **Critérios de aprovação:** procedimento reproduzível e limitações documentadas, inclusive caso a opção não exista.
- **Evidências esperadas:** versões, passos, capturas e observações.
- **Resultado observado:** inspeção e verificações automáticas concluídas em Ubuntu; a tentativa de execução Wayland não abriu o display. Os cenários manuais estão pendentes; consulte [resultados da POC-02](poc-02-results.md).
- **Limitações e conclusão:** H02 pendente de evidências no Debian/GNOME/Wayland; não se presume controle direto da ordem das janelas pelo GTK4.

## POC-03 — Aparência e comportamento

- **Objetivo e hipótese:** investigar modos claro/escuro, redimensionamento e legibilidade; estilos personalizados podem interagir com libadwaita.
- **Procedimento:** alternar aparência do sistema, redimensionar a janela e comparar a interface com e sem CSS experimental.
- **Critérios de aprovação:** texto e controles legíveis, janela utilizável e interações com CSS documentadas em ambos os modos.
- **Evidências esperadas:** versões, configurações, capturas e observações.
- **Resultado observado:** pendente.
- **Limitações e conclusão:** pendentes.

## POC-04 — Flatpak

- **Objetivo e hipótese:** avaliar empacotamento e execução em sandbox; o comportamento pode diferir da execução nativa.
- **Procedimento:** preparar um pacote experimental, executar no sandbox e comparar a janela e os recursos disponíveis com a execução nativa.
- **Critérios de aprovação:** aplicação inicia no sandbox e diferenças e permissões necessárias ficam documentadas.
- **Evidências esperadas:** manifesto experimental, versões, comandos, permissões e registros comparativos.
- **Resultado observado:** pendente.
- **Limitações e conclusão:** pendentes.

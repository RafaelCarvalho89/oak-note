# Síntese das POCs Oak Note

## Objetivo e estado

O laboratório independente investigou Rust, GTK4/libadwaita e comportamentos necessários ao planejamento do MVP em Debian/GNOME/Wayland. **A investigação geral não está concluída com sucesso:** hipóteses críticas gráficas e de empacotamento permanecem pendentes.

| POC | Implementação e hipótese | Teste e evidência | Estado |
| --- | --- | --- | --- |
| 00 | Janela GTK4/libadwaita e editor experimental; bootstrap nativo | `cargo check`, Clippy e testes passaram em Ubuntu; abertura da janela não observada | Validação gráfica pendente |
| 01 | CSS RGBA e slider; H01: transparência seletiva | Testes de conversão passaram; display inacessível | H01 pendente |
| 02 | H02: sobreposição manual pelo compositor | Roteiro e pesquisa documentados; sem teste de janela ou menu | H02 pendente |
| 03 | Seletor Sistema/Claro/Escuro; H03-A temas, B transparência, C dimensões e D usabilidade | Código compilado; nenhuma avaliação visual ou de teclado | H03-A–D pendentes |
| 04 | Manifesto experimental, recursos e fontes Cargo; H04-A build offline, B execução, C compatibilidade, D isolamento, E bundle | Metadados e checksums validados; builder ausente, sem build/instalação/bundle | H04-A–E pendentes |

Detalhes: [plano](poc-plan.md), [POC-01](poc-01-results.md), [POC-02](poc-02-results.md), [POC-03](poc-03-results.md) e [POC-04](poc-04-results.md). A disponibilidade de Wayland em variáveis de ambiente não comprovou acesso ao display. Nenhuma evidência gráfica foi produzida.

## Tecnologias e decisões candidatas

No host Ubuntu 24.04.5 foram observados Rust/Cargo 1.100.0, GTK 4.14.5, libadwaita 1.5.0 e GNOME Shell 46.0; o lockfile usa gtk4-rs 0.9.7 e libadwaita-rs 0.7.2. A configuração Flatpak candidata usa GNOME 50 e extensão Rust 25.08; versão do compilador da extensão e compatibilidade no SDK permanecem desconhecidas.

Para o MVP, vale continuar avaliando libadwaita e `AdwStyleManager`, CSS de fundo restrito e a opção manual do compositor, sem tratá-los como validados. Evitar depender de controle automático da ordem de janelas e de permissões amplas de filesystem ou rede antes de demonstrar necessidade. Para futura exportação, investigar portais de arquivos.

## Riscos e próximos experimentos

Validar primeiro abertura nativa, transparência, contraste, foco, modo Sistema e dimensões compactas no Debian/GNOME/Wayland; registrar a sobreposição manual e suas limitações. Depois instalar os pré-requisitos Flatpak autorizados, executar build no SDK e comparar com a linha de base nativa, incluindo sandbox e bundle. Persistência, arquitetura definitiva e publicação não foram investigadas aqui. Somente evidências desses testes devem orientar decisões finais sobre o MVP.

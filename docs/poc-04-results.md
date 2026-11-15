# POC-04 — Viabilidade Flatpak

## 1. Identificação

Objetivo: avaliar compilação offline, execução isolada, compatibilidade e distribuição local da aplicação experimental. Hipóteses: **H04-A** compilar com SDK Flatpak e Cargo offline; **H04-B** executar no Debian/GNOME/Wayland; **H04-C** preservar comportamentos nativos validados; **H04-D** funcionar com permissões mínimas; **H04-E** gerar, instalar e executar bundle local.

Ambiente observado em 15/11/2026: Ubuntu 24.04.5, GNOME Shell 46.0, Flatpak 1.14.6, Rust/Cargo 1.100.0 do host, GTK 4.14.5, libadwaita 1.5.0, gtk4-rs 0.9.7 e libadwaita-rs 0.7.2. `XDG_SESSION_TYPE=wayland` e `XDG_CURRENT_DESKTOP=ubuntu:GNOME`, mas POCs anteriores não conseguiram abrir o display; não há evidência de acesso gráfico nem validação em Debian. `flatpak-builder` não está instalado. Há remote `flathub` no sistema e runtimes GNOME 49 e 50; SDK GNOME e extensão Rust não aparecem na lista de runtimes instalados. Não foram instalados pacotes, SDKs, remotes ou aplicações.

## 2. Configuração de build

ID `io.github.rafaelcarvalho89.OakNotePoc` coincide com o código GTK, manifesto, desktop, metainfo e ícone. `desktop-file-validate` e `appstreamcli` aceitaram os arquivos; a validação pedântica AppStream observou letras maiúsculas no ID e ausência de releases. O ID existente foi preservado, separado do produto principal; aceitação pelo builder e exportação ainda pendem de teste.

O manifesto seleciona `org.gnome.Platform//50`, `org.gnome.Sdk//50` e `org.freedesktop.Sdk.Extension.rust-stable//25.08`. O runtime 50 já estava instalado, usa a família Freedesktop 25.08 e foi escolhido como base disponível em vez de presumir a versão mais recente. Consulta pública ao Flathub mostrou SDK 50 atualizado em 28/10/2026 e extensão 25.08 atualizada em 07/09/2026, sem indicação de fim de suporte nos metadados consultados. A versão exata de `rustc` da extensão não foi observada; `cargo metadata` identifica requisitos declarados até Rust 1.85 entre as dependências, mas compatibilidade efetiva do SDK e bibliotecas nativas depende do build. Os artefatos remotos podem mudar.

O [gerador Cargo do flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo) produziu `flatpak/cargo-sources.json` a partir do `Cargo.lock`. Uma verificação local comparou nome, versão e SHA-256 das 75 crates com o lockfile. O builder deve baixar/verificar as fontes antes do build; `CARGO_HOME` aponta para a configuração gerada e `CARGO_NET_OFFLINE=true` e `cargo build --release --locked --offline` proíbem downloads pelo Cargo. Isso ainda não comprova um build offline no SDK. O PATH prioriza `/usr/lib/sdk/rust-stable/bin`; o comando de build registra versões de `rustc` e Cargo. Não é usado binário compilado no host.

## 3. Manifesto e permissões

O módulo instala binário em `/app/bin`, desktop em `/app/share/applications`, metainfo em `/app/share/metainfo` e ícone provisório em `/app/share/icons/hicolor/scalable/apps`. A única permissão declarada é `--socket=wayland`, para acesso ao compositor Wayland. Não há declaração de rede, filesystem amplo, acesso irrestrito ao D-Bus ou dispositivos, X11 ou DRI. A suficiência dessa configuração, inclusive para renderização e portal de preferências, é **pendente**; DRI e fallback X11 não foram adicionados sem evidência de necessidade. A permissão declarada não substitui inspeção das permissões efetivas, defaults, portais e overrides.

## 4. Resultados dos experimentos

| Experimento | Esperado | Observado e evidência | Estado |
| --- | --- | --- | --- |
| E01 — compilação | Build no SDK com Cargo offline e binário em `/app/bin` | Manifesto e 75 fontes com checksums preparados; `flatpak-builder` ausente, SDK/extensão não instalados | Pendente |
| E02 — instalação | Manifesto válido, build exportado, desktop e ícone reconhecidos; execução e fechamento | YAML analisado; desktop e AppStream validados localmente; nenhum build ou instalação | Pendente |
| E03 — isolamento | Execução com permissões mínimas e diretórios privados | Apenas permissões declaradas inspecionadas; efetivas, overrides e XDG no sandbox não medidos | Pendente |
| E04 — compatibilidade | Comportamento equivalente ao nativo validado | Nenhum teste gráfico; POCs nativas 01–03 também pendentes | Pendente |
| E05 — bundle | Gerar, instalar, executar e remover bundle | Nenhum repositório de build ou bundle gerado | Pendente |

Verificações executadas: `cargo check` e `cargo test` iniciais passaram; `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test` passaram (2 testes). `flatpak --version`, `flatpak remotes`, `flatpak list --runtime` e `flatpak info` identificaram o ambiente acima. `flatpak-builder --version` retornou comando não encontrado. `desktop-file-validate` passou; `appstreamcli validate --no-net --pedantic` passou com duas observações pedânticas citadas acima. YAML/JSON foram analisados e os checksums comparados com o lockfile. Não foi executada validação do manifesto pelo próprio builder.

## 5. Comparação com execução nativa

| Funcionalidade | Nativo | Flatpak | Diferença observada |
| --- | --- | --- | --- |
| Inicialização | Display inacessível nas POCs anteriores | Não executado | Pendente |
| Editor | Implementado; validação visual pendente | Não executado | Pendente |
| Transparência | H01 pendente | Não executado | Pendente |
| Sobreposição | H02 pendente | Não executado | Pendente |
| Temas | Implementados; H03-A pendente | Não executado | Pendente |
| Redimensionamento | H03-C pendente | Não executado | Pendente |

Não há equivalência ou incompatibilidade gráfica comprovada. Para o MVP, é necessária uma linha de base nativa observada antes de atribuir diferenças ao sandbox.

## 6. Limitações e diagnóstico futuro

Build, instalação, diretórios XDG e permissões efetivas não foram testados. O builder e o SDK 50 mais a extensão Rust 25.08 precisam ser disponibilizados pelo usuário. Um bundle normalmente não contém o runtime necessário; outra máquina pode precisar de um remote e download. Um bundle é um artefato local, não um repositório de atualização: atualizações exigem nova instalação ou uma estratégia de distribuição própria.

Após instalar a POC, use `flatpak info --user --show-permissions io.github.rafaelcarvalho89.OakNotePoc` e consulte também `flatpak override --user --show io.github.rafaelcarvalho89.OakNotePoc`, `flatpak override --show io.github.rafaelcarvalho89.OakNotePoc` e overrides globais aplicáveis. Valores efetivos de XDG podem ser observados com `flatpak run --command=sh io.github.rafaelcarvalho89.OakNotePoc -c 'printf "DATA=%s\nCONFIG=%s\nCACHE=%s\n" "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME"'`; não se assume o resultado. Portais e D-Bus filtrado também precisam ser considerados.

Para futura exportação, investigar o [portal FileChooser](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html) por meio de APIs GTK/GIO adequadas à versão escolhida, permitindo seleção explícita de destino sem acesso amplo à home. No MVP, testar seleção, permissões efetivas, cancelamento, escrita e erros. Não há exportação nesta POC.

## 7. Conclusões

**H04-A, H04-B, H04-C, H04-D e H04-E: pendentes.** A configuração é uma candidata experimental; nenhuma das hipóteses de build, execução ou distribuição foi comprovada. A viabilidade de Flatpak como formato inicial permanece em aberto. Validar as POCs nativas, compilar com os componentes Flatpak necessários e então comparar comportamento e permissões antes de decidir para o MVP.

## 8. Evidências e roteiro

Não foram criadas capturas ou bundle. O [README](../README.md) contém os comandos de preparação, build, instalação, inspeção e bundle. Após obter os componentes necessários, execute-os no Debian/GNOME/Wayland, registre saídas e versões do compilador no SDK, inspecione desktop, ícone, metadados e permissões, e teste abertura e fechamento. Compare editor, temas e redimensionamento com uma execução nativa observada; teste transparência e sobreposição conforme os resultados das POCs 01 e 02. Para o bundle, teste instalação e remoção apenas em ambiente isolado ou após confirmar que não afetará instalação existente. Guarde evidências reais em `docs/evidence/poc-04/`.

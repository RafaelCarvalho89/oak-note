# Oak Note POC

Prova de conceito independente do [Oak Note](https://github.com/RafaelCarvalho89/oak-note), um bloco de anotações nativo para Linux. Este repositório investiga tecnologias e comportamentos antes do MVP; não é o aplicativo principal. A visão do produto inclui notas rápidas, aparência ajustável e possível sobreposição de janelas, sujeita às capacidades do ambiente gráfico.

A POC-00 usa Rust, GTK4 e libadwaita para apresentar uma janela com texto editável. O ambiente principal pretendido é Debian com GNOME/Wayland.

## Pré-requisitos no Debian

Instale a toolchain estável do [Rust](https://www.rust-lang.org/tools/install), incluindo Cargo, rustfmt e Clippy, e as bibliotecas de desenvolvimento do GTK4 e libadwaita. No Debian, os pacotes necessários são `build-essential`, `pkg-config`, `libgtk-4-dev` e `libadwaita-1-dev`:

```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev
rustup component add rustfmt clippy
```

O comando `rustup` aplica-se a toolchains instaladas por rustup. Consulte a [documentação do GTK](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html) para detalhes da configuração. As dependências Rust são resolvidas pelo Cargo; é necessário acesso ao registro de crates na primeira compilação.

## Compilar e executar

Na raiz deste repositório:

```bash
cargo build --locked
cargo run --locked
```

Execute em uma sessão gráfica para visualizar e fechar a janela. O conteúdo é apenas demonstrativo e as alterações são perdidas ao fechar a aplicação.

## Verificações

```bash
cargo check --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

O [plano de experimentos](docs/poc-plan.md) registra hipóteses, procedimentos, evidências e resultados. A abertura real da janela em GNOME/Wayland ainda requer validação manual.

## Limitações

Não há persistência, gerenciamento de notas, editor Markdown, controle de transparência ou implementação de janela sempre no topo. A viabilidade da sobreposição depende do compositor e será investigada; não é uma capacidade presumida do GTK4. O código é experimental e pode ser descartado.

## Licença

Distribuído sob a [licença MIT](LICENSE).

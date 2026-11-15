# Oak Note POC

Prova de conceito independente do [Oak Note](https://github.com/RafaelCarvalho89/oak-note), um bloco de anotações nativo para Linux. Este repositório investiga tecnologias e comportamentos antes do MVP; não é o aplicativo principal. A visão do produto inclui notas rápidas, aparência ajustável e possível sobreposição de janelas, sujeita às capacidades do ambiente gráfico.

As POCs usam Rust, GTK4 e libadwaita para apresentar uma janela com texto editável, controle experimental de transparência do fundo e seletor de tema Sistema/Claro/Escuro. O ambiente principal pretendido é Debian com GNOME/Wayland.

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

Execute em uma sessão gráfica para visualizar e fechar a janela. Posicione outro aplicativo atrás e mova “Transparência do fundo” entre 0% e 70%; o valor inicial é 0%. Alterne o seletor entre Sistema, Claro e Escuro, inclusive após mudar a preferência do GNOME, e teste edição e redimensionamento (aproximadamente 800 × 560, 500 × 400 e 360 × 280). O conteúdo, o tema e o percentual são descartados ao fechar a aplicação.

## Sobreposição manual

A POC-02 investiga o menu de gerenciamento da janela no GNOME/Wayland. Ainda não há procedimento de ativação verificado para o Oak Note: o roteiro propõe tentar `Alt+Space` com a janela focada e procurar uma ação equivalente a “Sempre no topo”. A disponibilidade depende do compositor e das configurações da sessão. Consulte os [resultados e roteiro da POC-02](docs/poc-02-results.md).

## Verificações

```bash
cargo check --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

O [plano de experimentos](docs/poc-plan.md) registra hipóteses, procedimentos, evidências e resultados. A compilação e os testes unitários passaram no ambiente de desenvolvimento Ubuntu; a abertura da janela e a hipótese de transparência ainda requerem validação manual em Debian/GNOME/Wayland. Consulte os [resultados da POC-01](docs/poc-01-results.md) e os [resultados e roteiro da POC-03](docs/poc-03-results.md); a validação de temas, contraste, teclado e dimensões compactas também está pendente.

## Limitações

Não há persistência, gerenciamento de notas, editor Markdown ou implementação de janela sempre no topo. A viabilidade da sobreposição manual depende do compositor e permanece pendente; não é uma capacidade presumida do GTK4. O código é experimental e pode ser descartado.

## Licença

Distribuído sob a [licença MIT](LICENSE).

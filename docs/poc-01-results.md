# POC-01 — Transparência seletiva

## Identificação

**Objetivo:** investigar fundo com transparência ajustável, preservando a opacidade do conteúdo.

**H01:** CSS RGBA em GTK4/libadwaita permite variar a transparência do fundo e manter os widgets de conteúdo opacos no GNOME/Wayland.

**Ambiente observado em 15/11/2026:** Ubuntu 24.04.5, GNOME (`XDG_CURRENT_DESKTOP=ubuntu:GNOME`), `XDG_SESSION_TYPE=wayland`, Rust/Cargo 1.100.0, GTK 4.14.5, libadwaita 1.5.0, gtk4-rs 0.9.7 e libadwaita-rs 0.7.2. Este ambiente não é o Debian de referência. Apesar das variáveis de sessão, o processo não conseguiu acessar o display.

## Implementação

`GtkScale` controla um percentual de 0 a 70, inicialmente 0, mostrado ao lado. A função limita a entrada ao intervalo e calcula `alpha = 1 - percentual / 100`. Um `GtkCssProvider` é atualizado ao mover o controle ou mudar a preferência claro/escuro, detectada pelo `AdwStyleManager`. As cores explícitas são cinza claro (250, 250, 250) e escuro (36, 36, 36), evitando depender de variáveis CSS de versões mais recentes.

A hierarquia é `AdwApplicationWindow` → caixa vertical → `AdwHeaderBar` e caixa de fundo → `GtkScrolledWindow` → `GtkTextView`, mais a linha de controles. Classes específicas delimitam o CSS: a janela, o editor, o nó `text` e a área rolável recebem fundo transparente; a caixa de fundo recebe RGBA. Assim o alpha é aplicado uma vez, e não à opacidade dos widgets. O cabeçalho conserva o estilo padrão opaco. Nenhum `set_opacity` é utilizado. A renderização efetiva e possíveis camadas adicionais ainda precisam ser verificadas no compositor.

A [documentação GTK sobre migração](https://gnome.pages.gitlab.gnome.org/gtk/gtk4/migrating-3to4.html) descreve visuais RGBA e CSS para fundos transparentes; a [documentação de TextView](https://docs.gtk.org/gtk4/class.TextView.html) identifica seu nó interno `text`. Não houve comparação visual com alternativas porque o display não abriu.

## Resultados

| Cenário | Esperado | Observado | Estado | Evidência |
| --- | --- | --- | --- | --- |
| 01 — Transparência dinâmica | 0%, 25%, 50% e 70% mudam o fundo e revelam outro aplicativo | Não executado visualmente | Pendente | `cargo run`: `Failed to open display` |
| 02 — Legibilidade | Texto e controles opacos; contraste avaliado sobre fundos claros e escuros | Não executado visualmente | Pendente | Display inacessível |
| 03 — Redimensionamento | Fundo acompanha a janela sem artefatos relevantes | Não executado visualmente | Pendente | Display inacessível |
| 04 — Interação | Edição, seleção e controles funcionam a 70% | Não executado visualmente | Pendente | Display inacessível |
| 05 — Aparência | Comparar modos claro e escuro | Não executado visualmente | Pendente | Display inacessível |

Verificações em Ubuntu: `cargo check`, `cargo fmt --all -- --check` e `cargo clippy --all-targets -- -D warnings` aprovados; `cargo test` aprovado com dois testes de conversão e limites. `cargo run` compilou, mas terminou com `Gtk-WARNING: Failed to open display`; não é evidência de abertura de janela. O `cargo check` anterior às alterações também passou.

## Limitações e conclusão

**H01: pendente.** A implementação e os testes numéricos não comprovam que outros aplicativos aparecem atrás da janela ou que texto e controles permanecem visualmente opacos. Compatibilidade no Debian, artefatos, contraste e diferenças entre temas não foram observados. Mesmo com texto opaco, contraste pode ser insuficiente sobre conteúdo variável, especialmente a 70%. A viabilidade para o MVP permanece indeterminada até executar os cinco cenários no ambiente de referência e registrar evidências reais.

Após essa validação, a POC-02 deve investigar o procedimento manual de “Sempre no topo” no GNOME/Wayland, sem presumir controle da ordem das janelas pelo GTK4.

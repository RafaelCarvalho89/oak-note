# POC-03 — Aparência e comportamento da interface

## 1. Identificação

**Objetivo:** investigar temas, transparência, redimensionamento e usabilidade da interface experimental.

- **H03-A:** Sistema, Claro e Escuro podem ser alternados dinamicamente com libadwaita.
- **H03-B:** transparência seletiva funciona nos temas claro e escuro, preservando a opacidade do conteúdo.
- **H03-C:** a interface permanece funcional e consistente ao redimensionar, inclusive em tamanho compacto.
- **H03-D:** legibilidade e navegação básica por teclado são mantidas nos cenários exigidos.

**Ambiente observado em 15/11/2026:** Ubuntu 24.04.5 LTS, GNOME Shell 46.0, `XDG_SESSION_TYPE=wayland`, `XDG_CURRENT_DESKTOP=ubuntu:GNOME`, Rust 1.100.0, Cargo 1.100.0, GTK 4.14.5, libadwaita 1.5.0, gtk4-rs 0.9.7 e libadwaita-rs 0.7.2. Não é o Debian de referência. As variáveis indicam Wayland, mas não garantem acesso à sessão: `cargo run` e a tentativa com `GDK_BACKEND=wayland` retornaram `Failed to open display`. Backend efetivo e preferência do sistema não puderam ser observados. Orca está instalado em `/usr/bin/orca`, mas não foi possível testá-lo sem sessão gráfica acessível.

As hipóteses das [POCs 01](poc-01-results.md) e [02](poc-02-results.md) continuam pendentes; em particular, a transparência real ainda não foi validada.

## 2. Implementação

Um `GtkDropDown` seleciona Sistema, Claro ou Escuro. `AdwStyleManager::set_color_scheme` aplica respectivamente `PreferLight`, `ForceLight` e `ForceDark`; não há persistência. Sistema solicita preferência clara com acompanhamento da preferência escura do sistema quando disponível. A seleção é distinta do estado efetivo `is_dark`: este último já orienta a atualização do fundo. A disponibilidade da preferência do sistema pode ser consultada pela propriedade `system-supports-color-schemes` do `AdwStyleManager`, mas não foi medida nesta sessão. Não se infere a preferência real do sistema a partir da seleção.

O `GtkCssProvider` existente mantém classes específicas para janela, caixa de fundo, editor e rolagem. A caixa usa RGBA calculado pelo slider; editor e rolagem possuem fundo transparente e a header bar mantém estilo próprio. Cores fixas distintas (250,250,250 no claro; 36,36,36 no escuro) acompanham a aparência efetiva, mas podem divergir das cores do tema ou apresentar contraste insuficiente. O percentual não é alterado na troca de tema. Não se altera a opacidade de texto e controles por código; a composição e sua aparência continuam sem validação visual.

O conteúdo demonstrativo inclui parágrafos, lista Markdown, caracteres especiais e linhas para verificar rolagem. Os controles foram organizados em duas linhas para ajudar a investigação de tamanhos compactos; o tamanho mínimo efetivo ainda não foi medido. Não há alterações nas dependências.

## 3. Resultados experimentais

Todos os cenários visuais abaixo estão **pendentes**: não houve abertura observável da aplicação. A evidência disponível é a falha de acesso ao display, não uma avaliação da interface.

### E01 — Temas

| Teste | Esperado | Observado | Estado | Evidência |
| --- | --- | --- | --- | --- |
| Claro | Componentes claros sem reinício | Não executado visualmente | Pendente | `Failed to open display` |
| Escuro | Componentes escuros sem reinício | Não executado visualmente | Pendente | `Failed to open display` |
| Sistema claro e escuro | Acompanhar preferência disponível do sistema sem reinício | Não executado; suporte do sistema não medido | Pendente | Display inacessível |

### E02 — Transparência e temas

Em cada célula, o esperado é fundo realmente translúcido, texto e controles opacos, valor preservado na troca e editor utilizável. Observar contraste, janelas atrás, cor e artefatos. **Todas as células: pendentes**, não executadas; evidência: display inacessível e H01 pendente.

| Tema efetivo | 0% | 25% | 50% | 70% |
| --- | --- | --- | --- | --- |
| Claro | Pendente | Pendente | Pendente | Pendente |
| Escuro | Pendente | Pendente | Pendente | Pendente |
| Sistema — claro | Pendente | Pendente | Pendente | Pendente |
| Sistema — escuro | Pendente | Pendente | Pendente | Pendente |

### E03 — Redimensionamento

| Dimensão aproximada | Esperado | Observado | Estado | Evidência |
| --- | --- | --- | --- | --- |
| 800 × 560 | Editor, controles e rolagem funcionais | Não executado | Pendente | Display inacessível |
| 500 × 400 | Sem controles essenciais inacessíveis | Não executado | Pendente | Display inacessível |
| 360 × 280 | Cenário compacto funcional sem sobreposições | Não executado; mínimo desconhecido | Pendente | Display inacessível |
| Maximizada e redimensionamento contínuo | Renderização e interação sem falhas relevantes | Não executado | Pendente | Display inacessível |

### E04 — Legibilidade e acessibilidade básica

| Teste | Esperado | Observado | Estado | Evidência |
| --- | --- | --- | --- | --- |
| Contraste, seleção, cursor e foco | Legíveis em ambos os temas, inclusive transparência elevada | Não executado | Pendente | Display inacessível |
| Teclado | Tab/Shift+Tab, editor, seletor e slider utilizáveis, com foco visível | Não executado | Pendente | Display inacessível |
| Orca | Nomes e estados dos controles identificáveis | Instalado, não testado | Pendente | `command -v orca`: `/usr/bin/orca` |

## 4. Limitações

Contraste, cores, artefatos e limites de tamanho não foram observados. Texto opaco não garante legibilidade sobre conteúdo variável atrás da janela, sobretudo a 70%. O modo Sistema depende da preferência exposta pelo ambiente; não foi possível verificar seu suporte. As cores fixas podem não acompanhar todas as variações de tema. A disponibilidade do Orca não comprova acessibilidade; nenhuma conformidade integral é declarada. A investigação depende também da validação da POC-01 no compositor de referência.

## 5. Conclusões

**H03-A, H03-B, H03-C e H03-D: pendentes**, cada uma por falta das observações obrigatórias. O código compila, mas a viabilidade para o MVP ainda não foi demonstrada; não se deve escolher limites de transparência, dimensões mínimas ou afirmar acessibilidade com base apenas na implementação. Validar composição, contraste, teclado e mudanças de preferência antes de definir comportamento definitivo. Após obter uma linha de base nativa, a POC-04 poderá comparar execução Flatpak e sandbox sem atribuir diferenças ainda não medidas ao empacotamento.

## 6. Evidências e roteiro manual

Nenhuma captura foi produzida. Para validar em Debian/GNOME/Wayland:

1. Registre `cat /etc/os-release`, `gnome-shell --version`, `echo "$XDG_SESSION_TYPE"`, `echo "$XDG_CURRENT_DESKTOP"`, `pkg-config --modversion gtk4 libadwaita-1`, `rustc --version` e `cargo --version`. Execute `GDK_BACKEND=wayland cargo run` em terminal da sessão gráfica.
2. Com outro aplicativo visível atrás, alterne Sistema, Claro e Escuro sem reiniciar. No modo Sistema altere a preferência do GNOME para claro e escuro, se disponível; registre a preferência selecionada, a do sistema e a aparência observada, inclusive se o sistema não a disponibilizar.
3. Para claro e escuro, teste 0%, 25%, 50% e 70%; repita Sistema em ambos os estados efetivos quando disponíveis. Observe fundo, controles, opacidade, cores, contraste, seleção, cursor e bordas sobre fundos claros e escuros. Confira que a troca de tema preserva o percentual e que o editor funciona.
4. Teste aproximadamente 800 × 560, 500 × 400, 360 × 280 e maximizada, sem forçar abaixo do mínimo permitido. Em cada tamanho teste rolagem, edição, controles, temas e transparência; observe o redimensionamento contínuo.
5. Use Tab e Shift+Tab, acesse editor e seletor, ajuste o slider com teclado e observe foco e bloqueios. Se Orca estiver disponível na sessão, verifique nomes e estados sem instalar ou alterar configurações globais.
6. Registre resultados, versões, limites e eventuais falhas; armazene capturas reais e representativas, com nomes de tema, percentual e dimensão, em `docs/evidence/poc-03/`.

Verificações nesta sessão: `cargo check` inicial e final, `cargo fmt --all -- --check` e `cargo clippy --all-targets -- -D warnings` passaram; `cargo test` passou com dois testes existentes. `cargo run` terminou com `Failed to open display` (código 1); a tentativa com `GDK_BACKEND=wayland` apresentou a mesma falha. Não há evidência de execução gráfica.

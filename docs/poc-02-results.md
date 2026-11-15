# POC-02 — Sobreposição manual no GNOME/Wayland

## 1. Identificação

**Objetivo:** investigar a ativação manual de sobreposição para a janela GTK4/libadwaita do Oak Note.

**H02:** no GNOME/Wayland, o usuário consegue ativar manualmente “Sempre no topo” para uma janela convencional, que permanece acima de janelas normais mesmo sem foco.

**Ambiente observado em 15/11/2026:** Ubuntu 24.04.5 LTS (não Debian), GNOME Shell 46.0, `XDG_SESSION_TYPE=wayland`, `XDG_CURRENT_DESKTOP=ubuntu:GNOME`, `WAYLAND_DISPLAY=wayland-0`, GTK 4.14.5, libadwaita 1.5.0, Rust 1.100.0 e Cargo 1.100.0. As variáveis indicam uma sessão Wayland, mas não comprovam que ela esteja acessível neste processo. `GDK_BACKEND=wayland cargo run` falhou com `Gtk-WARNING: Failed to open display`; o backend efetivo de uma janela não pôde ser confirmado. O Debian e sua versão não foram verificados.

## 2. Fundamentação técnica

No GNOME/Wayland, o compositor/gerenciador de janelas Mutter decide o posicionamento das janelas. A aplicação apresenta uma `AdwApplicationWindow` convencional com `AdwHeaderBar`; `present()` apresenta a janela, mas não configura sobreposição permanente. Esta POC não usa API GTK3 de manter acima nem altera a ordem por código.

A [ajuda do GNOME](https://help.gnome.org/gnome-help/keyboard-shortcuts-set) lista `Alt+Space` como atalho padrão para o menu da janela; ele pode ter sido personalizado. A [ajuda de navegação](https://help.gnome.org/gnome-help/keyboard-nav.html) também descreve o clique direito na barra de título. Ambos são pontos de partida, não procedimentos confirmados nesta aplicação. A existência ou o nome da opção de sobreposição e sua ativação/desativação devem ser observados no menu real.

O GTK4 documenta [`GDK_TOPLEVEL_STATE_ABOVE`](https://docs.gtk.org/gdk4/flags.ToplevelState.html) como estado de uma superfície. Sua disponibilização e correspondência com o comportamento no backend Wayland não foram verificadas; não foi adicionado diagnóstico nem se considera essa flag evidência funcional.

## 3. Procedimentos executados e roteiro pendente

Foram consultadas as versões e variáveis do ambiente, executadas as verificações Cargo e tentada a execução com `GDK_BACKEND=wayland`. Nenhum dos testes interativos abaixo pôde ser executado.

Em uma sessão Debian/GNOME/Wayland acessível, registre `cat /etc/os-release`, `gnome-shell --version`, `echo "$XDG_SESSION_TYPE"`, `echo "$XDG_CURRENT_DESKTOP"`, `pkg-config --modversion gtk4 libadwaita-1`, `rustc --version` e `cargo --version`. Execute `GDK_BACKEND=wayland cargo run` para exigir o backend Wayland; confirme a sessão e registre as versões e eventuais personalizações ou extensões relevantes.

1. **Menu:** com o Oak Note focado, tente `Alt+Space`; registre as opções reais. Tente também clique direito na barra de título e, se necessário, `Super`+clique direito. Se houver “Sempre no topo” ou equivalente, ative-a e registre como desativá-la; caso não apareça, registre exatamente o observado.
2. **Foco e sobreposição:** sobreponha o Oak Note e outro aplicativo normal. Ative a opção, foque e use o segundo aplicativo, alterne o foco, desative a opção e compare. Verifique interação nas partes acessíveis do aplicativo de trás.
3. **Interação:** com a opção ativa, redimensione em diferentes tamanhos, alterne o foco, digite e selecione texto e use os controles; observe artefatos e se o estado permanece.
4. **Transparência:** somente após validar o funcionamento da POC-01, repita com 0%, 25%, 50% e 70%, observando conteúdo atrás, opacidade, contraste e entrada de mouse e teclado. A POC-01 está pendente; não corrigir sua implementação nesta etapa.
5. **Ciclo de vida:** com a opção ativa, minimize e restaure; maximize e restaure; feche e reabra; alterne de workspace; teste outro aplicativo em tela cheia. Registre cada resultado sem presumir persistência. “Sempre no topo” e visibilidade em todos os workspaces são estados distintos; fullscreen é exploratório.

## 4. Resultados

| Cenário | Resultado esperado | Resultado observado | Estado | Evidência |
| --- | --- | --- | --- | --- |
| Menu: `Alt+Space` | Acessar menu e ativar/desativar opção | Não executado | Pendente | Display inacessível |
| Menu: mouse | Identificar alternativas disponíveis | Não executado | Pendente | Display inacessível |
| Sobreposição e foco | Permanecer acima de janela normal sem foco, permitindo interação e desativação | Não executado | Pendente | Display inacessível |
| Redimensionamento e editor | Editor e controles funcionam, sem perda indevida da sobreposição | Não executado | Pendente | Display inacessível |
| Integração com transparência | Sobreposição e transparência coexistem, com conteúdo opaco e entrada normal | Não concluído; POC-01 também pendente | Pendente | [Resultados da POC-01](poc-01-results.md) |
| Minimizar e restaurar | Observar estado após restauração | Não executado | Pendente | Display inacessível |
| Maximizar e restaurar | Observar comportamento e estado | Não executado | Pendente | Display inacessível |
| Fechar e reabrir | Observar eventual persistência | Não executado | Pendente | Display inacessível |
| Workspaces | Distinguir sobreposição de presença em todos os workspaces | Não executado | Pendente | Display inacessível |
| Outro aplicativo em fullscreen | Observar comportamento, sem exigência de sobreposição | Não executado | Pendente | Display inacessível |

## 5. Evidências e verificações

Não há capturas: nenhuma janela pôde ser observada. Ao executar os testes, salve apenas evidências reais relevantes em `docs/evidence/poc-02/` e registre ações, versões e observações, incluindo a troca de foco e a desativação.

`cargo check`, `cargo fmt --all -- --check` e `cargo clippy --all-targets -- -D warnings` passaram. `cargo test` passou com dois testes existentes. A tentativa `GDK_BACKEND=wayland timeout 8s cargo run` compilou e informou `Failed to open display`; não comprova abertura nem comportamento gráfico.

## 6. Limitações

A disponibilidade do menu e da ação depende do GNOME Shell/Mutter, da configuração e do tipo de janela; extensões e atalhos personalizados podem interferir. O comportamento em outros compositores Wayland e tipos de janela não foi investigado. Não se deve presumir sobreposição acima de fullscreen, interfaces do sistema ou diálogos privilegiados, nem persistência após reiniciar. A integração com transparência depende da validação da POC-01. Não foram observados comportamentos inconsistentes nem artefatos, pois os testes visuais não foram executados.

## 7. Conclusão

**H02: pendente**, por ausência de evidência funcional em Debian/GNOME/Wayland. A viabilidade para o MVP e as instruções definitivas para o produto dependem dos testes; qualquer documentação futura deve explicitar a dependência do compositor e os limites efetivamente observados. Após validar H01 e H02, a POC-03 pode investigar aparência, legibilidade e redimensionamento sem presumir que esses resultados já foram aprovados.

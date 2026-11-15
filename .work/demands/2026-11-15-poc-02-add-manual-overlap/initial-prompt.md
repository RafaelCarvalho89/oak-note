# Oak Note POC — 02: Sobreposição manual no GNOME/Wayland

## Contexto

Estamos desenvolvendo o **Oak Note**, um aplicativo Open Source nativo para Linux Debian, utilizando Rust + GTK4 + libadwaita.

O repositório atual, `oak-note-poc`, é um laboratório independente do projeto principal `oak-note`.

A finalidade da POC é validar hipóteses técnicas antes de iniciar o MVP.

**Ambiente principal:** Debian + GNOME (Wayland).

### Etapas anteriores

- **POC-00:** Bootstrap da aplicação Rust + GTK4/libadwaita.
- **POC-01:** Investigação de transparência seletiva do fundo.
- **POC-02:** Investigação da sobreposição manual — etapa atual.

Não presuma que todas as etapas anteriores foram aprovadas. Verifique os arquivos, resultados e evidências existentes antes de continuar.

## 1. Objetivo

Investigar e validar o mecanismo manual que permite manter a janela do Oak Note acima de outras janelas no GNOME/Wayland.

A POC deverá responder:

1. O GNOME oferece a ação "Sempre no topo" para a janela do Oak Note?
2. Como o usuário pode ativar e desativar essa opção?
3. A ação permanece funcional com uma janela GTK4/libadwaita?
4. A janela permanece sobreposta quando outro aplicativo recebe foco?
5. O comportamento continua funcionando após redimensionamento e interação com o editor?
6. Como a transparência da POC-01 se comporta durante a sobreposição?
7. Quais são as limitações identificadas?

A POC deve produzir evidências técnicas e um procedimento reproduzível.

**Não implemente controle automático de sobreposição.**

## 2. Inspeção inicial

Antes de realizar alterações:

1. Inspecione a estrutura do repositório.
2. Leia README.md.
3. Leia `docs/poc-plan.md`, se existir.
4. Leia `docs/poc-01-results.md`, se existir.
5. Analise a janela GTK4/libadwaita implementada.
6. Verifique como a header bar foi construída.
7. Identifique alterações existentes que não fazem parte desta tarefa.
8. Execute `cargo check`, quando possível.

Preserve todo o código funcional das POCs anteriores.

Não reinicialize Cargo, não substitua arquivos indiscriminadamente e não introduza refatorações sem necessidade.

## 3. Hipótese técnica

**H02:** No GNOME/Wayland, o usuário consegue ativar manualmente o estado "Sempre no topo" para uma janela GTK4/libadwaita convencional, fazendo com que ela permaneça visível acima de outras janelas normais, mesmo quando perde o foco.

### Restrições técnicas

O GTK4 removeu `gtk_window_set_keep_above()`.

Portanto:

- Não utilize APIs exclusivas do GTK3.
- Não tente modificar diretamente a ordem das janelas pelo protocolo Wayland.
- Não utilize comandos X11 como `wmctrl` ou `xdotool` como solução para o GNOME/Wayland.
- Não implemente extensões GNOME Shell nesta etapa.
- Não utilize `gtk4-layer-shell` nesta etapa.
- Não implemente manipulações privadas do Mutter.
- Não tente simular sobreposição utilizando fullscreen ou modal.
- Não considere `window.present()` uma implementação de "Sempre no topo".

A ativação deve ocorrer pelo gerenciador de janelas, mediante ação explícita do usuário.

## 4. Verificação do ambiente

Identifique o ambiente em que os testes serão realizados.

Utilize comandos não destrutivos, como:

```bash
echo "$XDG_SESSION_TYPE"
echo "$XDG_CURRENT_DESKTOP"
gnome-shell --version
```

Registre:

- Distribuição e versão do Debian.
- Versão do GNOME Shell.
- Tipo de sessão gráfica.
- Versões GTK4 e libadwaita.
- Versões Rust e Cargo.
- Backend gráfico efetivamente utilizado, quando verificável.

Não assuma que GNOME significa necessariamente uma sessão Wayland.

Se não houver sessão GNOME/Wayland acessível, mantenha os testes funcionais como pendentes.

Não altere configurações globais do sistema automaticamente.

## 5. Experimento 01 — Acesso ao menu da janela

Execute a aplicação normalmente.

Investigue os seguintes métodos para acessar o menu de gerenciamento da janela:

### Método A — Atalho Alt+Space

1. Coloque o foco na janela do Oak Note.
2. Pressione `Alt+Space`.
3. Verifique se o GNOME apresenta o menu da janela.
4. Procure uma opção equivalente a "Sempre no topo" / "Always on Top".
5. Registre as opções efetivamente encontradas.

### Método B — Menu pelo mouse

1. Tente abrir o menu contextual da janela pela barra de título.
2. Se necessário, experimente `Super+clique direito` sobre a janela.
3. Verifique se o GNOME apresenta o menu.
4. Identifique a ação de sobreposição, caso disponível.

Não presuma que o clique direito convencional funcionará em todas as header bars do libadwaita.

### Critério de aprovação

Ao menos um procedimento manual deve permitir acessar e ativar a opção de sobreposição para a janela do Oak Note.

Caso o menu esteja indisponível, registre exatamente o comportamento observado, incluindo possíveis interferências de extensões ou personalizações do GNOME.

## 6. Experimento 02 — Permanência acima de outras janelas

Utilize pelo menos um segundo aplicativo, preferencialmente o Firefox, um editor de texto ou o gerenciador de arquivos.

Procedimento:

1. Abra o Oak Note.
2. Abra o segundo aplicativo.
3. Posicione as duas janelas de maneira que suas áreas se sobreponham.
4. Ative manualmente "Sempre no topo" no Oak Note.
5. Coloque o foco no segundo aplicativo.
6. Digite ou interaja com ele.
7. Observe a posição do Oak Note.
8. Alterne o foco entre os dois aplicativos.
9. Desative "Sempre no topo" e observe a diferença.

### Critérios de aprovação

- O Oak Note permanece visível acima de outras janelas normais após perder o foco.
- É possível interagir com o aplicativo posicionado atrás dele.
- O comportamento pode ser desativado manualmente.
- A funcionalidade não depende de modificar o código da aplicação.

Não confunda uma janela que recebeu foco recentemente com uma janela efetivamente configurada como "Sempre no topo".

## 7. Experimento 03 — Redimensionamento e interação

Com a sobreposição ativada:

1. Redimensione o Oak Note.
2. Altere o foco para outro aplicativo.
3. Retorne ao Oak Note.
4. Digite no editor.
5. Selecione texto.
6. Utilize os controles existentes.
7. Repita o teste com diferentes dimensões.

Verifique se:

- O redimensionamento continua funcionando.
- O editor permanece utilizável.
- Os controles respondem normalmente.
- A sobreposição não é desativada indevidamente.
- Não aparecem artefatos visuais relevantes.

## 8. Experimento 04 — Integração com transparência

Caso a POC-01 tenha implementado um mecanismo funcional de transparência seletiva, execute este experimento.

1. Ative "Sempre no topo".
2. Posicione outro aplicativo atrás do Oak Note.
3. Ajuste a transparência para 0%.
4. Repita com 25%, 50% e 70%.
5. Mantenha o foco no aplicativo de trás.
6. Observe a legibilidade do texto no Oak Note.
7. Verifique se o aplicativo de trás permanece visível através do fundo.

### Critérios de avaliação

- A sobreposição manual funciona em conjunto com a transparência.
- O texto e os controles continuam opacos.
- A janela mantém comportamento normal de entrada de teclado e mouse.
- Não ocorre passagem involuntária de cliques através da janela.
- As limitações de contraste são identificadas.

Se a POC-01 estiver reprovada ou pendente, não tente corrigir sua implementação nesta etapa.

Registre que o experimento integrado não pôde ser concluído e indique a dependência.

## 9. Experimento 05 — Ciclo de vida da janela

Investigue o comportamento da sobreposição nos seguintes cenários:

### Cenário A — Minimização

1. Ative a sobreposição.
2. Minimize o Oak Note.
3. Restaure a janela.
4. Observe se a sobreposição continua ativa.

### Cenário B — Maximização

1. Ative a sobreposição.
2. Maximize e restaure a janela.
3. Observe o comportamento.

### Cenário C — Encerramento

1. Ative a sobreposição.
2. Feche o Oak Note.
3. Execute novamente.
4. Observe se a opção continua ativa.

Não presuma que o estado "Sempre no topo" será persistido após encerrar e reabrir a aplicação.

### Cenário D — Workspaces

1. Mantenha o Oak Note sobreposto.
2. Alterne para outro workspace.
3. Observe o comportamento.

Diferencie "Sempre no topo" de "Sempre no workspace visível". São funcionalidades distintas.

### Cenário E — Aplicativo em tela cheia

Verifique como o Oak Note se comporta diante de outro aplicativo em fullscreen.

Este cenário é exploratório e não constitui requisito de aprovação da hipótese H02.

Não exija que o Oak Note permaneça acima de janelas fullscreen, interfaces do sistema ou diálogos privilegiados.

## 10. Diagnóstico opcional

Investigue se é viável consultar, em modo somente leitura, o estado de sobreposição disponibilizado por `GdkToplevel`.

O GTK4 define o estado `GDK_TOPLEVEL_STATE_ABOVE`.

Se as versões presentes permitirem observar esse estado sem adicionar complexidade desnecessária, considere implementar um diagnóstico experimental.

Requisitos:

- Não modificar o estado da janela por código.
- Não adicionar dependências novas apenas para esse diagnóstico.
- Não apresentar um indicador como confiável sem verificar que corresponde ao comportamento observado.
- Documentar se a flag é informada corretamente pelo backend utilizado.

Essa investigação é opcional.

A ausência de detecção programática não reprova a hipótese de sobreposição manual.

## 11. Alterações no código

Evite modificar a aplicação se os experimentos puderem ser realizados diretamente com a janela já existente.

Se forem necessárias pequenas alterações para facilitar os testes:

- Preserve a interface da POC-01.
- Mantenha a janela como uma janela GTK4/libadwaita convencional.
- Utilize APIs públicas suportadas.
- Evite abstrações complexas.
- Documente cada alteração e sua finalidade.

Não implemente um botão "Sempre no topo" que dê a entender que a aplicação consegue controlar essa propriedade.

É aceitável incluir instruções de uso na documentação em vez de adicionar elementos permanentes à interface.

## 12. Testes automatizados

Preserve os testes existentes.

Execute:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Esta POC não exige novos testes unitários caso nenhuma lógica testável seja adicionada.

A comprovação principal será realizada por testes manuais no GNOME/Wayland.

Não tente substituir a validação visual por testes artificiais que apenas confirmem que o aplicativo compila.

## 13. Registro dos resultados

Crie:

`docs/poc-02-results.md`

Utilize a seguinte estrutura:

### 1. Identificação
- Nome da POC.
- Objetivo.
- Hipótese H02.
- Ambiente de execução.

### 2. Fundamentação técnica
- Responsabilidade do Mutter pela sobreposição.
- Limitações das APIs GTK4.
- Mecanismos manuais identificados.

### 3. Procedimentos executados
- Acesso ao menu da janela.
- Sobreposição e alternância de foco.
- Redimensionamento e interação.
- Integração com transparência.
- Minimização, maximização e encerramento.
- Workspaces e fullscreen.

### 4. Resultados

Para cada cenário, documente:

- Resultado esperado.
- Resultado observado.
- Estado: aprovado, reprovado ou pendente.
- Evidências disponíveis.

### 5. Evidências

Quando possível, registre capturas de tela em:

`docs/evidence/poc-02/`

As evidências devem demonstrar o comportamento real, sem imagens fictícias.

### 6. Limitações
- Dependências do GNOME Shell/Mutter.
- Comportamentos inconsistentes.
- Limitações do Wayland.
- Diferenças entre tipos de janelas.
- Problemas encontrados.

### 7. Conclusão
- Estado da hipótese H02.
- Viabilidade da sobreposição manual para o MVP.
- Limitações que precisam constar na documentação do produto.
- Recomendações para etapas posteriores.

## 14. Atualização da documentação

Atualize `docs/poc-plan.md` com o andamento real da POC-02.

Atualize README.md com:

- Descrição do experimento.
- Instruções de ativação manual efetivamente verificadas.
- Limitações conhecidas.
- Referência para `docs/poc-02-results.md`.

Não altere o README para declarar a funcionalidade como validada caso os testes não tenham sido executados.

## 15. Critérios de aprovação

A hipótese H02 será considerada **aprovada** quando:

1. O ambiente GNOME/Wayland estiver confirmado.
2. Houver um procedimento manual reproduzível para ativar "Sempre no topo".
3. O Oak Note permanecer acima de janelas normais após perder o foco.
4. O usuário conseguir interagir com outros aplicativos enquanto o Oak Note permanece visível.
5. A sobreposição puder ser desativada.
6. O editor e o redimensionamento continuarem funcionais.
7. Os resultados estiverem documentados com evidências.

Classifique a hipótese como:

- **Aprovada:** todos os critérios obrigatórios verificados.
- **Reprovada:** algum critério obrigatório testado não foi atendido.
- **Pendente:** evidências insuficientes para uma conclusão.

Os testes de integração com transparência, fullscreen e persistência após reinicialização são complementares e devem ser registrados separadamente.

Não considere a implementação concluída como prova automática da hipótese.

## 16. Fora do escopo

Não implementar nesta tarefa:

- Extensão GNOME Shell.
- Sobreposição automática.
- API personalizada de controle de janelas.
- Integração D-Bus com Mutter.
- Manipulação da ordem das janelas por X11.
- Modificações globais nas configurações do GNOME.
- Persistência automática do estado "Sempre no topo".
- Múltiplas janelas de notas.
- Novas funcionalidades de edição.
- Empacotamento Flatpak.
- Funcionalidades definitivas do MVP.

## 17. Entrega

Ao finalizar, apresente:

1. Resumo da investigação.
2. Arquivos criados e modificados.
3. Procedimento manual de sobreposição.
4. Resultados dos experimentos.
5. Comandos executados e respectivas saídas.
6. Estado da hipótese H02.
7. Limitações técnicas identificadas.
8. Recomendação para a POC-03.

Caso não tenha acesso à sessão gráfica, documente a investigação e apresente um roteiro de validação manual para o usuário executar.

Não invente resultados ou evidências.

**Não crie commits e não envie alterações ao GitHub sem autorização.**

Não modifique o repositório principal `oak-note`.

Não inicie a POC-03 nesta execução.

Priorize investigação controlada, mínimo de alterações no código, evidências reproduzíveis e documentação objetiva.
# Oak Note — Especificação do Produto (MVP v0.1.0)

> **Estado:** RASCUNHO — PREP-01, aguardando revisão e aprovação.  
> **Última atualização:** 2026-11-16  
> **Produto:** Oak Note  
> **Repositório principal:** https://github.com/RafaelCarvalho89/oak-note  
> **Documento:** `.docs/mvp/product-specification.md`  
> **Próxima etapa:** PREP-02 — Especificação técnica e arquitetura

## 1. Objetivo e fontes de decisão

Este documento consolida o escopo de produto do **Oak Note v0.1.0** a partir das **21 decisões do briefing** e das **ADRs 01–07**, aprovadas durante o planejamento. Define **o que** o MVP deverá entregar e quais comportamentos serão aceitos, sem antecipar detalhes técnicos de implementação ou iniciar desenvolvimento.

As POCs 00–04 serviram de investigação técnica, e o responsável pelo projeto confirmou a validação manual no Debian/GNOME/Wayland e a instalação Flatpak. Os relatórios experimentais podem registrar estados anteriores dessa validação. Essa confirmação não constitui uma exigência de reutilizar integralmente o código da POC.

**Precedência:** em caso de divergência, prevalecem as escolhas explícitas mais recentes do briefing e das ADRs, inclusive os refinamentos posteriores. A **DT-01 (alternativa B)** foi aprovada em 2026-11-16: quando houver nome duplicado, a aplicação sugere um nome alternativo e permite ao usuário confirmar ou editar. Foram também aprovadas a **organização feature-based**, a aplicação pragmática de arquitetura hexagonal e o **diretório XDG Documentos como local inicial configurável**. Questões não decididas estão em [Questões abertas](#12-questões-abertas-para-refinamento).

## 2. Visão do produto

O Oak Note é um **bloco de anotações Open Source, nativo para Linux**, projetado para registrar e consultar informações rapidamente, com uma interface discreta e intuitiva. O diferencial é a possibilidade de manter a janela acima de outras aplicações, através do controle **manual do GNOME**, combinada com transparência dos fundos e texto legível.

**Proposta de valor:** permitir que uma pessoa consulte ou edite suas notas Markdown enquanto trabalha em outros aplicativos, sem depender de nuvem, conta ou ferramenta complexa de organização.

**Plataforma primária:** Debian + GNOME em sessão Wayland.  
**Stack estabelecida no planejamento:** Rust, GTK4 e libadwaita.  
**Licença:** MIT.  
**Modelo de distribuição inicial:** código-fonte e Flatpak publicado no GitHub Releases, com publicação controlada.  
**Idioma da interface:** português brasileiro e inglês, acompanhando automaticamente o idioma do sistema.

### 2.1. Público-alvo e contexto de uso

Pessoas que usam Linux no trabalho ou no estudo e precisam manter pequenas notas de referência, ideias, tarefas ou textos acessíveis durante outras atividades. O produto não exige conhecimento de programação ou de Markdown para inserir texto, embora aceite sua sintaxe.

### 2.2. Resultados esperados

- Registrar uma anotação rapidamente e encontrá-la depois pelo título.
- Evitar perda silenciosa de conteúdo durante salvamentos e encerramentos.
- Permitir consulta de uma nota em janela compacta, com lista lateral ocultável.
- Aplicar temas e transparência sem tornar texto e controles translúcidos.
- Preservar arquivos `.md` legíveis e portáveis, sem serviços externos.

Não foram estabelecidas métricas numéricas de adoção, desempenho ou retenção. Metas quantitativas só serão adicionadas mediante medição e decisão posterior.

## 3. Escopo funcional aprovado

| Área | Incluído no MVP | Limite do escopo |
| --- | --- | --- |
| Organização | Várias notas em **uma única janela** | Sem múltiplas janelas de notas |
| Instância | Uma instância ativa por sessão; nova abertura ativa a janela existente | Sem editores independentes simultâneos na mesma sessão |
| Edição | Editor de **Markdown textual**, sem preview obrigatório | Sem editor rico/WYSIWYG |
| Arquivos | Uma nota por `.md`, nome físico derivado do título | Sem SQLite ou índice auxiliar |
| Diretório de notas | Pasta XDG **Documentos** do usuário por padrão, com seleção de outro diretório nas preferências | Sem caminho fixo de usuário e sem obrigação de guardar em área privada do Flatpak |
| Gerenciamento | Criar, selecionar, renomear e enviar nota à lixeira após confirmação | Sem pastas, tags ou favoritos; sem exclusão permanente como fallback |
| Busca | Pesquisa rápida **por título** | Sem busca de conteúdo |
| Ordenação | Mais recentemente modificada primeiro | Sem ordenação configurável |
| Persistência | Autosave, gravação segura, controle de conflito e erros | Sem versões históricas automáticas |
| Layout | Barra lateral recolhível e editor expandido | Sem interface definitiva de múltiplos painéis |
| Janela | Redimensionamento, maximização e restauração de estado | Sem controle programático de posição |
| Sobreposição | Ativação **manual** pelo GNOME | Sem extensão GNOME no MVP |
| Aparência | Sistema/Claro/Escuro e transparência dos **fundos da janela principal**, inclusive cabeçalho | Sem tornar conteúdo translúcido ou habilitar click-through |
| Preferências | Janela libadwaita com esquema GSettings único | Sem perfis de configuração |
| Acesso rápido | Atalhos internos e atalho do GNOME configurado pelo usuário | Sem captura global de atalhos pelo aplicativo |
| Exportação | Exportação manual de todas as notas Markdown | Sem sincronização ou backup automático |
| Qualidade | Testes automatizados do núcleo, CI e testes manuais da UI | Sem exigência de suíte GUI automatizada |
| Distribuição | Flatpak GitHub Releases e compilação pelo código-fonte | Flathub futuro, não incluído |

## 4. Modelo conceitual e regras de negócio

### 4.1. Nota

Uma **nota** possui:
- **Título**, derivado do nome físico do arquivo `.md`, sem a extensão;
- **Conteúdo**, texto Markdown do arquivo;
- **Caminho**, referência operacional ao arquivo correspondente;
- **Última modificação**, derivada dos metadados do sistema de arquivos quando disponível.

**ADR-01 e ADR-02:** o título origina o nome do arquivo; o Markdown permanece puro, sem YAML Front Matter, identificador interno persistente ou índice centralizado. A data histórica de criação não é garantida.

O caminho da nota **não é uma identidade imutável**: a renomeação altera o nome físico. O MVP deve lidar com isso sem perder a nota aberta ou sobrescrever outra nota.

**Diretório de armazenamento:** por padrão, as notas residem diretamente no diretório especial **Documentos** configurado para o usuário (XDG user directory; por exemplo, `/home/<USER>/Documentos` em ambiente pt-BR). O usuário pode selecionar outro diretório. Não se presume a criação automática de uma subpasta `Oak Note`. **DT-17 aprovada (alternativa C):** ao alterar o diretório ativo, o usuário escolhe explicitamente entre **manter as notas na pasta original** e **migrá-las para a nova pasta**. Não mover ou apagar arquivos implicitamente. **DT-20 aprovada (alternativa B):** quando houver migração autorizada, copiar todas as notas, validar a integridade das cópias, **ativar o novo destino apenas após a validação completa** e remover os arquivos originais somente após **confirmação explícita adicional e verificações de segurança**. Erros não podem causar exclusão silenciosa; não há garantia de transação atômica entre pastas. O detalhamento de falhas parciais e recuperação permanece na PREP-02.

**DT-19 aprovada (alternativa B):** no primeiro acesso a uma pasta que já contenha arquivos `.md` diretamente na **raiz**, o Oak Note informa que esses arquivos serão reconhecidos como notas e solicita confirmação. O usuário pode aceitar ou escolher outra pasta. Não há busca em subdiretórios, cópia/importação nem alteração de arquivos por causa desse reconhecimento. A forma de lembrar a confirmação por pasta será refinada na PREP-02.

**DT-18 aprovada (alternativa A):** a distribuição Flatpak solicita acesso estático de leitura e escrita **apenas ao diretório XDG Documentos**, que é a pasta padrão. Para outras pastas, o usuário autoriza o acesso por seletor integrado ao **portal de arquivos**, sem ampliar permissões automaticamente para `home`, `host` ou destinos arbitrários. A aplicação deve **validar operações reais de leitura, listagem, criação, gravação/substituição segura, renomeação, envio à lixeira e acesso persistente entre sessões** antes de ativar a nova pasta. Se alguma capacidade obrigatória falhar, informar a limitação e manter o diretório anterior; o comportamento real depende de validação no Flatpak e não está tecnicamente comprovado.

### 4.2. Integridade e concorrência

**ADR-03:** o salvamento automático usa debounce, arquivo temporário no mesmo diretório, substituição atômica quando suportada, escrita serializada por nota e tratamento explícito de falhas. Revisões antigas não podem substituir conteúdo mais recente.

**DT-04 aprovada (alternativa A):** o Oak Note possui **uma instância ativa por sessão da aplicação**. Novas ativações na mesma sessão direcionam o usuário à janela existente, sem iniciar um segundo editor independente. Operações internas sobre a mesma nota — salvamento, renomeação, envio à lixeira e migração — devem ser coordenadas e serializadas. Essa regra não restringe editores externos, outras sessões ou instalações com identidade diferente; a detecção e resolução de conflitos da DT-02 continuam obrigatórias.

**DT-05 aprovada (alternativa B):** no autosave, após gravar o conteúdo em arquivo temporário no mesmo diretório, o Oak Note solicita sincronização dos dados do arquivo; depois publica a substituição atômica quando suportada e solicita sincronização do diretório quando disponível. O aplicativo deve distinguir **edição pendente, gravação publicada, durabilidade confirmada e durabilidade não verificada**; erros não podem ser comunicados como salvamento seguro. Isso **não garante durabilidade absoluta**, especialmente diante de limitações do filesystem ou de acessos via portal/Flatpak. O debounce de aproximadamente 700 ms continua **candidato para validação**, não valor aprovado.

**DT-06 aprovada (alternativa B):** ao trocar de nota ou encerrar, o aplicativo tenta concluir as gravações pendentes antes de prosseguir. Se houver falha ou durabilidade incerta, oferece **tentar novamente**, **cancelar a ação e continuar editando** ou **descartar conscientemente as alterações locais**, com confirmação específica e aviso de que não há garantia de recuperação. Não descarta conteúdo silenciosamente nem anuncia conclusão segura quando a DT-05 não a confirmou. Conflitos externos continuam sujeitos às opções da **DT-02**, não a uma sobrescrita forçada. Encerramento imposto pelo sistema operacional não é controlável com garantia.

**DT-02 aprovada (alternativa B):** se uma nota aberta tiver sido modificada, substituída, removida ou renomeada externamente, o Oak Note **bloqueia qualquer sobrescrita automática** do caminho afetado e sinaliza conflito. Oferece três ações: **recarregar a versão externa** (confirmando antes de descartar edições locais); **salvar a edição local como nova nota** (respeitando a sugestão editável de nome da DT-01 e sem substituir a versão externa); ou **cancelar**, mantendo o buffer e o conflito pendentes. Nenhum desses caminhos pode perder silenciosamente alterações.

**DT-09 aprovada (alternativa B):** manter em memória, ao carregar cada nota, um **fingerprint com metadados e hash do conteúdo**, a ser comparado com o estado atual antes de salvar/substituir, renomear, mover para a lixeira ou realizar etapas destrutivas da migração. Identidade alterada, arquivo ausente ou conteúdo divergente impedem a operação automática, acionando a DT-02 quando pertinente. A verificação ocorre sob demanda, **sem monitoramento contínuo obrigatório**; não oferece prevenção absoluta de mudanças concorrentes entre conferência e operação (TOCTOU). Não persistir hashes no Markdown ou em índice de notas.

A substituição atômica não significa garantia absoluta contra perda de dados em falha de hardware ou energia, nem substitui backups.

### 4.3. Preferências e estados

**ADR-04:** utilizar um único esquema GSettings, com chaves separadas logicamente para:
- Preferências: tema, transparência, visibilidade da barra lateral e diretório de armazenamento selecionado;
- Estado operacional: dimensões, maximização e última nota selecionada.

O conteúdo das notas não fica no GSettings. A posição absoluta da janela não será persistida nem controlada no Wayland.

### 4.4. Estilo e sobreposição

**ADR-07:** todos os **fundos da janela principal** — cabeçalho, editor e barra lateral — devem acompanhar a transparência configurada; os textos, ícones e controles permanecem opacos. Popovers, diálogos e janela de preferências não estão automaticamente incluídos nessa regra.

O GTK não promete manter a janela sobre outras por API própria. A sobreposição é uma ação manual do usuário pelo ambiente GNOME e está sujeita ao comportamento do compositor.

## 5. Requisitos funcionais (RF)

Os critérios abaixo são condições de aceitação do produto. Não prescrevem APIs nem implementação específica.

### 5.1. Notas, edição e pesquisa

**RF-01 — Criar nota.** Permitir criar uma nova nota a partir da janela principal.  
**Aceite:** a nota criada aparece na lista, torna-se selecionável/editável e é persistida como arquivo `.md` no diretório ativo com nome válido; se o título conflitar, o aplicativo **sugere nome disponível** (por exemplo, `Ideias (2)`), permitindo aceitar ou escolher outro; nenhuma nota existente é sobrescrita.

**RF-02 — Listar notas.** Mostrar as notas gerenciadas pelo aplicativo.  
**Aceite:** arquivos `.md` válidos do diretório de notas ativo (Documentos por padrão, ou outro selecionado) são apresentados com títulos derivados de seus nomes; arquivos desconhecidos não são alterados apenas pela listagem. **DT-19:** ao primeiro acesso a pasta com arquivos `.md` preexistentes diretamente na raiz, informar que eles serão reconhecidos como notas e solicitar confirmação antes de listá-los/abri-los; permitir aceitar ou escolher outra pasta. Não percorrer subdiretórios, alterar nem importar arquivos por causa da identificação. O mecanismo de lembrar a confirmação por pasta será refinado.

**RF-03 — Selecionar nota.** Alternar entre notas da lista.  
**Aceite:** selecionar uma nota carrega seu conteúdo no editor; antes de abandonar a anterior, o aplicativo tenta concluir as gravações pendentes. Se a gravação falhar ou sua durabilidade for incerta, aplica as escolhas da **DT-06**; cancelar mantém a nota atual e seu texto. Conflitos externos seguem a **DT-02**; nenhuma troca descarta edição silenciosamente.

**RF-04 — Editar texto Markdown.** Permitir inserir, alterar, copiar, selecionar e colar texto bruto.  
**Aceite:** caracteres Unicode, quebras de linha e sintaxe Markdown são preservados no conteúdo salvo; não há exigência de renderização nem editor visual de formatação.

**RF-05 — Renomear nota.** Alterar o título da nota e seu nome físico, segundo regras de nomes.  
**Aceite:** após renomeação bem-sucedida a lista e a seleção refletem o novo nome; havendo colisão, é sugerido um nome disponível, editável pelo usuário; nomes inválidos e falhas não sobrescrevem outros arquivos nem eliminam o conteúdo anterior.

**RF-06 — Excluir nota.** Oferecer exclusão mediante confirmação, **movendo o arquivo para a lixeira** conforme DT-03 (alternativa A).  
**Aceite:** cancelar mantém a nota intacta; confirmar, após tratar alterações pendentes e verificar que o arquivo ainda corresponde à nota selecionada, solicita sua movimentação para a lixeira. Somente após confirmação de sucesso da operação a nota deixa a lista. Se a lixeira estiver indisponível, a operação não for suportada ou falhar, o aplicativo informa o erro, mantém a nota e **não executa exclusão definitiva como alternativa automática**. Arquivos externos modificados desde a leitura são tratados conforme DT-02; nenhuma outra nota é excluída.

**RF-07 — Pesquisar por título.** Filtrar a lista pelo título.  
**Aceite:** uma pesquisa mostra apenas notas correspondentes ao título; conteúdo textual que não esteja no título não é pesquisado; limpar a pesquisa restaura a listagem.

**RF-08 — Ordenar notas.** Mostrar as notas da mais recentemente modificada para a menos recente.  
**Aceite:** a listagem usa os metadados de modificação disponíveis no filesystem; alterações salvas podem mudar a posição da nota; empates e metadados indisponíveis têm tratamento determinístico a definir.

### 5.2. Salvamento, conflitos e diagnóstico

**RF-09 — Autosave.** Iniciar salvamento depois de um intervalo sem edição.  
**Aceite:** alterações disparam um salvamento sem comando manual obrigatório; o intervalo exato será definido na PREP-02 (700 ms é sugestão, não compromisso fechado).

**RF-10 — Salvar com segurança.** Evitar substituição parcial ou regressão para revisão antiga.  
**Aceite:** operações de gravação da mesma nota são ordenadas; publicar exige escrever um temporário no mesmo diretório, verificar escrita/flush, sincronizar seus dados, substituir atomicamente quando suportado e solicitar sincronização do diretório quando disponível (DT-05). Falha antes da publicação preserva a edição como pendente; falha após a publicação, ausência de suporte ou resultado incerto **não podem ser apresentados como durabilidade confirmada** nem apagar o buffer. Revisões antigas não podem marcar revisões novas como salvas.

**RF-11 — Comunicar estado de salvamento.** Exibir estados compreensíveis, como alteração pendente, salvando, salvo e falha.  
**Aceite:** após falha aparece aviso perceptível, alterações pendentes são preservadas e há possibilidade de nova tentativa; um conflito externo é apresentado de forma distinta de falha de I/O. O estado exibido distingue **publicação no destino** de **durabilidade verificada**, sinalizando sincronização falha, indisponível ou incerta sem alegar conclusão segura (DT-05).

**RF-12 — Proteger alterações ao alternar ou encerrar.** Não descartar dados alterados silenciosamente.  
**Aceite:** ao trocar de nota ou fechar, o aplicativo tenta concluir as gravações pendentes. Se houver erro, resultado desconhecido ou durabilidade não confirmada, apresenta **tentar novamente**, **cancelar troca/fechamento e continuar editando** ou **descartar conscientemente as alterações locais mediante confirmação específica**, sem tratar a revisão como salva com segurança. O descarte é decisão explícita, com aviso do risco; não apaga automaticamente o arquivo de disco nem garante recuperação posterior. Conflitos externos são resolvidos conforme DT-02; ao fechar com várias notas pendentes, cada edição não confirmada exige tratamento, sem perda silenciosa. Encerramento forçado pelo sistema pode não permitir diálogo (DT-06).

**RF-13 — Detectar conflitos externos.** Não sobrescrever modificações feitas fora do Oak Note sem tratamento.  
**Aceite:** ao carregar uma nota, registrar em memória metadados disponíveis e hash do conteúdo (DT-09); antes de operações potencialmente destrutivas, compará-los com o arquivo atual, inclusive quando tamanho e data de modificação coincidirem. Se a nota tiver sido modificada, substituída, removida ou renomeada externamente desde a leitura, **bloquear sobrescrita automática** e oferecer: (1) **recarregar a versão externa**, após confirmação explícita caso haja edição local pendente; (2) **salvar a edição local como nova nota**, com sugestão editável de nome (DT-01), sem sobrescrever a versão externa; (3) **cancelar**, preservando o buffer e o conflito. Falha na verificação não autoriza exclusão ou gravação destrutiva. Nenhuma ação descarta alterações silenciosamente.

**RF-14 — Tratar erros de arquivo.** Comunicar falhas de leitura, gravação, renomeação, exclusão e exportação.  
**Aceite:** o usuário recebe mensagem contextualizada; o aplicativo preserva, sempre que possível, o estado editável e não registra o conteúdo da nota nos logs.

### 5.3. Navegação, janela e aparência

**RF-15 — Barra lateral recolhível.** Permitir mostrar ou ocultar a lista de notas.  
**Aceite:** ao recolher, o editor ganha espaço útil; ao expandir, a navegação retorna sem perder o texto.

**RF-16 — Temas.** Oferecer Sistema, Claro e Escuro.  
**Aceite:** a escolha aplica aparência correspondente sem reiniciar e é restaurada; Sistema acompanha a preferência disponível do GNOME.

**RF-17 — Transparência seletiva.** Disponibilizar controle de transparência global dos fundos da janela principal.  
**Aceite:** cabeçalho, editor e barra lateral têm fundos ajustáveis; textos, ícones e controles mantêm opacidade; o valor é restaurado; não há clique através da janela. A faixa de 0–70% é referência a confirmar com testes de legibilidade.

**RF-18 — Sobreposição manual.** Permitir uso em conjunto com o recurso de janela do GNOME.  
**Aceite:** o produto não bloqueia o procedimento manual validado para manter a janela acima de outras janelas normais e documenta como ativá-lo/desativá-lo. Não promete ativação pelo próprio Oak Note.

**RF-19 — Estado da janela.** Restaurar largura, altura, maximização e última nota quando aplicável.  
**Aceite:** ao reabrir, estado válido é recuperado; se a nota anterior não existir, a aplicação abre sem erro impeditivo; a posição fica a cargo do GNOME.

**RF-20 — Atalhos internos.** Oferecer ações essenciais por teclado.  
**Aceite:** criar nota, pesquisar, recolher a barra lateral e acessar preferências têm atalhos internos documentados e funcionais quando a janela possui foco. Mapeamento sugerido: `Ctrl+N`, `Ctrl+F`, `F9`, `Ctrl+,` e `Esc` (a confirmar quanto a conflitos).

**RF-21 — Acesso rápido via GNOME.** Permitir que o usuário configure externamente um atalho para abrir a aplicação.  
**Aceite:** o aplicativo pode ser iniciado normalmente pelo comando de instalação e a documentação explica a configuração do atalho no GNOME. Não exige registrar um hotkey global.

**RF-22 — Preferências.** Disponibilizar janela nativa com configurações de tema, transparência e diretório de notas, integrada ao estado da aplicação.  
**Aceite:** alterações de preferência refletem-se na interface e persistem via GSettings; o diretório atual é identificado de modo compreensível e pode ser alterado por seletor de pastas; estado operacional não precisa ter controles editáveis.

### 5.4. Dados, idioma e distribuição

**RF-23 — Exportar notas.** Permitir exportar manualmente todas as notas Markdown a um diretório escolhido.  
**Aceite:** arquivos exportados são `.md` legíveis; seleção do destino é explícita; cancelamento não altera as notas originais; conflitos e falhas são comunicados; não há sobrescrita silenciosa.

**RF-24 — Internacionalização.** Disponibilizar interface em português brasileiro e inglês.  
**Aceite:** textos próprios da interface são localizados com gettext; idioma segue a configuração regional disponível, com fallback definido; textos das anotações não são traduzidos.

**RF-25 — Distribuição.** Oferecer instalação por Flatpak e instruções para compilação pelo código-fonte.  
**Aceite:** artefato Flatpak da versão `0.1.0` pode ser instalado, aberto e encerrado no ambiente de referência; Application ID é `io.github.rafaelcarvalho89.oaknote`; publicação exige aprovação e inclui notas de versão.

**RF-26 — Configurar diretório das notas.** Por padrão, utilizar a pasta XDG **Documentos** do usuário e permitir selecionar outro diretório para o conjunto de notas gerenciado.  
**Aceite:** em instalação inicial, o aplicativo reconhece a pasta Documentos configurada pelo sistema; exibe o diretório ativo; utiliza o acesso Flatpak limitado a XDG Documentos e permite selecionar outra pasta mediante autorização do usuário pelo portal (DT-18). **Antes de ativar uma pasta alternativa, verifica as capacidades obrigatórias de gerenciamento e persistência do acesso**; se não forem comprovadas, mantém a pasta anterior e comunica o motivo, sem solicitar acesso amplo à `home`. Exibe e persiste a seleção válida e a restaura ao reiniciar; indisponibilidade ou perda posterior da autorização exige reautorização ou nova escolha, sem redirecionamento silencioso. Ao trocar de pasta, **oferece escolha explícita entre manter as notas na origem e migrá-las**, sem movimentação ou remoção silenciosa. **Se a pasta selecionada contiver arquivos `.md` preexistentes na raiz, aplicar a confirmação da DT-19 antes de reconhecê-los como notas.** Na opção **migrar**, copiar sem sobrescrever arquivos existentes, conferir todas as cópias antes de ativar o novo destino e solicitar confirmação explícita antes de remover originais. Revalidar as condições de exclusão e preservar qualquer original cuja remoção não seja segura. Falhas, conflitos e cancelamentos precisam preservar os dados e ser comunicados.

**RF-27 — Reativar instância existente.** Manter uma instância ativa por sessão do Oak Note, com uma janela principal para todas as notas (DT-04).  
**Aceite:** iniciar o Oak Note novamente na mesma sessão reativa a janela já existente, sem outro editor independente; conteúdo e estados de salvamento não são reinicializados ou descartados. Falhas de ativação são comunicadas sem presumir exclusividade sobre outros programas ou sessões.

## 6. Requisitos não funcionais (RNF)

| ID | Requisito | Evidência/critério de verificação |
| --- | --- | --- |
| RNF-01 | Execução nativa no Debian/GNOME/Wayland | Testes manuais no ambiente de referência, incluindo integração com o GNOME |
| RNF-02 | Operação offline | Criar, editar, buscar e salvar notas sem conexão; nenhuma conta exigida |
| RNF-03 | Privacidade | Sem telemetria, sincronização ou envio de conteúdo das notas |
| RNF-04 | Portabilidade | Arquivos Markdown individuais no diretório selecionado, legíveis em editor externo, sem front matter obrigatório |
| RNF-05 | Integridade | Escrita temporária, publicação segura, coordenação interna e serialização de operações, conflitos externos e falhas testados; instância única não equivale a bloqueio global de filesystem |
| RNF-06 | Responsividade | Operações de I/O não congelam o loop principal GTK; verificar manualmente e por testes aplicáveis |
| RNF-07 | Acessibilidade de teclado | Funções essenciais sem mouse, foco visível e navegação consistente |
| RNF-08 | Tecnologias assistivas | Verificação manual básica com Orca de controles e estados |
| RNF-09 | Contraste e legibilidade | Testar texto, seleção, cursor e controles em Claro/Escuro e níveis suportados de transparência |
| RNF-10 | Mínimo privilégio | Flatpak com `--filesystem=xdg-documents` (leitura/escrita dentro de Documentos), demais pastas somente via seleção e autorização explícita pelo portal; não solicitar `home`/`host`, rede ou D-Bus irrestrito como fallback; operações e persistência exigem validação prática (DT-18) |
| RNF-11 | Observabilidade local | Logs estruturados de falha e diagnóstico sem conteúdo das notas, credenciais ou dados sensíveis |
| RNF-12 | Testabilidade | Testes unitários das regras e integração do filesystem, inclusive falhas e conflitos |
| RNF-13 | Integração contínua | GitHub Actions executa Fmt, Clippy e testes para alterações propostas |
| RNF-14 | Distribuição controlada | Versão SemVer, `CHANGELOG.md`, verificação do Flatpak e release aprovada manualmente |
| RNF-15 | Arquitetura proporcional | Organização **feature-based**; um crate Rust; arquitetura modular com limites hexagonais seletivos; aplicar SOLID, YAGNI e DRY de forma contextual, sem interfaces ou camadas artificiais |
| RNF-16 | Preferências persistentes | Um esquema GSettings versionado, distribuído corretamente e testado no Flatpak |

Os RNFs 05 e 06 não estabelecem, nesta etapa, garantias absolutas de durabilidade ou metas numéricas de latência; essas garantias devem ser refinadas e testadas na PREP-02.

## 7. Fluxos principais (casos de uso)

### UC-01 — Primeira anotação
1. Usuário abre o Oak Note.
2. Cria uma nota e atribui um título válido.
3. Digita texto Markdown.
4. Após intervalo sem edição, o aplicativo salva automaticamente **na pasta ativa** (Documentos por padrão).
5. A nota aparece na lista e pode ser reaberta após reiniciar.

**Falhas:** colisão de nome gera sugestão editável; erro de escrita deve ser comunicado, sem sobrescrever outro arquivo.

### UC-02 — Consultar e editar nota existente
1. Usuário pesquisa por título ou escolhe a nota na lista.
2. Conteúdo é carregado no editor.
3. Usuário altera o conteúdo.
4. O salvamento atualiza o arquivo e o estado visual.

**Falhas:** se houver edição externa, o aplicativo bloqueia autosave na nota conflitante e permite recarregar com confirmação, salvar a edição local como nova nota ou cancelar mantendo a edição; nenhuma alternativa sobrescreve a versão externa silenciosamente.

### UC-03 — Renomear ou excluir
1. Usuário seleciona uma nota.
2. Renomeia ou solicita exclusão.
3. Em caso de exclusão, o aplicativo verifica alterações pendentes, conflitos externos e se o arquivo ainda corresponde à nota selecionada.
4. O usuário confirma ou cancela o envio **para a lixeira**.
5. Após confirmação do resultado bem-sucedido da operação, a lista e a seleção são atualizadas. Em caso de falha, a nota permanece disponível e um erro é apresentado.

**Falhas:** nome inválido, destino ocupado, alteração externa, lixeira indisponível ou erro de I/O não podem remover outra nota nem perder conteúdo pendente. **Não oferecer exclusão definitiva como fallback (DT-03).**

### UC-04 — Usar a janela enquanto trabalha
1. Usuário abre a nota desejada.
2. Recolhe a barra lateral, se quiser.
3. Ajusta tema e transparência.
4. Ativa manualmente “Sempre no topo” pelo GNOME, quando disponível.
5. Interage com outro aplicativo enquanto mantém o Oak Note visível.

**Limite:** o próprio aplicativo não controla programaticamente a sobreposição.

### UC-05 — Fechar e restaurar
1. Usuário solicita fechar a janela; o aplicativo identifica notas com gravações pendentes, falhas, conflitos ou durabilidade incerta.
2. Tenta concluir as gravações pendentes e verifica a confirmação do resultado conforme DT-05.
3. Em caso de erro ou durabilidade incerta, oferece tentar novamente, cancelar o fechamento para continuar editando ou descartar alterações **mediante confirmação específica** (DT-06). Se houver conflito externo, aplicar DT-02 antes de considerar a edição resolvida.
4. Apenas após resolver os estados pendentes ou confirmar o descarte explicitamente, conclui o fechamento normal; não equipara publicação incerta a salvamento seguro.
5. Ao reabrir, restaura preferências, janela e última nota selecionada quando existente, **sem prometer recuperação de edições descartadas ou de conteúdo perdido em encerramento forçado**.

### UC-06 — Exportar dados
1. Usuário solicita exportação de todas as notas.
2. Escolhe destino no seletor nativo.
3. Aplicação gera cópias Markdown ou informa conflitos/falhas.
4. Arquivos originais permanecem intactos.

### UC-07 — Configurar pasta de notas
1. Usuário acessa Preferências e consulta a pasta de armazenamento ativa.
2. Seleciona outro diretório usando seletor de pastas do sistema.
3. O aplicativo valida disponibilidade e autorização por portal para a nova pasta e verifica operações de gerenciamento necessárias; se alguma capacidade obrigatória não funcionar, explica a limitação, conserva o diretório anterior e não amplia as permissões do Flatpak (DT-18).
4. Se o novo diretório já possuir arquivos `.md` na raiz e for seu primeiro acesso, informa que eles serão reconhecidos como notas e exige confirmação ou a escolha de outra pasta (DT-19).
5. Apresenta opção explícita de **manter as notas na pasta anterior** ou **migrá-las para a pasta selecionada**.
6. Ao escolher manter, preserva integralmente os arquivos de origem e passa a utilizar a pasta de destino.
7. Ao escolher migrar, conclui as gravações pendentes, verifica conflitos e **copia todas as notas** para o destino sem sobrescrever arquivos existentes.
8. Verifica a integridade de todas as cópias; somente após êxito **ativa e persiste** a nova pasta como destino de notas.
9. Solicita **confirmação explícita adicional** para remover os arquivos originais, revalidando a segurança da operação; nenhuma remoção acontece automaticamente.
10. Se a cópia ou a validação falhar, a pasta anterior permanece ativa e os originais são preservados; eventuais cópias parciais são identificadas e tratadas com segurança.
11. Se a remoção de originais falhar após a ativação da nova pasta, informa o resultado parcial e preserva os arquivos remanescentes; não assume nem anuncia limpeza concluída.

**Pendente na PREP-02:** detalhamento operacional da cópia/validação, novas edições ou alterações externas durante a transferência, remoção parcial dos originais, retomada após interrupção e limpeza de cópias incompletas. As etapas de ativação e confirmação de remoção estão aprovadas pela DT-20.

### UC-08 — Nova ativação da aplicação
1. Com uma nota aberta no Oak Note, o usuário tenta iniciar a aplicação novamente na mesma sessão.
2. A nova ativação é encaminhada à instância em execução.
3. A janela existente é apresentada sem novo editor independente e sem perder conteúdo ou estados de salvamento.

**Limite:** outros aplicativos e sessões podem modificar os mesmos arquivos; DT-02 permanece aplicável.

## 8. Estados da experiência

Os seguintes estados precisam ser representados de maneira legível e acessível:
- **Sem notas:** oferecer criação da primeira nota;
- **Nota selecionada:** editor pronto para leitura ou edição;
- **Alteração pendente:** conteúdo em memória ainda não confirmado em disco;
- **Salvando:** gravação em andamento;
- **Salvo:** revisão atual efetivamente persistida;
- **Falha:** conteúdo pendente preservado e opção de nova tentativa;
- **Conflito externo:** operação potencialmente destrutiva bloqueada até resolução;
- **Pesquisa sem resultados:** informar ausência de títulos correspondentes;
- **Exportação:** distinguir conclusão, cancelamento e resultado parcial.

A apresentação exata (toast, indicador, diálogo ou banner) será definida na especificação técnica e no design da interface, respeitando padrões GNOME.

## 9. Critérios de aceitação transversais

**CA-01 — Ciclo de vida:** criar uma nota, editar e fechar; reabrir e encontrar conteúdo persistido, sem ação manual de salvar.

**CA-02 — Proteção contra colisões:** criar ou renomear para nome ocupado oferece sugestão editável e não altera o conteúdo da nota existente.

**CA-03 — Falha de gravação:** simular erro de I/O e comprovar que o aplicativo não exibe estado “salvo” para revisão não publicada, preserva o texto em memória e permite nova tentativa. Falhas **após** a publicação precisam ser distinguidas de falhas anteriores, sem alegar durabilidade confirmada.

**CA-04 — Concorrência:** provocar alterações em sequência e verificar que gravações atrasadas não substituem revisões mais recentes.

**CA-05 — Alteração externa:** modificar, remover ou substituir uma nota fora do Oak Note durante sua edição; confirmar bloqueio do autosave no caminho afetado e apresentação das três ações da DT-02. Recarregar não descarta alterações locais sem confirmação; salvar como nova nota respeita DT-01 e preserva a versão externa; cancelar mantém a edição e o conflito pendentes.

**CA-20 — Fingerprint de alterações externas (DT-09):** carregar arquivo e comparar metadados e hash sob demanda antes de salvar, renomear, enviar à lixeira e remover originais em migração. Detectar conteúdo alterado com **mesmo tamanho e mtime**, substituição do arquivo por outro, remoção e erro de leitura, sem operação destrutiva automática. Testar que o fingerprint é atualizado após gravação confirmada da revisão correta, que a edição permanece em memória no conflito e que a verificação não é apresentada como bloqueio atômico contra outros processos.

**CA-21 — Permissões Flatpak e pastas alternativas (DT-18):** validar acesso à pasta XDG Documentos sem acesso irrestrito à `home`; escolher uma pasta alternativa via portal, verificar listagem, leitura, criação, escrita, renomeação, salvamento seguro com temporário/sync quando suportado, lixeira e persistência da autorização após reiniciar. Testar permissão negada, pasta inacessível ou desmontada, destino via FUSE, recurso não suportado e revogação de autorização: comunicar o problema, preservar notas e manter o destino anterior ao impedir uma troca. Não ampliar permissões silenciosamente.

**CA-06 — Arquivos portáveis:** inspecionar arquivos criados/renomeados em editor externo; confirmar Markdown puro e título coerente com o nome físico.

**CA-16 — Exclusão pela lixeira:** cancelar exclusão preserva arquivo e lista; confirmar move a nota para a lixeira quando suportado; a lista é atualizada somente após sucesso. Simular lixeira indisponível, acesso negado e erro de operação: o arquivo permanece no local e **nunca** há exclusão definitiva substitutiva. Antes da operação, verificar edição pendente e alteração externa sem apagar revisão inesperada.

**CA-17 — Instância única e concorrência:** iniciar o Oak Note repetidamente na mesma sessão e confirmar reativação da janela existente, sem segundo editor independente e sem perder alterações pendentes. Provocar operações internas simultâneas na mesma nota e verificar serialização, ausência de publicação obsoleta e integridade. Modificar externamente a nota e confirmar que a DT-02 continua aplicável.

**CA-18 — Sincronização e durabilidade (DT-05):** verificar escrita no temporário, sincronização do arquivo, substituição e sincronização do diretório quando suportada. Simular falhas em cada etapa, incluindo **falha após a substituição**; confirmar que a UI distingue publicação de durabilidade não confirmada e preserva o buffer para recuperação segura. Testar filesystem/portal que não ofereçam todas as garantias, sem prometer recuperação absoluta após queda de energia.

**CA-19 — Troca e encerramento seguros (DT-06):** com nota alterada, tentar trocar ou fechar e verificar tentativa de salvar antes de prosseguir. Simular erro antes da publicação, resultado incerto e falha de sync após publicação: exibir **tentar novamente**, **cancelar** ou **descartar com confirmação específica**, preservando a edição ao cancelar e sem informar falsamente salvamento seguro. Com conflito externo, manter opções da DT-02. Com várias notas pendentes, não concluir fechamento com descarte silencioso. Confirmar que encerrar à força não oferece garantia de recuperação.

**CA-07 — Navegação:** criar várias notas, pesquisar pelo título, ordenar por modificação e recolher/expandir a barra lateral sem perda de edição.

**CA-08 — Aparência:** alternar Sistema/Claro/Escuro, variar transparência e reiniciar; validar persistência e opacidade do conteúdo, inclusive no cabeçalho.

**CA-09 — Integração GNOME:** ativar e desativar manualmente sobreposição enquanto outro aplicativo possui foco; validar operação sem click-through.

**CA-10 — Distribuição:** compilar, instalar e iniciar Flatpak com ID definitivo, inspecionando permissões efetivas.

**CA-11 — Localização e acessibilidade:** percorrer funções essenciais em pt-BR/en-US com teclado, foco visível e teste básico com Orca.

**CA-12 — Exportação:** exportar todas as notas, verificar integridade das cópias e não alterar originais, inclusive em casos de falha/cancelamento.

**CA-13 — Pasta de notas:** em instalação inicial utilizar a pasta XDG Documentos disponível; selecionar outra pasta, reiniciar e verificar persistência, listagem e escrita no novo destino; simular acesso negado e remoção do diretório sem perda silenciosa.

**CA-14 — Escolha de migração:** ao trocar o diretório, escolher **manter** e confirmar que notas anteriores permaneceram intactas; escolher **migrar**, copiar e validar **todas** as notas antes de ativar o destino; não sobrescrever arquivos existentes; solicitar confirmação explícita para remover originais **depois** da ativação e revalidar sua segurança. Cancelamento, falha de I/O, colisão e migração parcial não podem apagar originais silenciosamente. Falha na limpeza posterior deve ser informada sem desativar automaticamente a pasta nova já validada.

**CA-15 — Arquivos Markdown preexistentes:** no primeiro acesso a uma pasta com `.md` na raiz, solicitar confirmação; ao aceitar, listá-los sem alterar seu conteúdo; ao escolher outra pasta, não abri-los nem modificá-los. Diretórios sem `.md` não exigem confirmação; subpastas não são percorridas.

**Nota de escopo:** a DT-03 rege a **exclusão individual solicitada pelo usuário**. A limpeza de arquivos de origem após uma migração confirmada continua sujeita à DT-20, com política operacional própria a ser refinada.

Esses cenários devem ser transformados em testes e checklists detalhados nas próximas etapas.

## 10. Fora do escopo do MVP v0.1.0

- Cadastro, login, serviços em nuvem ou sincronização entre dispositivos;
- Colaboração, compartilhamento em tempo real ou contas;
- Editor WYSIWYG, visualização Markdown formatada obrigatória ou realce avançado;
- Pesquisa por conteúdo, etiquetas, categorias, pastas e favoritos;
- Notas com janelas independentes ou modo compacto separado do recolhimento da barra lateral;
- Controle automático de “Sempre no topo” ou extensão GNOME Shell;
- Atalho global controlado pelo aplicativo;
- Cliques que atravessam a janela;
- Histórico de versões, backups automáticos ou criptografia de conteúdo;
- Importação em lote, perfis de preferência ou temas personalizados;
- Suporte formal e validado a outros ambientes gráficos/distribuições;
- Publicação no Flathub e atualizações automáticas;
- Testes automatizados end-to-end da interface como requisito obrigatório do MVP.

## 11. Marcos e dependências de entrega

As etapas de **planejamento** são:
- **PREP-01:** especificação de produto (este documento);
- **PREP-02:** especificação técnica e arquitetura detalhada;
- **PREP-03:** backlog de histórias INVEST e critérios de aceitação por história;
- **PREP-04:** roadmap e sequência de marcos;
- **PREP-05:** revisão da baseline e critérios de prontidão.

Os **marcos de implementação** aprovados são:
- **M1 — Fundação:** estrutura do projeto principal, integração GTK4/libadwaita, GSettings, gettext e CI;
- **M2 — Núcleo de notas:** gerenciamento de arquivos, edição Markdown, busca e autosave;
- **M3 — Experiência desktop:** navegação, temas, transparência, janela e atalhos;
- **M4 — Confiabilidade:** exportação, erros, privacidade, testes e acessibilidade;
- **M5 — Release:** pacote Flatpak, validação final, documentação e publicação controlada de `0.1.0`.

Esta divisão é orientativa; a alocação definitiva das histórias ocorrerá na PREP-04.

## 12. Questões abertas para refinamento

Os pontos abaixo **não foram decididos** pelas ADRs ou exigem detalhamento de comportamento. Não devem ser assumidos como já aprovados:

| ID | Questão | Impacto | Etapa sugerida |
| --- | --- | --- | --- |
| QA-01 | Regras de normalização de título, nomes reservados, espaços, extensão e caracteres Unicode | Integridade e portabilidade | PREP-02 |
| QA-02 | Detalhes de formato da sugestão de nome alternativo e título inicial (a política de **sugestão editável** já está aprovada na DT-01) | Criação e renomeação | PREP-02 |
| QA-03 | Mecanismo de referência à nota durante renomeação, sem ID persistente | Seleção e concorrência | PREP-02 |
| QA-04 | **DT-05 aprovada:** sincronizar dados do temporário, publicar por substituição atômica quando suportada e sincronizar diretório quando disponível; **DT-06 aprovada:** oferecer retry, cancelar ou descarte confirmado em troca/fechamento. Detalhar debounce, estados incertos e compatibilidade real | Proteção dos dados | PREP-02 |
| QA-05 | **DT-02 aprovada:** recarregar, salvar local como nova nota ou cancelar; **DT-09 aprovada:** comparar metadados + hash em memória antes de operações destrutivas. Refinar algoritmo, fingerprint indisponível, TOCTOU e arquivo ausente/renomeado | Proteção dos dados | PREP-02 |
| QA-06 | **DT-03 aprovada:** enviar nota à lixeira após confirmação; se indisponível ou falhar, informar erro sem excluir definitivamente. Detalhar suporte GIO/Flatpak e validações pré-exclusão | Segurança do usuário | PREP-02 |
| QA-07 | **DT-04 aprovada:** instância única por sessão, ativações encaminhadas à janela existente, operações internas serializadas. Refinar limites entre sessões/instalações, arquivos ausentes, symlinks e concorrência externa | Robustez | PREP-02 |
| QA-08 | Semântica de pesquisa (acentos, maiúsculas e correspondência parcial), empates de ordenação | Consistência de uso | PREP-02 |
| QA-09 | Intervalo e condições de transparência aceitáveis, popovers/diálogos e contraste mínimo | Acessibilidade visual | PREP-02 |
| QA-10 | Dimensões mínimas reais e composição adaptativa | Usabilidade | PREP-02 |
| QA-11 | Política de colisões e resultados parciais na exportação | Proteção das cópias | PREP-02 |
| QA-12 | Detalhes de GSettings, esquema de versão, valores padrão e fallback de idioma | Persistência | PREP-02 |
| QA-13 | Versão final suportada do SDK/runtime Flatpak, permissões e metadados definitivos | Distribuição | PREP-02 |
| QA-14 | Mapeamento final dos atalhos internos e orientação do atalho no GNOME | Acessibilidade | PREP-02 |
| QA-15 | **DT-17/DT-20 aprovadas:** oferecer manter ou migrar; migração por cópia integral e verificação antes de ativar destino, com confirmação adicional antes da remoção. Detalhar recuperação, falhas parciais e concorrência externa | Integridade e experiência | PREP-02 |
| QA-16 | **DT-18 aprovada:** acesso estático só a XDG Documentos e portal para demais pastas; validar capacidades e persistência antes da troca. Detalhar fallback sem ampliar permissões e **DT-21:** lembrar confirmação DT-19 por pasta, sem índice de notas | Acesso a dados | PREP-02 |

## 13. Rastreabilidade e diretrizes aprovadas

| Referência | Decisão aplicável |
| --- | --- |
| ADR-01 | Título como nome físico da nota `.md` |
| ADR-02 | Markdown puro, metadados de filesystem, sem ID interno persistente |
| ADR-03 | Autosave com debounce, temporário, substituição segura, concorrência e conflitos |
| ADR-04 | GSettings em esquema único para preferências e estado |
| ADR-05 | Monólito modular de um crate com traits seletivas; organização **feature-based** do projeto e hexagonal apenas no essencial |
| ADR-06 | `io.github.rafaelcarvalho89.oaknote` e Flatpak com runtime controlado |
| ADR-07 | Transparência de todos os fundos da janela principal, conteúdo opaco |
| DT-01 | Nome duplicado: sugerir nome alternativo e permitir aceitar/editar, sem sobrescrever |
| DT-17 | Troca de pasta: escolher manter notas na origem ou migrá-las, sem movimentação silenciosa |
| DT-19 | Confirmar reconhecimento de `.md` existentes na raiz no primeiro acesso ou selecionar outra pasta; sem recursão |
| DT-20 | Migrar: copiar tudo sem sobrescrever, validar antes de ativar destino, confirmar explicitamente antes de remover originais e verificar segurança |
| DT-02 | Conflito externo: bloquear sobrescrita e oferecer recarregar com confirmação, salvar local como nova nota ou cancelar preservando o buffer |
| DT-03 | Excluir nota: mover para lixeira após confirmação; sem exclusão permanente como fallback |
| DT-04 | Uma instância ativa por sessão, com reativação da janela existente e serialização interna; conflitos externos permanecem protegidos |
| DT-05 | Salvamento reforçado: sync do temporário, substituição atômica quando suportada, sync de diretório quando disponível e distinção entre publicação e durabilidade comprovada |
| DT-06 | Antes de trocar ou encerrar, tentar salvar; em erro ou resultado incerto, oferecer retry, cancelar para editar ou descartar mediante confirmação; conflitos seguem DT-02 |
| DT-09 | Detectar alterações externas com metadados + hash do conteúdo em memória, comparados sob demanda antes de operações destrutivas; sem monitoramento contínuo obrigatório |
| DT-18 | Flatpak: acesso estático somente a XDG Documentos; portal para pastas alternativas, com validação efetiva das operações e autorização persistente; sem ampliação automática de permissões |

As decisões do briefing 01–21 completam o contexto de produto e distribuição. Este documento não substitui os futuros registros detalhados das ADRs em `.docs/architecture/`.

## 14. Aprovação e próxima ação

**Situação atual:** rascunho apresentado para revisão. Não está aprovado como baseline enquanto existirem correções solicitadas ou a revisão final da PREP-01 não tiver ocorrido.

**Condição para encerrar a PREP-01:** confirmar que os 27 RFs, 16 RNFs, 21 critérios transversais e os limites de escopo refletem integralmente o produto pretendido, e identificar as lacunas a resolver na PREP-02.

**Próximo entregável:** `.docs/mvp/technical-specification.md` (PREP-02), somente depois de revisar esta especificação.

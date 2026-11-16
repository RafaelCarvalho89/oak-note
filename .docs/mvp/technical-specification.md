# Oak Note — Especificação Técnica e Arquitetura (MVP v0.1.0)

> **Estado:** RASCUNHO DE REFINAMENTO — PREP-02, pendente de aprovação dos pontos abertos.  
> **Data:** 2026-11-16  
> **Referência de produto:** [PREP-01 — Especificação do Produto](product-specification.md)  
> **Identificador:** `io.github.rafaelcarvalho89.oaknote`  
> **Escopo desta entrega:** somente planejamento/documentação; nenhuma implementação.

## 1. Objetivo, contexto e situação do repositório

Traduzir os requisitos RF-01–RF-27 e RNF-01–RNF-16 da PREP-01 em desenho técnico verificável, mantendo as sete ADRs aprovadas como restrições. Não antecipar código de implementação nem converter propostas ainda pendentes em decisões aprovadas.

Em 2026-11-16, a `main` do repositório `RafaelCarvalho89/oak-note` foi verificada contendo apenas `.gitignore`, `LICENSE` e `README.md`. A POC havia sido retirada da branch principal pelo mantenedor. A documentação nova não deve copiar arquivos experimentais da branch antiga. A base de implementação do MVP deverá ser estabelecida somente em M1.

## 2. Restrições arquiteturais aprovadas (ADRs)

| ADR | Restrição |
| --- | --- |
| 01 | Nome físico do arquivo Markdown derivado do título; renomear título pode alterar caminho. |
| 02 | Markdown puro, sem front matter ou índice auxiliar; título e modificação derivados do filesystem; sem ID interno persistente e sem garantia da data histórica de criação. |
| 03 | Autosave com debounce, arquivo temporário no mesmo diretório, substituição atômica quando suportada, controle de revisões/concorrência e erros explícitos. |
| 04 | Esquema GSettings único, separação lógica entre preferências e estado operacional. |
| 05 | Monólito modular de um crate, domínio independente, abstrações seletivas nas fronteiras úteis; **organização de arquivos feature-based** como diretriz organizacional adicional. |
| 06 | Application ID definitivo `io.github.rafaelcarvalho89.oaknote`, Flatpak com runtime GNOME versionado e atualizações controladas. |
| 07 | Transparência em **todos os fundos da janela principal**, incluindo cabeçalho, editor e barra lateral; textos e controles opacos. |

**Baseline adicional:** Debian/GNOME/Wayland; Rust + GTK4 + libadwaita; POC validada manualmente pelo responsável; sobreposição exclusivamente manual pelo GNOME; notas offline em **Documentos por padrão** com pasta configurável; localização pt-BR/en-US; GitHub Releases para Flatpak; **DT-04 aprovada: instância única por sessão do aplicativo, com reativação da janela existente**. **DT-01 aprovada:** sugerir nome alternativo editável em colisões. **Diretriz transversal:** os projetos de produção seguem organização feature-based; aplicar princípios hexagonais, SOLID, YAGNI e DRY somente de forma útil e compatível com o contexto.

## 3. Organização feature-based e arquitetura modular

A **organização dos arquivos é feature-based**: agrupar código por capacidade do produto, não por categoria técnica global. A arquitetura define **dependências, contratos e responsabilidades** entre partes do sistema; não é sinônimo de uma árvore de diretórios `domain/application/infrastructure/ui`.

### 3.1. Estrutura de referência (não implantada)

```text
oak-note/
├── .docs/
│   ├── architecture/
│   └── mvp/
│       ├── product-specification.md
│       └── technical-specification.md
├── src/
│   ├── main.rs                   # inicialização
│   ├── app.rs                    # composição da aplicação
│   ├── features/
│   │   ├── notes/
│   │   │   ├── mod.rs
│   │   │   ├── model.rs         # regras essenciais da nota
│   │   │   ├── service.rs       # casos de uso
│   │   │   ├── repository.rs    # porta seletiva, se necessária
│   │   │   ├── filesystem.rs    # adaptador de arquivos
│   │   │   ├── save.rs          # coordenação de salvamento
│   │   │   └── ui/              # editor e lista
│   │   ├── storage/
│   │   │   ├── mod.rs
│   │   │   └── settings.rs      # pasta de notas selecionada
│   │   ├── appearance/
│   │   │   ├── mod.rs
│   │   │   └── ui.rs            # tema/transparência
│   │   └── export/
│   │       ├── mod.rs
│   │       └── service.rs
│   └── platform/                # somente infraestrutura reutilizada
│       ├── mod.rs
│       └── settings.rs          # acesso compartilhado a GSettings, se útil
├── data/                        # schema, desktop, AppStream, traduções
├── flatpak/
├── tests/
├── Cargo.toml
└── Cargo.lock
```

A estrutura é uma **referência mínima adaptável**, não uma obrigação de criar todos os arquivos, features, traits ou pastas no primeiro commit. Uma capacidade pequena pode residir em um único módulo; dividir por submódulos apenas quando necessário. Por exemplo, gerenciamento de notas, edição, listagem e pesquisa podem permanecer juntos em `features/notes/`.

### 3.2. Limites arquiteturais, sem imposição de pastas por camada

- **Núcleo de domínio:** modelo de nota e regras relevantes; independente de GTK4, libadwaita e operações concretas de filesystem.
- **Casos de uso:** coordenam edição, seleção, salvamento, renomeação, exclusão e troca do diretório ativo.
- **Adaptadores:** filesystem, GSettings e portals GTK/GIO implementam somente contratos necessários.
- **Interface:** widgets invocam casos de uso e mostram resultados; não gravam notas diretamente.
- **Composição:** `main.rs`/`app.rs` ligam implementações concretas aos consumidores.

Essas responsabilidades podem estar **dentro da mesma feature**, separadas em módulos apenas quando isso simplificar a implementação. O núcleo não importa bibliotecas de UI. Interfaces abstratas são empregadas apenas quando testabilidade, substituição real ou controle de efeitos as justificarem.

### 3.3. Hexagonal, SOLID, YAGNI e DRY: critérios de aplicação

1. **Hexagonal no essencial:** usar uma porta como `NoteRepository` para isolar filesystem do salvamento e dos testes; não reproduzir mecanicamente portas/adaptadores para cada widget ou configuração.
2. **SOLID contextual:** adotar responsabilidades coesas, dependências na direção correta e contratos pequenos; não multiplicar tipos para perseguir abstrações especulativas.
3. **YAGNI:** não criar extensões para bancos de dados, sincronização ou outros provedores ainda fora do escopo.
4. **DRY:** eliminar duplicação de regras de domínio e persistência, mas permitir repetição local simples quando abstraí-la acrescentaria acoplamento.
5. **Modularidade prática:** evitar dependências circulares entre features; promover código a `platform/` somente quando compartilhamento concreto justificar.
6. **Um único crate:** preservar decisão de monólito modular Rust, sem workspace ou multi-crate obrigatório.

### 3.4. Contrato de armazenamento seletivo

Uma trait `NoteRepository` é candidata, com operações orientadas à pasta ativa:

```text
list_notes(directory) -> Result<list<NoteMetadata>, RepositoryError>
read_note(directory, path) -> Result<NoteSnapshot, RepositoryError>
create_note(directory, title, content) -> Result<NoteSnapshot, RepositoryError>
replace_if_unchanged(directory, path, expected_fingerprint, content) -> Result<NewFingerprint, RepositoryError>
rename_if_available(directory, source, target, expected_fingerprint) -> Result<NoteSnapshot, RepositoryError>
delete_note(directory, path, expected_fingerprint) -> Result<(), RepositoryError>
export_notes(directory, destination) -> Result<ExportReport, RepositoryError>
```

**Pseudocontrato, não API Rust final.** `expected_fingerprint` é um estado observado, não identidade persistente. A referência a `directory` pode ser capturada por um objeto de repositório em vez de parâmetro repetido; decidir durante implementação sem romper as garantias. Não é obrigatório criar trait para configurações, seletor de diretório ou aparência.

## 4. Modelo de dados e convenções

### 4.1. Modelo lógico

```text
NoteSnapshot
  title: String                   # derivado do nome do arquivo
  relative_path: Path             # referência operacional, mutável
  content: String                 # corpo Markdown puro
  last_modified: Option<Time>     # filesystem
  fingerprint: FileFingerprint    # concorrência otimista, não persistente
  revision: u64                   # somente memória da aplicação
```

**Nota:** não existe `note_id` persistente; `revision` é somente estado efêmero interno da sessão e não é gravado no Markdown.

### 4.2. Diretório de armazenamento configurável

**Decisão de produto aprovada:** a pasta padrão de notas é o diretório **Documentos** do usuário, resolvido como diretório especial XDG, sem caminho hardcoded. Por exemplo, pode ser `/home/<USER>/Documentos` em uma sessão pt-BR. **Não criar implicitamente subpasta Oak Note**: o diretório ativo é a própria pasta Documentos, a menos que o usuário escolha outra.

- Consultar a localização de `GLib.UserDirectory.DIRECTORY_DOCUMENTS` ou API XDG equivalente, verificando retorno ausente, caminho inexistente e permissões.
- Permitir selecionar um diretório alternativo nas Preferências, usando seletor de pastas integrado ao GTK/GIO e ao portal onde necessário.
- Persistir a seleção no esquema único de GSettings (ADR-04), sem salvar o conteúdo das notas no GSettings.
- Operações de listar, criar, editar, renomear, apagar e exportar consultam explicitamente a pasta ativa.
- Não considerar a área privada do Flatpak o destino padrão das notas; dados operacionais internos e configurações podem continuar em áreas próprias.
- Se o diretório padrão ou escolhido estiver indisponível, informar e solicitar intervenção, **sem redirecionar silenciosamente** e sem apagar/transportar arquivos.
- A pasta Documentos pode conter `.md` criados fora do Oak Note. A política aprovada na **DT-19** exige confirmação inicial para reconhecer arquivos `.md` preexistentes na raiz, sem acessar subpastas ou modificar arquivos apenas pelo reconhecimento.
- **DT-19 aprovada (alternativa B):** ao primeiro acesso a pasta com arquivos `.md` **preexistentes na raiz** e sem confirmação anterior válida, informar que serão reconhecidos como notas e solicitar confirmação. Oferecer **aceitar** ou **selecionar outra pasta**; antes da confirmação, não apresentar/abrir o conteúdo como nota. A detecção não percorre subdiretórios, não importa arquivos nem os altera. Pastas sem arquivos `.md` na raiz não exigem confirmação. **DT-21 aprovada (alternativa A):** persistir o consentimento de reconhecimento **por diretório** no esquema GSettings único, para evitar perguntar novamente ao revisitar uma pasta validamente reconhecida; esse consentimento não concede acesso via portal nem registra notas individualmente.
- **DT-17 aprovada (alternativa C):** ao trocar a pasta ativa, oferecer opção explícita de **manter notas na origem** ou **migrá-las para a pasta selecionada**. Manter preserva os arquivos na origem; migrar é regido pela **DT-20 aprovada (alternativa B)**: copiar integralmente, validar cópias, ativar destino após validação completa e exigir confirmação explícita adicional, com verificações de segurança, antes de remover originais. A migração entre pastas não é uma única transação atômica; falhas parciais e recuperação exigem tratamento.

**DT-18 aprovada (alternativa A):** conceder ao Flatpak acesso estático de leitura/escrita **somente ao diretório XDG Documentos** (`--filesystem=xdg-documents`; sem `:ro`), reconhecendo que isso expõe toda a pasta Documentos ao aplicativo. Para locais alternativos, solicitar seleção explícita pelo **FileChooser Portal**, sem adicionar `--filesystem=home`, `host` ou permissões arbitrárias como fallback. Antes de ativar uma pasta alternativa, verificar capacidades indispensáveis de gerenciamento com segurança; validar continuidade do acesso entre reinícios. Se o acesso ou operação necessária falhar, explicar o problema e preservar o destino anterior. A escolha é arquitetural, **não validação técnica da capacidade do portal**. O caminho mediado por Documents Portal/FUSE pode não suportar todas as semânticas de filesystem exigidas (DT-03/05/09/20).

Referências oficiais:
- https://docs.gtk.org/glib/func.get_user_special_dir.html
- https://docs.flatpak.org/en/latest/sandbox-permissions.html
- https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html

### 4.3. Título e nome físico (QA-01/QA-02)

**Proposta para avaliação:** derivar nome a partir de título legível, preservando Unicode válido, normalizando espaços periféricos, impedindo `/`, NUL, `.`, `..`, títulos vazios e nomes reservados pela política do produto. Manter extensão `.md` minúscula como convenção; preservar legibilidade em vez de sempre aplicar slug ASCII.

**DT-01 aprovada — alternativa B:** em caso de colisão, **sugerir nome alternativo disponível** (ex.: `Ideias (2).md`), com possibilidade de aceitar ou editar antes de confirmar. **Nunca substituir arquivo existente**. A normalização de caracteres, extensão, casefold, números de sufixo e o nome inicial sugerido ainda exigem detalhamento.

Listar somente arquivos regulares apropriados **na raiz** do diretório ativo; sem busca recursiva. Condicionar o reconhecimento de arquivos `.md` preexistentes sem consentimento válido à confirmação do usuário (DT-19/DT-21). Definir política sobre symlinks e arquivos não regulares antes do desenvolvimento; não presumir que entradas externas sejam seguras.

### 4.4. Fonte de verdade e reordenação

O conteúdo do `.md` é a fonte de verdade da nota. A data de modificação do filesystem determina a ordem decrescente; empates devem empregar título/caminho para produzir ordem estável, sujeito a decisão final. Metadados em cache na memória são descartáveis e sincronizados com resultados reais do repositório.

## 5. Contrato de salvamento e integridade (prioridade alta)

### 5.1. Estados internos propostos

```text
Clean              # revisão correspondente com publicação e sincronização verificadas conforme capacidades do sistema
Dirty              # alteração em memória não publicada
Scheduled          # debounce aguardando
Saving             # escrita/sincronização/publicação em andamento
PublishedUnverified # destino pode ter sido atualizado, mas durabilidade não foi confirmada
Error              # falha de gravação; preservar buffer e evidência do estágio atingido
Conflict           # alteração externa impede publicação automática
```

O usuário precisa distinguir **pendente**, **salvando**, **salvo com sincronização confirmada conforme suporte**, **publicação com durabilidade não verificada**, **falha** e **conflito**. Estados podem ser simplificados na UI sem apresentar conclusão segura quando a sincronização falhou, não é suportada ou teve resultado incerto. `PublishedUnverified` é um estado lógico proposto, não exige esse nome literal no código.

### 5.2. Sequência nominal

1. Editor emite alteração e o `SaveCoordinator` incrementa `revision` da nota ativa.
2. Debounce cancela/substitui agendamentos anteriores; **700 ms é candidato inicial**, não valor fechado pela DT-05.
3. Ao disparar, coleta snapshot imutável do conteúdo e da revisão e coordena escrita em sequência por nota (DT-04).
4. Confere estado esperado em disco antes de publicar, preservando a proteção de alterações externas (DT-02).
5. Escreve **arquivo temporário no mesmo diretório** e verifica sucesso integral da escrita e flush de buffers.
6. **DT-05 aprovada (B):** solicita sincronização dos dados do temporário (ex.: operação equivalente a `fsync`/`sync_all`, conforme a plataforma), verificando resultado antes da publicação.
7. Publica usando substituição atômica por rename **quando suportada pelo sistema e pelo diretório**, com revalidação necessária para evitar sobrescrita indevida.
8. Solicita sincronização do **diretório de destino quando disponível** para reforçar persistência da alteração de entrada de diretório; tratar falha, indisponibilidade ou resultado desconhecido explicitamente.
9. Registra separadamente a revisão **publicada** e o nível de **durabilidade verificado**. Se surgiram edições mais novas, elas continuam pendentes e não são marcadas como salvas por resultado antigo.
10. Comunica o resultado ao contexto GTK sem bloquear o loop principal.

### 5.3. Garantias e limites

- **DT-04:** uma instância ativa por sessão não substitui coordenação interna: somente uma operação mutável por nota/recurso deve publicar por vez, incluindo escrita, renomeação, envio à lixeira, troca de pasta e migração quando compartilharem arquivos; revisões e destinos esperados são revalidados antes de confirmar resultados.
- Escrita atrasada nunca marca como salva uma revisão mais nova.
- Falhas preservam o buffer e deixam a revisão não salva; **não há garantia de recuperar memória após crash** sem journal/snapshot, que estão fora do MVP.
- Evitar sobrescrever outro arquivo em caso de colisão de nome.
- Verificar mudanças externas antes de operação destrutiva; **checagem separada e posterior rename não são, por si só, garantia contra TOCTOU**. PREP-02 deve escolher e testar uma política segura diante de múltiplos processos; não prometer segurança absoluta sem mecanismos do SO.
- **DT-05 aprovada (alternativa B):** exigir sincronização dos dados do arquivo temporário antes de publicar; solicitar sincronização da entrada do diretório após substituição **quando suportado**. Não confundir `flush` de buffers com garantia de persistência em mídia.
- **Resultados por etapa:** se falhar antes da publicação, manter conteúdo em memória e não reportar revisão salva. Se houver falha **após** a substituição (inclusive na sincronização do diretório), o caminho de destino pode já conter a nova revisão, mas a **durabilidade não é verificada**: não executar rollback nem repetir cegamente a substituição; reconciliar observação do disco, revisão e fingerprint antes de tentar novamente.
- **Sem suporte/sem garantia:** filesystem, montagem, hardware, diretório via portal Flatpak ou operação assíncrona podem não oferecer confirmação suficiente de sync ou rename atômico. Registrar e expor a limitação de forma proporcional, sem anunciar durabilidade confirmada. A política exata de UI para degradação fica para refinamento, mas não pode esconder erro ou incerteza.
- Publicação atômica e sincronização **não garantem** recuperação absoluta diante de desligamento, falha de hardware, processo encerrado à força ou efeitos externos; a confiabilidade real depende das capacidades do ambiente. Não adicionar journal de recuperação como obrigação da DT-05.
- A implementação não deve deixar temporários órfãos com conteúdo sensível, sempre que limpeza segura seja possível. Eventuais resíduos após crash devem ser tratados sem arriscar notas válidas.
- Uma aplicação secundária ou edição por terceiro não autoriza sobrescrita silenciosa.

### 5.3.1. Resultado de persistência e recuperação após falhas (DT-05)

| Resultado observado | Estado lógico | Resposta exigida |
| --- | --- | --- |
| Falha antes de publicar | Edição pendente/erro | Preservar buffer, comunicar falha e permitir retentativa controlada |
| Publicação confirmada e sincronizações aplicáveis concluídas | Revisão publicada com durabilidade verificada segundo os mecanismos disponíveis | Confirmar revisão correspondente, ressalvados os limites do filesystem |
| Substituição ocorreu, mas sync do diretório falhou ou ficou indisponível | `PublishedUnverified` | Informar que o destino pode ter sido atualizado, mas durabilidade não foi verificada; manter buffer e reconciliar antes de repetir |
| Resultado de publicação desconhecido | Estado incerto/erro | Não assumir que salvou nem que não salvou; inspecionar o destino de forma segura e preservar texto |
| Alteração externa concorrente | `Conflict` | Bloquear publicação e seguir DT-02 |

O contrato entre `SaveCoordinator` e adaptador de filesystem precisa retornar **estágio atingido e evidências**, não só `Result<(), Error>`. Erros de sync após publicação devem ser tratados de modo distinto de falha antes da publicação, inclusive na troca de nota/fechamento (DT-06). A sincronização reforçada deve ser avaliada no ambiente de referência sem bloquear a thread GTK.

### 5.4. Troca de nota, encerramento e erros — DT-06 aprovada (alternativa B)

**Política decidida:** ao trocar de nota ou solicitar fechamento normal da janela, o Oak Note tenta concluir as gravações pendentes **antes** de abandonar o contexto editado. Em falha ou durabilidade não confirmada, oferece **Tentar novamente**, **Cancelar (continuar editando)** ou **Descartar alterações locais (com confirmação específica)**. Em estado `Conflict`, a resolução continua regida pela **DT-02**, não por sobrescrita automática.

#### 5.4.1. Fluxo de troca de nota

1. Identificar a nota ativa e seu estado efetivo (`Dirty`, `Scheduled`, `Saving`, `Error`, `PublishedUnverified` ou `Conflict`). Se não houver trabalho pendente, trocar normalmente.
2. Para estados com salvamento possível, cancelar o debounce pendente, concluir operações em andamento e solicitar a persistência da revisão atual conforme a **DT-05**, sem bloquear a thread GTK.
3. Somente trocar automaticamente quando a revisão aplicável estiver publicada **com o nível de durabilidade verificado segundo as capacidades disponíveis**. Não tratar `PublishedUnverified` como conclusão segura.
4. Em erro ou resultado incerto, apresentar: **Tentar novamente** (após reconciliação apropriada caso publicação seja desconhecida), **Cancelar** (continuar na nota atual sem perder o buffer) ou **Descartar** (confirmar explicitamente quais edições locais poderão ser perdidas). O descarte não remove nem regrava automaticamente o arquivo em disco.
5. Em `Conflict`, oferecer as alternativas da **DT-02**: recarregar com confirmação, salvar como nova nota ou cancelar mantendo o conflito. A escolha de encerrar/abandonar uma edição não autoriza sobrescrever a versão externa.

#### 5.4.2. Fluxo de fechamento normal

1. Interceptar a solicitação **normal** de fechamento pelo mecanismo de ciclo de vida GTK/GApplication aplicável e coordenar a conclusão de escritas pendentes. Evitar destruir a janela enquanto houver decisão do usuário ou operação de persistência por concluir.
2. Conferir os estados das notas com buffers pendentes, conflitos ou durabilidade incerta, não apenas o estado da nota selecionada. Se tudo estiver concluído de forma verificável, permitir fechamento.
3. Diante de falha ou incerteza, oferecer as três escolhas da DT-06. **Cancelar** mantém a aplicação aberta e os buffers intactos. **Tentar novamente** segue DT-05 e não repete cegamente escrita cuja publicação é incerta. **Descartar** exige confirmação específica, com advertência clara de que as alterações locais podem ser irrecuperáveis.
4. Se houver conflito externo, aplicar a **DT-02**; preservar o buffer até ação explicitamente escolhida. Quando houver várias notas afetadas, não concluir o fechamento com descarte silencioso de nenhuma delas.
5. Depois que todas as notas estiverem resolvidas ou seus descartes tiverem sido confirmados, liberar o fechamento normal. Não anunciar que todas as notas foram salvas quando alguma foi conscientemente descartada ou tinha durabilidade não comprovada.

#### 5.4.3. Integração técnica e limites

- O coordenador de navegação/fechamento, ligado à feature de notas e ao ciclo de vida da janela, consulta os estados por nota e solicita operações ao `SaveCoordinator`; nenhuma gravação deve ocorrer diretamente no widget.
- Bloquear temporariamente a troca/destruição **normal** da janela enquanto a operação/decisão estiver pendente; a escolha precisa ser acessível por teclado e em pt-BR/en-US.
- **Incerteza após publicação:** o arquivo de destino pode já conter a edição, mas sem durabilidade comprovada. O texto e a revisão em memória são preservados enquanto a decisão não foi tomada. Descartar o buffer não executa rollback do que já foi publicado em disco; a UI deve explicar essa distinção.
- A política não inclui journal ou snapshots de recuperação (fora do MVP) e não pode garantir a exibição de confirmação nem preservação do buffer em `SIGKILL`, crash, perda de energia ou encerramento forçado pelo sistema/gerenciador de janelas.
- **Detalhes de interface pendentes:** apresentação de vários buffers pendentes, diálogos/toasts específicos, interação com a reativação da janela (DT-04), e política visual quando sincronização do diretório não for suportada (DT-05). Estes pontos não alteram as três ações aprovadas.

**Critérios de verificação:** troca de nota com `Dirty` e `Saving`, fechamento com mais de uma nota pendente, falha antes/depois da publicação, sincronização não disponível, resultado incerto, tentativa de retry após reconciliação, cancelamento sem perda de buffer, descarte com confirmação e conflito externo (DT-02).

### 5.5. Fingerprints e modificações externas (QA-05/QA-07)

**DT-09 aprovada (alternativa B) — metadados + hash sob demanda.** Ao carregar uma nota, o adaptador de filesystem calcula um fingerprint **efêmero** que combina, quando disponíveis, identificação operacional do arquivo (por exemplo, dispositivo/inode ou identidade equivalente), tamanho, `mtime` e um **hash do conteúdo efetivamente lido**. O fingerprint acompanha o snapshot aberto, **somente em memória**; não é identificador persistente, front matter ou índice de notas.

**Aplicação da política:**
1. Na leitura, associar o fingerprint à revisão do conteúdo carregado; hash e texto devem corresponder ao **mesmo snapshot** da leitura, evitando comparar dados de momentos diferentes quando possível.
2. Antes de **publicar/sobrescrever**, **renomear**, **enviar à lixeira** ou **remover um original após migração**, recalcular e comparar metadados e hash com o estado esperado. A integridade das cópias durante migração (DT-20) também precisa de verificação.
3. Se conteúdo, identidade operacional ou caminho esperado divergir, se o arquivo desaparecer, ou se a verificação não puder ser feita com segurança, **não executar a operação potencialmente destrutiva**. Para uma nota aberta com edição pendente, sinalizar `Conflict` e apresentar as alternativas da DT-02; erro de leitura/permissão deve ser distinguido de divergência comprovada.
4. Depois de uma gravação confirmada, atualizar a referência esperada **somente para a revisão correspondente**. Se a publicação aconteceu mas a durabilidade está incerta (DT-05), preservar evidências da operação e reconciliar disco/buffer antes de nova escrita ou atualização definitiva do estado.
5. Uma divergência de metadados com **conteúdo de hash igual** requer avaliação de identidade e contexto: `mtime` alterado isoladamente não significa, por si só, bytes alterados, mas identidade/caminho inesperado não pode ser ignorado. A política detalhada ficará para testes e refinamento do contrato.

**Sem monitoramento contínuo obrigatório:** a checagem é efetuada **sob demanda nas operações críticas**, inclusive no autosave. Isso não garante aviso imediato quando outro editor modifica um arquivo enquanto o Oak Note está inativo. O cálculo do hash e leitura de disco não devem bloquear o loop GTK.

**Limites verificáveis:** comparar hash reduz falsos negativos de tamanho/`mtime`, mas não elimina colisões de hash nem mudanças entre conferência e publicação (**TOCTOU**). `stat + hash + rename/trash` executados separadamente **não formam operação atômica condicional**. DT-04 controla somente a instância da aplicação; técnicas de proteção adicionais e garantias reais do filesystem/portal precisam ser avaliadas, sem prometer exclusividade de escrita por outros processos. O algoritmo concreto de hash e tratamento de arquivos especiais/symlinks ainda são pontos de implementação/DT-14, sem alterar a decisão aprovada.

**DT-02 aprovada (alternativa B) — resolução manual:** ao detectar que o arquivo aberto mudou, foi substituído, removido ou renomeado externamente desde a leitura, o `SaveCoordinator` suspende a publicação automática **daquela nota** e sinaliza `Conflict` à interface. A resolução oferece:

1. **Recarregar versão externa:** reler a versão atualmente existente do disco, mostrando confirmação explícita antes de descartar qualquer alteração local pendente. Se o arquivo original não estiver disponível, apresentar erro/estado de arquivo ausente; não criar arquivo em seu lugar automaticamente.
2. **Salvar edição local como nova nota:** criar outro arquivo Markdown com nome validado e **sugestão editável conforme DT-01**, usando operação que não sobrescreve destino existente. Preservar o arquivo externo intacto. Somente considerar a edição local salva após sucesso real da criação; se houver falha, conservar buffer e conflito.
3. **Cancelar:** fechar a escolha de resolução sem perder a edição local; permanecer em `Conflict`, manter autosave bloqueado **para o arquivo afetado** e permitir nova tentativa de resolução.

**Garantias:** a detecção de conflito e a criação de nova nota devem verificar o estado do filesystem na operação efetiva; a DT-09 estabelece fingerprint combinado, mas uma checagem antecipada não elimina riscos de TOCTOU, mesmo com instância única (DT-04). A escolha de recarregar não significa mesclagem de textos; não há merge automático no MVP. O comando de salvar como nova nota **não deve salvar sobre o caminho da versão externa**. A interface não pode apresentar `Clean/Salvo` antes da persistência confirmada. Troca de nota e encerramento respeitam RF-12/DT-06 quando existir buffer local conflitado.

**Detalhes ainda abertos:** distinguir renomeação de remoção externa sem rastreamento contínuo, algoritmo e identidade operacional disponíveis no filesystem/portal, concorrência de outros aplicativos ou sessões, controles de saída em estado `Conflict` e erro durante a opção de salvar como nova nota. A DT-04 evita múltiplas instâncias independentes dentro da mesma sessão do aplicativo, mas não elimina os conflitos externos definidos na DT-02.

## 6. Casos de uso e fluxo de operações

| Caso de uso | Camada de aplicação | Infraestrutura | Erro/contingência |
| --- | --- | --- | --- |
| Criar | Validar título, sugerir alternativa editável em colisão, coordenar seleção e salvar | Criação sem sobrescrita na pasta ativa | Nome duplicado, I/O |
| Listar | Ordenar e filtrar metadados, respeitando a confirmação inicial DT-19 | Detecção/enumeração controlada sem mutação | Entrada inválida, recusa ou escolha de outra pasta |
| Selecionar | Proteger edição pendente anterior | Leitura condicionada | Arquivo removido |
| Editar | Atualizar revisão e agendar autosave; em conflito externo, bloquear publicação e solicitar decisão (DT-02) | Nenhuma escrita direta pela UI; verificar estado esperado antes de publicar | Falha de gravação, edição externa ou arquivo ausente |
| Renomear | Coordenar salvamento e referência mutável | Renomeação sem colisão | Conflito externo |
| Excluir | Confirmar, resolver edições pendentes, verificar identidade operacional/fingerprint e aplicar DT-03 | Tentar mover arquivo selecionado para a lixeira; sem fallback destrutivo | Cancelamento, alteração externa, lixeira não suportada e falha de I/O |
| Buscar | Filtrar notas carregadas por título | Não exige índice | Nenhum resultado |
| Exportar | DT-07: coordenar lote, estados pendentes e relatório por arquivo | Copiar `.md` para destino escolhido sem sobrescrita, validar resultados e continuar após erros recuperáveis | Destino ocupado (ignorar), erro individual, acesso perdido, cancelamento ou resultado incerto |
| Alterar diretório | Validar pasta/edições pendentes, oferecer **manter ou migrar** (DT-17), coordenar cópia e validação integral (DT-20) | Seleção via portal, cópia sem sobrescrita, verificação, ativação segura e remoção posterior com confirmação | Pasta indisponível, cancelamento, nomes ocupados, conteúdo divergente, cópia ou limpeza parcial |

### 6.1. Migração entre diretórios — DT-20 aprovada (alternativa B)

A migração é distinta da exportação e da escolha de **manter notas na pasta anterior** (DT-17). A transferência autorizada deve obedecer à ordem:

1. **Preparação:** validar o destino, concluir ou resolver salvamentos pendentes, suspender operações conflitantes da aplicação sobre as notas envolvidas e obter um snapshot do conjunto de arquivos elegíveis da raiz. Aplicar a DT-19 à presença de Markdown preexistente no destino. Interrupções e modificações por outras aplicações não estão controladas apenas por essa suspensão.
2. **Pré-verificação:** identificar colisões de nomes, permissões, falta de espaço e arquivos inacessíveis. Não substituir arquivos existentes. Em caso de impedimento, interromper antes de excluir qualquer original e comunicar opções seguras.
3. **Cópia:** copiar cada arquivo para o destino usando arquivos temporários no próprio destino e publicação sem sobrescrita conforme suportada pelo filesystem/portal. Não presumir que um `rename` entre sistemas de arquivos seja atômico. A publicação segura precisa ser verificada no ambiente Flatpak (DT-18).
4. **Verificação integral:** verificar o conjunto esperado de arquivos e integridade das cópias (por exemplo, bytes/hash, sem confiar apenas em tamanho). Conferir se houve modificações externas na origem durante a transferência; diante de alterações ou resultado desconhecido, não ativar o novo destino automaticamente.
5. **Ativação:** apenas depois da validação completa, registrar o novo diretório ativo e atualizar a interface. Antes disso, a origem continua sendo a fonte de verdade. O tratamento de falha ao persistir a preferência deve ser seguro e não deixar a interface assumindo conclusão inexistente.
6. **Limpeza opcional confirmada:** após a ativação, solicitar **confirmação explícita adicional** do usuário para remover os originais. Revalidar a correspondência entre os arquivos de origem e as cópias antes de cada remoção; nunca eliminar uma origem com alteração não refletida no destino.
7. **Falhas após a ativação:** não voltar silenciosamente à origem nem apagar outras cópias. Informar a quantidade de arquivos remanescentes e permitir resolução segura. A nova pasta continua sendo a ativa se estiver íntegra; a limpeza parcial não é reportada como sucesso completo.

**Regras transversais:** operação cancelável antes de ações destrutivas; nenhuma sobrescrita silenciosa; resultados intermediários explícitos; arquivos temporários e cópias parciais não devem ser limpos de modo a eliminar dados originais ou de terceiros. Garantia de rollback global não é presumida.

**Aspectos ainda pendentes de desenho/teste:** necessidade e formato mínimo de estado operacional recuperável após crash, isolamento diante de outras instâncias/processos, autorização do portal persistida, comportamento em cópia parcial e retentativa idempotente (evitar sobrescrever arquivos previamente criados), sem adicionar índice de metadados das notas. Detalhar sem criar framework de migração genérico.

### 6.2. Exclusão individual pela lixeira — DT-03 aprovada (alternativa A)

A exclusão solicitada pelo usuário deve **mover o arquivo Markdown selecionado para a lixeira**, após confirmação. Se a lixeira não estiver disponível, a operação não for suportada ou falhar, **informar o erro e não apagar definitivamente o arquivo**. Não implementar `unlink`/remoção permanente como fallback de exclusão individual.

**Fluxo de referência:**
1. Identificar o arquivo selecionado e tratar operações de salvamento em andamento, revisões locais não persistidas e eventual estado `Conflict`; nunca considerar um buffer não salvo como seguramente descartável.
2. Verificar a versão/caminho esperados antes da solicitação de envio à lixeira; se o arquivo tiver sido alterado, substituído, removido ou renomeado externamente, suspender a ação e aplicar a política de conflito **DT-02** em vez de remover um arquivo inesperado.
3. Solicitar confirmação explícita para **Mover para a lixeira**, sem linguagem de exclusão definitiva.
4. Executar a operação de lixeira no adaptador de filesystem, considerando API GIO/GFile de trash como candidata, **sem assumir suporte universal em sistemas de arquivos e nas pastas selecionadas por portal Flatpak**.
5. Somente após sucesso da operação, retirar a nota da lista e limpar seleção/estado associado; em caso de erro, manter o item e comunicar o resultado, sem exclusão definitiva alternativa.

**Limite importante:** uma verificação de fingerprint seguida de `trash` não é uma operação condicional atômica. Entre as duas ações, outro processo ainda pode modificar/substituir o caminho (**TOCTOU**). DT-04/DT-09 deverão considerar exclusividade de instância e redução de risco, e testes precisam evidenciar os limites reais da plataforma. Não prometer a impossibilidade absoluta de conflito externo sem garantias efetivas do SO.

**Testes:** confirmação/cancelamento, sucesso do trash, falha de permissão, backend sem lixeira, filesystem montado com comportamento diferente, Flatpak com pasta selecionada via portal, revisões pendentes, conflito por edição externa, caminho renomeado e ausência de fallback destrutivo.

**Distinção de escopo:** DT-03 aplica-se à **exclusão individual da nota pelo usuário**. A remoção dos originais após a migração aprovada na DT-20 é outro fluxo, exigindo confirmação adicional e verificações de segurança; sua política operacional precisa ser refinada sem assumir automaticamente que usa o mesmo mecanismo de lixeira.

### 6.3. Exportação — DT-07 aprovada (alternativa B)

**Política:** exportar tantas notas quanto possível, **sem sobrescrever arquivos preexistentes no destino**. Colisões são classificadas como **ignoradas**; falhas individuais recuperáveis são registradas e a exportação prossegue. Nenhum resultado parcial pode ser comunicado como exportação integral. Não há rollback do lote, exclusão dos originais, migração de diretório ou atualização da pasta ativa.

**Sequência de referência:**
1. Obter lista das notas elegíveis do diretório ativo (arquivos `.md` da raiz reconhecidos por DT-19/DT-21), excluir diretórios internos e selecionar destino explicitamente via seletor/portal (DT-18). Não modificar originais.
2. Antes de copiar cada nota, garantir que a revisão a exportar corresponde ao estado esperado: resolver buffer pendente, persistência incerta ou conflito conforme DT-02/DT-05/DT-06, ou classificar como **falha/pendência**, sem exportar silenciosamente conteúdo obsoleto. Não confundir exportar arquivo em disco com capturar alteração ainda só em memória.
3. Rejeitar ou tratar com segurança destino igual à origem ou arquivo origem=destino. Não tentar copiar arquivo sobre si próprio. Detectar nome ocupado e **ignorar essa nota**, deixando arquivo preexistente intacto. A detecção antecipada não substitui operação sem sobrescrita na publicação, diante de TOCTOU.
4. Criar cópias usando temporário próprio no destino, copiar o conteúdo e validar resultado/integridade; publicar sem sobrescrita quando suportado (ex.: criação exclusiva ou mecanismo condicional seguro avaliado no filesystem/portal). Uma cópia incompleta ou de resultado incerto não conta como sucesso. Não remover arquivos alheios ou arquivos preexistentes na limpeza.
5. Em falha individual **recuperável**, registrar item, motivo e seguir com os demais. Se o destino inteiro perder acesso ou ocorrer condição que inviabilize continuar, interromper de forma controlada e reportar o conjunto concluído e o restante **não processado**; não insistir repetidamente em operação sabidamente impossível.
6. Ao concluir ou cancelar, apresentar **relatório verificável** com contagens e identificação de notas **exportadas, ignoradas por colisão, com falha e não processadas por cancelamento/interrupção**. Arquivos já exportados com sucesso permanecem no destino. Cancelar não exclui cópias concluídas nem altera arquivos de origem.

**Integração:** feature `export` orquestra o lote e o progresso, reutilizando operações seguras do adaptador de filesystem da feature de notas sem introduzir framework genérico. UI libadwaita informa estado e resumo final, podendo detalhar notas problemáticas. Não bloquear main loop GTK com cópias ou hashes. O formato `.md` e o nome físico seguro são preservados; não criar arquivo de índice ou manifest de exportação no MVP.

**Falhas a verificar:** colisão, permissão revogada, espaço insuficiente, diretório removido, exportação para origem, erro de leitura de nota, estado Dirty/Conflict/PublishedUnverified, interrupção entre temporário e publicação, cancelamento em andamento e resultados que não podem ser confirmados. As garantias de cópia segura e as APIs de portal devem ser testadas no ambiente escolhido (DT-18).

**Detalhes ainda pendentes:** apresentação final do relatório, tratamento de arquivos `.md` inválidos/especiais (DT-14), critério de falha irrecuperável em todo o destino e mecanismos exatos de publicação sem sobrescrita. A escolha de continuar quando possível já está aprovada e não deve ser reinterpretada como exportação tudo-ou-nada.

## 7. Interface GTK4/libadwaita

### 7.1. Composição

Janela principal com barra de cabeçalho, lista lateral recolhível e editor `GtkTextView` simples, sem renderização Markdown. **DT-11:** layout adaptativo à largura disponível; o editor tem prioridade quando a janela fica estreita e a lateral se recolhe automaticamente, mantendo um controle acessível de navegação e preservando a preferência manual do usuário. Preferências usam janela libadwaita nativa. Escolhas de UI nativa devem ser compatíveis com o GNOME SDK 51 (DT-08).

A UI apresenta estados de notas e salvamento, confirma exclusão, informa conflito, **exibe a pasta ativa e permite alterá-la nas preferências**, além de abrir seletor de diretório na exportação. A navegação por teclado é requisito, não adição opcional.

### 7.2. Concorrência com o loop principal

O contexto GTK executa manipulação de widgets; I/O de arquivos que possa bloquear usa mecanismo de execução fora do loop de eventos. As respostas são entregues ao contexto principal e verificadas quanto à revisão e nota de origem. Não transportar referências de widgets para threads que não as suportem.

**A definir em M1/PREP-02:** mecanismo concreto GLib assíncrono versus threads controladas, e canal de entrega de resultados, conforme as capacidades das versões escolhidas.

### 7.3. Temas e transparência — DT-10 aprovada (alternativa A)

- `AdwStyleManager` controla Sistema/Claro/Escuro. A feature de aparência utiliza CSS localizado e atualiza as cores e alfas dos fundos ao alterar tema/transparência.
- **Faixa inicial aprovada: transparência de 0–70%** nos **fundos de toda a janela principal**, incluindo barra de cabeçalho, editor e barra lateral. `0%` representa fundo opaco e `70%` representa transparência máxima candidata, **não uma validação de legibilidade concluída**.
- **Textos, ícones, cursor, seleção, indicadores e controles** permanecem visualmente opacos/legíveis. Não aplicar `set_opacity` à janela inteira, pois reduziria a opacidade de conteúdo e controles.
- **Superfícies auxiliares com fundos opacos:** diálogos, popovers, menus e janela de preferências, incluindo confirmações de exclusão/conflito e controles críticos. Não herdar indiscriminadamente o alpha da janela principal para superfícies anexadas.
- Sem **click-through**: não alterar região de entrada de eventos para permitir cliques em outros aplicativos pela janela translúcida.
- O limite de 70% é uma **direção inicial de produto**. Se testes demonstrarem contraste insuficiente, **reduzir o máximo efetivo/documentado antes do release**; não alegar conformidade universal sobre fundos externos arbitrários.
- Validar no GNOME/Wayland e no runtime GNOME 51 (DT-08), com temas Claro/Escuro, alpha 0%, intermediário e 70%, fundos por trás claros/escuros/visualmente complexos, estados de foco, seleção, cursor, texto e controles. Não presumir que o compositor ofereça acesso ao conteúdo por trás da janela para ajuste automático.
- Verificar CSS das sub-superfícies e a composição RGBA no GTK4/libadwaita para evitar camadas com opacidades inconsistentes. Registrar limitações observadas e ajustar implementação ou limite permitido de modo proporcional.

**Critério de aceite técnico:** a faixa e as superfícies devem ser funcionalmente distintas (fundo translucido vs. conteúdo opaco), manter uso via teclado e foco visível, e não sacrificar legibilidade do fluxo de edição; testes manuais orientam o máximo final. Nenhum motor de adaptação visual automática ao conteúdo de outras janelas é exigido pela DT-10.

### 7.4. Janela, atalhos e GNOME

- Persistir tamanho e maximização, **não coordenadas absolutas**, em conformidade com Wayland.
- Restaurar última nota pelo caminho/nome; se indisponível, iniciar em estado seguro.
- Atalhos propostos: `Ctrl+N`, `Ctrl+F`, `F9`, `Ctrl+,`; verificar conflito com foco e localização.
- Atalho global externo: usuário configura no GNOME; o aplicativo só oferece instalação/comando de inicialização.
- Sempre no topo: recurso **manual** do menu/gerenciador de janelas GNOME, sem API de automação nem dependência de extensão no MVP.

### 7.4.0. Layout responsivo e barra lateral — DT-11 aprovada (alternativa B)

**Contrato de interface:** quando a largura disponível permitir, apresentar editor e lateral lado a lado **se a preferência manual da lateral estiver ativa**. Quando a largura ficar inferior ao limiar adaptativo validado, recolher automaticamente a lateral e **priorizar a área de edição**, sem bloquear acesso à lista ou ações essenciais. O recolhimento responsivo não equivale a escolha manual do usuário.

**Estado de visibilidade:**
- `sidebar-visible` no GSettings representa **apenas a intenção manual** do usuário, não o resultado momentâneo de uma largura reduzida. Não persistir o recolhimento automático nessa chave.
- Visibilidade efetiva decorre da combinação entre **intenção manual**, **largura disponível** e, se necessário, **navegação temporária explicitamente solicitada** pelo usuário. Em largura estreita, a lista permanece acessível por um controle de navegação com teclado; apresentar temporariamente a lateral não precisa transformar o editor em outra janela ou modo permanente.
- Quando a janela volta a ficar larga, restaurar a visibilidade **conforme a intenção manual**. Se a lateral foi ocultada manualmente, não reaparecer espontaneamente só pelo aumento da janela.
- Não reinicializar editor, nota ativa, buffers, foco, estados de conflito ou salvamento ao atravessar o ponto de quebra. A navegação temporária não pode ignorar a DT-06 sobre sair de nota com alterações pendentes.

**Candidatos de implementação para investigar (não decididos):** componentes adaptativos da libadwaita como `AdwOverlaySplitView` ou combinação equivalente suportada no SDK 51; separar estado manual de estado responsivo no modelo de UI. Se o componente escolhido introduzir mudança de navegação, preservar as mesmas ações da única janela, sem nova sessão de edição.

**Limites e critérios de responsividade:**
1. **Nenhuma largura/altura mínima ou breakpoint numérico é aprovado agora**. Derivar valores de testes do layout real, dimensões de controles, tradução pt-BR/en-US, escala de texto, teclado e comportamento de foco, sem cortar ações essenciais.
2. Validar largura estreita, limiar, larga e maximizada, incluindo transições repetidas estreita→larga→estreita. O editor deve ter espaço utilizável e evitar corte/colisões de cabeçalho, controles, barra lateral ou indicações de salvamento.
3. Validar tanto intenção manual “lateral visível” quanto “lateral oculta”, incluindo reabertura do aplicativo com dimensões persistidas. O estado responsivo não altera a preferência gravada; restauração normal não cria expansão inesperada.
4. Confirmar navegação acessível por teclado e tecnologia assistiva básica, foco perceptível, sem perda do buffer e sem click-through; incluir layout em tema Claro/Escuro e transparência conforme DT-10.
5. Caso as dimensões mínimas testadas não acomodem ações essenciais, ajustar a composição ou o mínimo da janela antes do release, registrando a razão; não fixar números arbitrários apenas por conveniência estética.

**Limite de escopo:** comportamento adaptativo é aprovado; valores concretos de `min-width`, `min-height`, breakpoint, transição e escolha de widget ainda exigem protótipo e validação no M3 com GTK4/libadwaita/GNOME 51. Não se exige uma segunda interface independente em modo compacto.

### 7.4.1. Instância única por sessão — DT-04 aprovada (alternativa A)

- **Invariante:** dentro de uma mesma sessão/identidade de execução do Oak Note, existe somente uma instância ativa que controla o editor e a janela principal. Uma nova tentativa de abertura **encaminha uma solicitação de ativação à instância existente**, em vez de inicializar um segundo editor independente.
- **Mecanismo candidato:** usar o comportamento de instância única e ativação de `GApplication`/`AdwApplication` com o Application ID definitivo `io.github.rafaelcarvalho89.oaknote` e o barramento de sessão. A escolha é diretriz técnica a verificar na configuração de desenvolvimento e Flatpak, não garantia universal de singleton entre todos os contextos de execução.
- **Estado preservado:** ao reativar, apresentar a janela principal existente e manter seleção, buffer, revisões pendentes, diálogo ou conflito em curso. Não reinicializar repositório, cancelar autosave nem abrir segunda janela.
- **Operações coordenadas:** o serviço de notas coordena serialização e ordem de conclusão para gravação, renomeação, lixeira e migração na mesma nota ou diretório envolvido. A identidade operacional de arquivo pode mudar ao renomear; não depender apenas da string de caminho para garantir integridade.
- **Limites:** sessões distintas, ambientes de barramento diferentes, instalações Flatpak e nativa com escopos distintos, editores externos e operações diretas no filesystem podem coexistir. **DT-02 continua obrigatória**; singleton de aplicação não é bloqueio de arquivos distribuído, nem prova de ausência de TOCTOU.
- **Falha/encerramento:** garantir que após encerrar a instância anterior a aplicação possa iniciar normalmente, sem bloqueios próprios órfãos. Falha ao comunicar a instância primária precisa de estado/erro claro; não criar editor independente como fallback silencioso.
- **Sem excesso de engenharia:** preferir mecanismos nativos da plataforma; não criar daemon, serviço de lock ou IPC customizado apenas para esse requisito, a menos que testes mostrem necessidade real.

**Validação na PREP-02/M1:** duas ativações na mesma sessão, inclusive launcher/Flatpak; restauração da janela; uma única janela e um único buffer; ativação durante autosave ou diálogo; encerramento e reinício após falha; acessos por editores externos continuam resultando em detecção de conflito.

### 7.5. Localização e acessibilidade

Usar gettext para textos próprios da interface em pt-BR e inglês, com fallback documentado. Preservar notas sem tradução. Labels e papéis acessíveis, estados de salvamento anunciáveis, foco visível; testes básicos com Orca e teclado.

## 8. Preferências com GSettings

**Um esquema único** com chaves sugeridas, ainda sem nomes finais aprovados:

| Chave candidata | Tipo conceitual | Valor inicial sugerido | Papel |
| --- | --- | --- | --- |
| `color-scheme` | enum/string | `system` | Preferência |
| `notes-directory` | URI/caminho GFile serializável (formato final a decidir) | diretório especial XDG Documentos | Preferência |
| `confirmed-notes-directories` | conjunto/lista de referências a diretórios aceitos (representação final a definir) | vazio | Consentimento de reconhecimento por diretório (DT-21), **não permissão de filesystem** |
| `background-transparency` | inteiro 0–70 | 0 | Preferência |
| `sidebar-visible` | boolean | true | **Preferência manual** da barra lateral; recolhimento automático DT-11 é estado transitório, não persistido |
| `window-width` | inteiro positivo | 800 | Estado |
| `window-height` | inteiro positivo | 560 | Estado |
| `window-maximized` | boolean | false | Estado |
| `last-note` | string opcional/vazia | vazio | Estado |

- Esquema proposto: `io.github.rafaelcarvalho89.oaknote`.
- Chaves versionadas e distribuídas com os arquivos de instalação.
- O domínio não depende da API GSettings.
- Valores inválidos, referências perdidas e configurações antigas recebem tratamento seguro.
- A pasta escolhida deve ser persistida sem depender de um caminho hardcoded no host; quando obtida via portal, verificar validade do identificador/caminho e autorização ao reiniciar.

### 8.1. Confirmação por diretório — DT-21 aprovada (alternativa A)

**Contrato de produto:** o Oak Note lembra, no esquema GSettings já definido pela ADR-04, cada pasta cuja inclusão de arquivos `.md` da raiz foi explicitamente aceita segundo a DT-19. Ao retornar a uma pasta reconhecida com referência válida, não solicitar novamente a confirmação. A configuração não guarda lista de arquivos, conteúdo, hashes das notas ou índice de documentos, e não escreve `sidecar` no diretório.

**Sequência e responsabilidades:**

1. Ao acessar uma pasta, resolver o diretório ativo e **validar a disponibilidade e a autorização de acesso**, independentemente do registro de consentimento (DT-18).
2. Identificar o diretório de forma suficientemente estável para comparar a referência ao estado persistido. Considerar URI/caminho normalizado e, quando disponível, identidade real de filesystem/portal; o formato definitivo e regras para symlinks, links simbólicos, mudanças de montagem e URI de portal continuam sujeitos à DT-14/DT-15 e validação.
3. Havendo arquivos `.md` na raiz e **nenhum consentimento válido** para a identidade do diretório, solicitar confirmação **antes** de listar ou abrir como notas. Aceitar persiste a referência no GSettings. Recusar permite escolher outra pasta sem registrar consentimento.
4. Havendo consentimento válido, reconhecer arquivos `.md` da raiz sem repetir a confirmação, mesmo após reiniciar ou retornar a partir de outra pasta. Não executar varredura recursiva.
5. Se o registro for inválido, o diretório estiver inacessível/removido/substituído, a identidade não puder ser comprovada ou a autorização do portal tiver sido revogada, **não usar cegamente um caminho antigo**. Solicitar reautorização ou nova confirmação conforme o caso. Não transformar uma confirmação de produto em permissão do portal.
6. Armazenar dados mínimos de referência a diretórios, sem novos metadados por nota. Validar/deduplicar registros e tratar valores malformados ou configurações antigas sem habilitar reconhecimentos implícitos.

**Pendências técnicas específicas (não novas decisões de produto):** representação da lista/estrutura em GSettings, normalização e equivalência de pastas, identidade sob mudança de caminho/montagem, sincronização com `notes-directory`, tratamento de diretório com `.md` criados após a confirmação e política de limpeza de registros obsoletos. A direção A permanece aprovada; não presumir identidade perfeita do filesystem nem acesso duradouro fornecido pelo portal.
- Persistência precisa ser testada tanto na execução local quanto dentro do Flatpak.

## 9. Flatpak, empacotamento e diretórios

### 9.1. Identidade

- Produto: Oak Note; crate e binário `oak-note`.
- Application ID e identificadores de desktop/AppStream: `io.github.rafaelcarvalho89.oaknote`.
- Manifesto principal versionado em `flatpak/`.
- **DT-08 aprovada (alternativa C):** runtime `org.gnome.Platform//51` e SDK `org.gnome.Sdk//51`; a referência GNOME 50 da POC não é a linha escolhida para o MVP e as integrações precisam de revalidação.
- **Rust `stable` não fixado em versão exata:** adotar o canal estável corrente nas ferramentas de desenvolvimento e CI/Flatpak, sem fixar número de compilador por projeto; não declarar uma versão específica como baseline. Verificar em M1 como a extensão `org.freedesktop.Sdk.Extension.rust-stable` funciona com GNOME SDK 51 (base Freedesktop SDK 26.08) e registrar a versão realmente instalada.
- Manter `Cargo.lock` versionado, preferir `cargo --locked` nos builds e resolução controlada de fontes para o build offline no SDK. Isso **fixa a resolução das dependências, não a toolchain Rust nem o conteúdo exato do SDK/extensão**, portanto não implica builds binariamente reproduzíveis.
- Saída para GitHub Releases após validação e aprovação manual.
- Não presumir atualização automática por distribuição de um arquivo `.flatpak` avulso.

### 9.1.1. Compatibilidade e rastreabilidade de builds — DT-08 aprovada (alternativa C)

**Direção:** os identificadores do runtime/SDK GNOME permanecem na linha **51**, mas o canal do compilador é **Rust stable móvel**, não uma versão de Rust fixada em `rust-toolchain.toml` ou no CI. Fixar apenas o nome `stable` em arquivo de ferramenta é compatível com a decisão, mas fixar número exato de release não é. As versões concretas dos compiladores e extensões podem mudar entre máquinas ou datas.

**Contrato de build e validação:**
1. Em M1, verificar que `org.gnome.Platform//51` e `org.gnome.Sdk//51` estão disponíveis e compatíveis com o ambiente de referência. GNOME 51 utiliza Freedesktop SDK **26.08** como base; a extensão Rust stable deve acompanhar a linha correspondente, com branch final e manifesto efetivo **confirmados no SDK instalado**, não presumidos.
2. Verificar Rust/Cargo efetivamente fornecidos na compilação Flatpak e Rust/Cargo locais ou da CI. Versões distintas podem gerar comportamento e diagnósticos distintos; qualquer divergência relevante deve ser reportada e validada, sem troca silenciosa de baseline.
3. Registrar, em logs/metadados do job de build, `rustc --version --verbose`, `cargo --version`, identidade/revisão do runtime, SDK e extensão Rust utilizados, dependências resolvidas e versão do artefato. Não colocar esses dados no conteúdo das notas.
4. Exigir `Cargo.lock` no repositório e builds com `--locked` quando aplicável, além de testes `fmt`, `clippy`, `test` e build Flatpak; uma atualização do canal stable precisa passar pelas verificações **antes de publicação**, sem promover automaticamente artefatos.
5. Validar GTK4/libadwaita, GIO, portais, transparência/Wayland, GSettings e compatibilidade das dependências/crates com a versão de Rust disponível; registrar incompatibilidades e bloquear o release até correção. O runtime Flatpak GNOME 51 não exige que o host já esteja executando GNOME Shell 51, mas a integração do aplicativo com o GNOME/Debian-alvo deve ser testada.
6. **Reprodutibilidade:** com canal móvel, SDK e extensões atualizáveis, não garantir builds binariamente idênticos entre datas. O registro de versões e revisões permite **rastreabilidade/diagnóstico**, não equivale a congelar o ambiente. Considerar snapshots externos apenas se futuramente necessário, sem rever silenciosamente a escolha C.

**Limite de escopo:** esta DT-08 define versões-alvo e estratégia de evolução, **não validação técnica concluída**. Não criar ou editar manifesto Flatpak, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` nem código até autorização de implementação.

**Referências para conferência técnica:**
- https://thisweek.gnome.org/posts/2026/08/twig-263/
- https://github.com/flatpak/flatpak-builder-tools/blob/master/cargo/README.md
- https://rust-lang.github.io/rustup/overrides.html

### 9.2. Permissões — DT-18 aprovada (alternativa A)

**Estratégia definida para validação técnica:** `--filesystem=xdg-documents` com acesso de leitura/escrita em toda a pasta XDG Documentos; para outras pastas, usar seleção explícita pelo FileChooser Portal (por exemplo, GTK/GIO FileDialog com escolha de diretório), sem permissões estáticas adicionais. A decisão respeita o mínimo privilégio relativo às opções avaliadas, mas **não significa acesso restrito exclusivamente aos arquivos criados pelo Oak Note dentro de Documentos**.

**Requisitos e restrições:**

1. **Destino padrão:** resolver XDG Documentos do usuário, sem caminho hardcoded. A permissão estática do manifesto abrange Documentos, não `home` inteira. Não conceder `--filesystem=home`, `host`, rede nem D-Bus de escopo amplo como fallback.
2. **Pasta alternativa:** exigir seleção voluntária via portal. Não tratar uma string de caminho salva em GSettings como autorização. Persistir a localização autorizada conforme API e contrato real (URI/referência GFile quando aplicável), distinguindo caminhos do host e caminhos expostos pelo Documents Portal.
3. **Pré-ativação:** verificar de forma segura se o diretório pode ser listado e se operações de leitura, criação, escrita temporária, substituição/publicação condicionada, renomeação e operações obrigatórias de gerenciamento são viáveis. **Não usar arquivos preexistentes do usuário como teste**, não sobrescrevê-los nem removê-los. Se testes técnicos exigirem arquivo temporário de prova, este deve ser exclusivo da aplicação e tratado com limpeza controlada.
4. **Lixeira:** validar a capacidade de mover para a lixeira no backend selecionado, respeitando DT-03; não substituir uma lixeira indisponível por exclusão definitiva. Sua disponibilidade pode variar por filesystem/montagem ou portal. O mecanismo exato de prova sem tocar dados do usuário será validado em M1.
5. **Integridade:** comprovar se o caminho selecionado permite temporário no mesmo diretório, sync do temporário, rename/replace com as garantias exigidas (DT-05), verificação de identidade/hash (DT-09), operações de cópia/migração sem sobrescrita (DT-20) e resolução de conflitos (DT-02). Não inferir atomicidade ou durabilidade apenas porque a escrita básica funcionou.
6. **Ativação:** se uma capacidade indispensável falhar ou não puder ser assegurada para o destino selecionado, comunicar a limitação e **manter a pasta anterior** sem elevar permissões automaticamente. Validações não garantem que a capacidade continue disponível depois; erros futuros precisam ser tratados de modo seguro.
7. **Acesso entre execuções:** o FileChooser Portal pode conceder acesso persistente a documentos selecionados, mas não presumir que isso implique todas as operações exigidas sobre o **diretório**. Testar com Flatpak encerrado/reiniciado, identificando autorização revogada, diretório movido/desmontado ou acesso não reconstituído. Nesses casos, solicitar reautorização ou nova pasta, **sem redirecionamento silencioso nem perda de notas**.
8. **Separação de consentimentos:** autorização de acesso pelo portal é uma permissão de sistema; confirmação para reconhecer Markdown preexistente (DT-19/DT-21) é uma escolha de produto. Uma **não substitui** a outra.
9. **Exportação:** seleção de destino por portal e validação do acesso de escrita conforme operações exigidas; não ampliar o acesso estático por causa da exportação.

**Validação de M1 / ambiente-alvo:** Debian, GNOME, Wayland, pacote Flatpak real, sem overrides ou permissões extras. Incluir testes com XDG Documentos localizado fora do caminho literal `~/Documentos`, pasta externa autorizada por portal, portal FUSE, permissões negadas, lixeira não suportada, arquivos existentes, reinício com e sem autorização, migração entre sistemas de arquivos, erros durante publicação, e limites de sync/rename/trash. Registrar os resultados observados e versões de Flatpak, xdg-desktop-portal, backend GNOME e filesystem. Uma checagem favorável em um backend não autoriza prometer compatibilidade universal.

**Se houver bloqueio técnico:** não alterar o manifesto para liberar `home`/ `host` automaticamente. Preservar as notas e trazer para revisão uma alternativa de produto/arquitetura fundamentada em testes, antes da implementação do fluxo incompatível.

**Referências de verificação:**
- https://docs.flatpak.org/en/latest/sandbox-permissions.html
- https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html
- https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html

### 9.3. Build e release

Pipeline proposto: `cargo fmt --check`, `cargo clippy`, `cargo test`, verificação de build Flatpak/manifesto e testes manuais no GNOME/Wayland antes do release. **DT-08:** Rust stable móvel exige verificação do compilador/SDK efetivos e compatibilidade a cada build/release; `Cargo.lock` e `--locked` evitam deriva das dependências resolvidas, não da ferramenta de compilação. O processo de liberação verifica licença MIT, changelog, metadados AppStream e caminho de instalação.

## 10. Qualidade, testes e diagnósticos

### 10.1. Testes unitários e de integração

| Categoria | Cenários prioritários |
| --- | --- |
| Títulos | caracteres permitidos, vazios, colisões, normalização, Unicode, extensão `.md` |
| Domínio | seleção/revisão, regras independentes de GTK, erros previsíveis |
| Salvamento | debounce, reordenação de resultados, revisão nova durante I/O, retry |
| DT-05 — sincronização | Escrita/flush parcial, sync do temporário, publicação atômica quando suportada, sync do diretório, erro antes/depois do rename, resultado incerto, ambiente/portal sem sync disponível, buffer e estado coerentes |
| DT-06 — troca/fechamento | Troca com Dirty/Saving/Error/PublishedUnverified/Conflict; retry, cancelamento preservando buffer, descarte com confirmação específica, reconciliação antes de repetir publicação incerta; fechamento com múltiplas notas; saída forçada sem garantia |
| Filesystem | escrita temporária, substituição, falha simulada, arquivo removido, acesso negado |
| DT-03 — lixeira | Confirmação/cancelamento, sucesso, lixeira não suportada, acesso negado, pasta via portal, item alterado externamente, nenhuma exclusão permanente como fallback |
| DT-04 — instância e serialização | Lançamento repetido na mesma sessão; reativação da janela mantendo buffer, conflitos e autosave; concorrência de gravação/renomeação/lixeira/migração; recuperação após encerramento; edição externa ainda detectada |
| Conflito | alteração externa, arquivo removido/renomeado, autosave bloqueado, recarregar com confirmação, salvar como nova nota respeitando DT-01, cancelar e preservar buffer; falha de I/O durante resolução |
| DT-09 — fingerprint | Alteração de bytes com tamanho e mtime preservados, substituição por novo inode/identidade, arquivo ausente, acesso negado, hash indisponível, metadados alterados sem mudança de conteúdo, verificação anterior a save/rename/trash/migração, atualização somente após revisão confirmada e TOCTOU reconhecido |
| Busca/ordem | correspondência por título, mudança de mtime e empates |
| Preferências | defaults, leitura/atualização, restauração, pasta de notas e entrada inválida |
| Diretório de notas | XDG Documentos em instalações pt-BR/en-US; pasta inexistente, acesso negado, seleção alternativa, restauro após reinício, arquivos `.md` preexistentes |
| DT-18 — permissões Flatpak | Manifesto restrito a `xdg-documents`, nenhuma permissão `home`/`host`; portal para nova pasta; listagem, leitura, criação, atualização, rename/replace, sincronização quando suportada, lixeira, teste em FUSE, revogação e persistência do acesso após reinício; rejeitar pasta incompatível sem alterar destino anterior |
| DT-19 — confirmação | Pasta com `.md` preexistente na raiz no primeiro acesso: aceitar, escolher outra pasta, cancelar, pasta vazia, subdiretórios ignorados e não modificação dos arquivos |
| DT-21 — persistência da confirmação | Confirmar pasta A e B, reiniciar e revisitar sem novo diálogo; pasta nova exige consentimento; valores GSettings inválidos, pasta substituída/inacessível, referência do portal expirada/revogada e identidade ambígua não permitem reconhecimento silencioso |
| DT-08 — runtime/toolchain | GNOME Platform/SDK 51, extensão Rust stable compatível, ambiente Flatpak real, Cargo.lock + --locked, registro de versões efetivas Rust/Cargo/SDK, atualização de stable com testes, libadwaita/GTK4/portais no Debian/GNOME/Wayland; sem alegação de build idêntico garantido |
| DT-10 — transparência e contraste | Transparência 0%, intermediária, 70%; temas Claro/Escuro e fundo atrás claro/escuro/complexo; cabeçalho/editor/lateral translúcidos, conteúdo e controles opacos; diálogos/popovers/menus/preferências com fundos opacos; foco/seleção/cursor legíveis; sem click-through e possibilidade de reduzir o limite após evidência |
| Troca de pasta | Escolha de manter, escolha de migrar, cancelamento, arquivos de mesmo nome, edições pendentes e falhas parciais sem perda silenciosa |
| DT-20 — cópia/validação | Cópia integral com verificação de bytes/hash; nenhuma ativação com cópia incompleta, origem modificada, destino ocupado ou acesso negado; arquivos de origem intactos durante a transferência |
| DT-20 — ativação/limpeza | Confirmação adicional para remoção, revalidação antes de cada exclusão, cancelamento da limpeza, falha parcial de remoção e persistência correta do diretório ativo |

| Exportação — DT-07 | Cópia de todas as notas possíveis; ocupação de nome ignorada sem overwrite; erro individual recuperável não interrompe lote; cópias verificadas e origem intacta; destino igual à origem, buffers pendentes, falha global, cancelamento e relatório com exportadas/ignoradas/com falha/não processadas |

Os testes de integração de filesystem devem usar diretórios isolados e não depender do diretório real do usuário.

### 10.2. Testes manuais GTK/GNOME

- Tema do sistema e troca em tempo real; clareamento/escurecimento correto.
- Fundo com transparência em cabeçalho, lista e editor; sem transparência dos controles.
- Texto, foco, seleção e cursor legíveis com conteúdos sobrepostos de fundo variados.
- DT-11: layout estreito/largo, recolhimento automático, acesso temporário à lista e retorno à preferência manual; testar tamanhos mínimos validados, pt-BR/en-US, escala de texto, maximização, restauração e edição pendente sem perda de buffer.
- Navegação por teclado, atalhos, Orca e mensagens de erro.
- GNOME manual “Sempre no topo”, funcionamento quando outra janela está ativa.
- Instalação/execução Flatpak e acessos autorizados, inclusive leitura/gravação em XDG Documentos, escolha de pasta externa e exportação via portal.

### 10.3. Logs e privacidade

Eventos de erro e operação podem ser estruturados em logs locais. **Não registrar corpo das notas, conteúdos selecionados, caminhos externos sensíveis sem necessidade, credenciais ou informações do clipboard.** Não há telemetry endpoint.

## 11. Rastreabilidade de implementação

| Requisitos de produto | Módulos/contratos predominantes | Teste principal |
| --- | --- | --- |
| RF-01–08 | feature `notes` + porta de repositório seletiva | unitário e integração de filesystem |
| RF-09–14 | feature `notes` (SaveCoordinator + adaptador), incluindo estados de persistência DT-05 e coordenação de navegação/fechamento DT-06 | unitário, integração, falhas antes/depois da publicação, decisões do usuário e testes manuais de fechamento |
| RF-15–22 | features `notes`/`appearance`/`storage` + GSettings, incluindo DT-11 | teste manual de layout adaptativo, intenção manual da lateral, acessibilidade e persistência |
| RF-23 | feature `export` + portal (DT-07) | integração de exportação parcial/colisão/cancelamento, relatório e testes manuais do portal |
| RF-24 | feature de UI/localização + gettext | checagem de tradução e teste manual |
| RF-25 | flatpak + CI (DT-08) | build, instalação e smoke test GNOME 51 com Rust stable móvel; logs de versões, reprodutibilidade limitada e compatibilidade das dependências |
| RF-26 | feature de storage + GSettings + portal (DT-18/DT-21) | seleção, validação das operações, autorização persistente e consentimento por pasta no GSettings, testados separadamente |
| RF-27 | composição da aplicação + GApplication/AdwApplication + serviço de notas | ativações repetidas, buffer preservado e serialização de operações |
| RNF-01–16 | transversal, conforme área | matriz de testes e revisão |

## 12. Decisões técnicas pendentes — não aprovadas

**Decisões concluídas:** **DT-01 — B:** sugerir nome alternativo editável em colisões, sem sobrescrever. **DT-17 — C:** oferecer manter notas na origem ou migrá-las. **DT-19 — B:** solicitar confirmação inicial para reconhecer `.md` preexistentes na raiz. **DT-20 — B:** copiar e validar integralmente antes de ativar a nova pasta; pedir confirmação adicional e revalidar antes de remover originais. **DT-02 — B:** em conflito externo, bloquear sobrescrita e oferecer recarregar com confirmação, salvar edição local como nova nota ou cancelar preservando o buffer e o estado de conflito. **DT-03 — A:** mover a nota para lixeira após confirmação, sem exclusão definitiva caso falhe ou não seja suportado. **DT-04 — A:** instância única por sessão da aplicação; novas ativações acionam a janela existente; operações internas serializadas, sem presumir proteção global contra editores externos. **DT-05 — B:** sincronizar dados do temporário, publicar via substituição atômica quando suportada e sincronizar o diretório quando disponível, distinguindo publicação de durabilidade verificada e resultados incertos. **DT-06 — B:** antes de trocar de nota ou encerrar normalmente, tentar concluir salvamentos; se falhar ou houver durabilidade incerta, oferecer retry, cancelar/continuar editando ou descartar com confirmação específica. Conflitos seguem DT-02 e encerramento forçado não garante recuperação. **DT-09 — B:** comparar metadados e hash de conteúdo mantidos em memória, verificados sob demanda antes de operações destrutivas; sem monitoramento contínuo obrigatório. **DT-18 — A:** conceder leitura/escrita estática somente a XDG Documentos no Flatpak e solicitar autorização por portal para outras pastas; testar operações reais e continuidade do acesso; jamais ampliar permissões automaticamente. **DT-21 — A:** persistir confirmações de reconhecimento de Markdown **por diretório** em GSettings; revisitas válidas não exigem confirmação, sem índice de notas ou concessão de acesso do sistema. **DT-07 — B:** exportar as notas possíveis sem sobrescrever destinos ocupados, continuar após falhas individuais recuperáveis e reportar exportadas, ignoradas, falhas e itens não processados, preservando originais. **DT-08 — C:** usar GNOME Platform/SDK 51 e canal Rust stable sem versão exata fixada; registrar versões efetivas e validar compatibilidade/release, sem prometer reprodutibilidade binária entre builds. **DT-10 — A:** transparência inicial de 0–70% nos fundos da janela principal; conteúdo e controles visualmente opacos; fundos de menus, popovers, diálogos e preferências opacos. Validar legibilidade e reduzir limite se necessário. **DT-11 — B:** layout adaptativo à largura; recolhimento automático da lateral em janela estreita com prioridade ao editor; manter preferência manual persistida independentemente do recolhimento responsivo. Permanecem pendentes os detalhes técnicos de detecção, recuperação e permissões, sem alterar essas decisões aprovadas.

| Prioridade | ID | Decisão a tomar | Alternativas a investigar |
| --- | --- | --- | --- |

| Média | DT-12 | Padrões de atalhos e busca de título (Unicode/casefold) | GNOME HIG + testes |
| Média | DT-13 | Ordem estável em empate de timestamp e arquivos sem mtime | título/caminho como desempate |
| Média | DT-14 | Política de arquivos inesperados e symlinks | ignorar e notificar / validar |
| Média | DT-15 | Valores finais das chaves GSettings e migração | defaults e versionamento |
| Média | DT-16 | Mecanismo de execução de I/O e retorno ao contexto GTK | GLib assíncrono / worker controlado |

**Recomendação de refinamento:** DT-01, DT-17, DT-19, DT-20, DT-02, DT-03 e DT-04 estão aprovadas. A primeira rodada de decisões críticas de integridade e as DT-05/DT-06 foram aprovadas; **DT-09, DT-18, DT-21, DT-07, DT-08, DT-10 e DT-11 também foram aprovadas; próxima decisão: DT-12 — atalhos internos e semântica de busca Unicode**, preservando validações de responsividade DT-11, legibilidade DT-10, suporte DT-08 e permissões DT-18. Validar DT-18 em POC técnica direcionada **dentro do próprio M1**, se necessário para comprovar suporte a permissões e atomicidade no Flatpak, sem assumir garantias antecipadas.

## 13. Critérios de conclusão da PREP-02

Para declarar a PREP-02 aprovada, precisamos:
1. Verificar cobertura de RF-01–RF-27 e RNF-01–RNF-16.
2. Incorporar as DT-01, DT-17, DT-19, DT-20, DT-02, DT-03, DT-04, DT-05, DT-06, DT-09, DT-18, DT-21, DT-07, DT-08, DT-10 e DT-11 aprovadas; verificar integridade do salvamento, Flatpak, compatibilidade GNOME 51/Rust stable, transparência/legibilidade e responsividade antes do backlog.
3. Estabelecer contratos operacionais de salvamento, renomeação, exclusão, troca de pasta e exportação suficientemente testáveis.
4. Respeitar a escolha GNOME Platform/SDK 51 e Rust stable não fixado (DT-08); validar versões efetivas/extensão/dependências, limitar promessas de reprodutibilidade e verificar permissões/publicação atômica em pastas selecionadas (DT-18).
5. Definir limites verificáveis de acessibilidade e falhas.
6. Confirmar que nenhum requisito foi inventado além da PREP-01.
7. Revisar este documento e aceitar sua baseline antes do backlog INVEST na PREP-03.

**Situação:** minuta técnica para revisão. Não é autorização para desenvolver o MVP.

## 14. Referências internas

- [Especificação do Produto (PREP-01)](product-specification.md)
- ADR-01–ADR-07: decisões aprovadas nesta fase de planejamento, ainda a registrar individualmente em `.docs/architecture/`.
- Relatórios de POC: fonte de experimentação anterior; validação manual complementar confirmada pelo mantenedor. Não transportar arquivos da POC para a `main` sem revisão expressa.

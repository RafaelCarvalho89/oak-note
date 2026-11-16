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
- A pasta Documentos pode conter `.md` criados fora do Oak Note. **É obrigatório definir política para arquivos preexistentes e escopo de listagem**, evitando tocar arquivos de terceiros sem ação do usuário (DT-19).
- **DT-19 aprovada (alternativa B):** ao primeiro acesso a pasta com arquivos `.md` **preexistentes na raiz**, informar que serão reconhecidos como notas e solicitar confirmação. Oferecer **aceitar** ou **selecionar outra pasta**; antes da confirmação, não apresentar/abrir o conteúdo como nota. A detecção não percorre subdiretórios, não importa arquivos nem os altera. Pastas sem arquivos `.md` na raiz não exigem confirmação. A forma de lembrar a autorização por pasta será refinada (DT-21), sem índice das notas.
- **DT-17 aprovada (alternativa C):** ao trocar a pasta ativa, oferecer opção explícita de **manter notas na origem** ou **migrá-las para a pasta selecionada**. Manter preserva os arquivos na origem; migrar é regido pela **DT-20 aprovada (alternativa B)**: copiar integralmente, validar cópias, ativar destino após validação completa e exigir confirmação explícita adicional, com verificações de segurança, antes de remover originais. A migração entre pastas não é uma única transação atômica; falhas parciais e recuperação exigem tratamento.

**Flatpak:** a permissão estática `--filesystem=xdg-documents` é candidata prática para acesso transparente à pasta Documentos configurada pelo usuário; porém permite acesso amplo **dentro de Documentos**, e exige revisão de mínimo privilégio. Para locais arbitrários, priorizar o portal de seleção de pasta com acesso autorizado pelo usuário; verificar persistência do acesso entre sessões e se criar/renomear/substituir arquivos com segurança é suportado no caminho devolvido pelo portal (DT-18). Não conceder `--filesystem=home` somente para acomodar a escolha de pasta. As garantias de escrita atômica podem variar no caminho FUSE do portal e precisam ser validadas experimentalmente.

Referências oficiais:
- https://docs.gtk.org/glib/func.get_user_special_dir.html
- https://docs.flatpak.org/en/latest/sandbox-permissions.html
- https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html

### 4.3. Título e nome físico (QA-01/QA-02)

**Proposta para avaliação:** derivar nome a partir de título legível, preservando Unicode válido, normalizando espaços periféricos, impedindo `/`, NUL, `.`, `..`, títulos vazios e nomes reservados pela política do produto. Manter extensão `.md` minúscula como convenção; preservar legibilidade em vez de sempre aplicar slug ASCII.

**DT-01 aprovada — alternativa B:** em caso de colisão, **sugerir nome alternativo disponível** (ex.: `Ideias (2).md`), com possibilidade de aceitar ou editar antes de confirmar. **Nunca substituir arquivo existente**. A normalização de caracteres, extensão, casefold, números de sufixo e o nome inicial sugerido ainda exigem detalhamento.

Listar somente arquivos regulares apropriados **na raiz** do diretório ativo; sem busca recursiva. Condicionar o reconhecimento de arquivos `.md` preexistentes no primeiro acesso à confirmação do usuário (DT-19). Definir política sobre symlinks e arquivos não regulares antes do desenvolvimento; não presumir que entradas externas sejam seguras.

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

### 5.4. Troca de nota, encerramento e erros

A aplicação tenta concluir gravações pendentes na troca e no encerramento. Em caso de falha, oferece uma decisão explícita (por exemplo, manter edição aberta e tentar novamente, cancelar a operação ou descartar conscientemente). Um diálogo de falha **não garante** retenção após matar o processo.

**Ponto de decisão:** a estratégia exata de gestão da saída e seleção com `Dirty/Error/Conflict` exige protótipo de interação e teste GTK. Evitar promessa de bloquear infalivelmente o encerramento pelo gerenciador de janelas.

### 5.5. Fingerprints e modificações externas (QA-05/QA-07)

**Proposta:** um fingerprint efêmero composto por informações disponíveis do arquivo (por exemplo, tamanho, modificação, identidade do objeto e/ou hash) permite detectar mudanças usuais. Não é chave estável de domínio e não substitui coordenação atômica entre processos.

**DT-02 aprovada (alternativa B) — resolução manual:** ao detectar que o arquivo aberto mudou, foi substituído, removido ou renomeado externamente desde a leitura, o `SaveCoordinator` suspende a publicação automática **daquela nota** e sinaliza `Conflict` à interface. A resolução oferece:

1. **Recarregar versão externa:** reler a versão atualmente existente do disco, mostrando confirmação explícita antes de descartar qualquer alteração local pendente. Se o arquivo original não estiver disponível, apresentar erro/estado de arquivo ausente; não criar arquivo em seu lugar automaticamente.
2. **Salvar edição local como nova nota:** criar outro arquivo Markdown com nome validado e **sugestão editável conforme DT-01**, usando operação que não sobrescreve destino existente. Preservar o arquivo externo intacto. Somente considerar a edição local salva após sucesso real da criação; se houver falha, conservar buffer e conflito.
3. **Cancelar:** fechar a escolha de resolução sem perder a edição local; permanecer em `Conflict`, manter autosave bloqueado **para o arquivo afetado** e permitir nova tentativa de resolução.

**Garantias:** a detecção de conflito e a criação de nova nota devem verificar o estado do filesystem na operação efetiva; uma checagem antecipada não elimina riscos de TOCTOU, abordados em DT-04/DT-09. A escolha de recarregar não significa mesclagem de textos; não há merge automático no MVP. O comando de salvar como nova nota **não deve salvar sobre o caminho da versão externa**. A interface não pode apresentar `Clean/Salvo` antes da persistência confirmada. Troca de nota e encerramento respeitam RF-12/DT-06 quando existir buffer local conflitado.

**Detalhes ainda abertos:** forma de identificar renomeação versus remoção, fingerprint efetivo, concorrência de outros aplicativos ou sessões, controles de saída em estado `Conflict` e erro durante a opção de salvar como nova nota. A DT-04 evita múltiplas instâncias independentes dentro da mesma sessão do aplicativo, mas não elimina os conflitos externos definidos na DT-02.

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
| Exportar | Coordenar lista e progresso | Cópia `.md` com destino do usuário | Parcial, colisões, permissões |
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

### 6.3. Exportação (QA-11)

**Proposta:** o usuário escolhe diretório usando seletor/portal adequado; exportar arquivos Markdown sem alterar originais, preservando títulos quando seguros. O relatório final identifica sucesso, falhas e cancelamento; nenhuma cópia existente deve ser sobrescrita silenciosamente. Definir antes da implementação se a operação é tudo-ou-nada ou permite resultado parcial.

## 7. Interface GTK4/libadwaita

### 7.1. Composição

Janela principal com barra de cabeçalho, lista lateral recolhível e editor `GtkTextView` simples, sem renderização Markdown. Preferências usam janela libadwaita nativa. Escolhas de UI nativa devem ser compatíveis com versões de SDK selecionadas.

A UI apresenta estados de notas e salvamento, confirma exclusão, informa conflito, **exibe a pasta ativa e permite alterá-la nas preferências**, além de abrir seletor de diretório na exportação. A navegação por teclado é requisito, não adição opcional.

### 7.2. Concorrência com o loop principal

O contexto GTK executa manipulação de widgets; I/O de arquivos que possa bloquear usa mecanismo de execução fora do loop de eventos. As respostas são entregues ao contexto principal e verificadas quanto à revisão e nota de origem. Não transportar referências de widgets para threads que não as suportem.

**A definir em M1/PREP-02:** mecanismo concreto GLib assíncrono versus threads controladas, e canal de entrega de resultados, conforme as capacidades das versões escolhidas.

### 7.3. Temas e transparência

- `AdwStyleManager` controla Sistema/Claro/Escuro.
- Um componente de aparência aplica CSS localizado e atualiza cores/alfas em alterações de configuração e tema efetivo.
- A transparência abrange **todos os fundos da janela principal**, inclusive barra de cabeçalho, editor e lateral. Textos, ícones e controles permanecem totalmente opacos.
- Não aplicar `set_opacity` à janela inteira.
- Não impor transparência em diálogos, popovers nem preferências sem decisão específica.
- Não alterar a região de entrada de eventos: sem click-through.
- Testar seleção, cursor, foco, contraste e superfícies sobrepostas, evitando composições de RGBA que produzam aparência irregular.
- Faixa inicial sugerida: **0–70%**, dependente de validação de legibilidade e acessibilidade; 0% corresponde a fundo opaco.

### 7.4. Janela, atalhos e GNOME

- Persistir tamanho e maximização, **não coordenadas absolutas**, em conformidade com Wayland.
- Restaurar última nota pelo caminho/nome; se indisponível, iniciar em estado seguro.
- Atalhos propostos: `Ctrl+N`, `Ctrl+F`, `F9`, `Ctrl+,`; verificar conflito com foco e localização.
- Atalho global externo: usuário configura no GNOME; o aplicativo só oferece instalação/comando de inicialização.
- Sempre no topo: recurso **manual** do menu/gerenciador de janelas GNOME, sem API de automação nem dependência de extensão no MVP.

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
| `background-transparency` | inteiro 0–70 | 0 | Preferência |
| `sidebar-visible` | boolean | true | Preferência |
| `window-width` | inteiro positivo | 800 | Estado |
| `window-height` | inteiro positivo | 560 | Estado |
| `window-maximized` | boolean | false | Estado |
| `last-note` | string opcional/vazia | vazio | Estado |

- Esquema proposto: `io.github.rafaelcarvalho89.oaknote`.
- Chaves versionadas e distribuídas com os arquivos de instalação.
- O domínio não depende da API GSettings.
- Valores inválidos, referências perdidas e configurações antigas recebem tratamento seguro.
- A pasta escolhida deve ser persistida sem depender de um caminho hardcoded no host; quando obtida via portal, verificar validade do identificador/caminho e autorização ao reiniciar.
- Persistência precisa ser testada tanto na execução local quanto dentro do Flatpak.

## 9. Flatpak, empacotamento e diretórios

### 9.1. Identidade

- Produto: Oak Note; crate e binário `oak-note`.
- Application ID e identificadores de desktop/AppStream: `io.github.rafaelcarvalho89.oaknote`.
- Manifesto principal versionado em `flatpak/`.
- Runtime `org.gnome.Platform` e SDK `org.gnome.Sdk` na mesma linha compatível, versão selecionada **antes de M1**; GNOME 50 da POC é **referência candidata**, não escolha final.
- `Cargo.lock` fixado; fontes resolvidas antecipadamente para build Cargo offline no SDK.
- Saída para GitHub Releases após validação e aprovação manual.
- Não presumir atualização automática por distribuição de um arquivo `.flatpak` avulso.

### 9.2. Permissões

- Começar com Wayland e mínimo necessário ao uso local.
- **Notas em XDG Documentos por padrão:** avaliar permissão de escrita `xdg-documents` no Flatpak para acesso inicial, reconhecendo seu alcance sobre a pasta inteira; não conceder acesso irrestrito a `home` somente por conveniência.
- **Pasta alternativa:** seleção explícita via GTK FileDialog/portal, verificando autorização persistente para listar/criar/renomear/gravar e delete; validar sem presumir garantias nativas de atomicidade em FUSE.
- Exportação por portal de seleção de arquivos/diretórios com permissões efetivas verificadas.
- Sem telemetria, acesso à rede ou serviços externos como requisito funcional.
- Inspecionar permissões reais do pacote e overrides do ambiente de testes.

### 9.3. Build e release

Pipeline proposto: `cargo fmt --check`, `cargo clippy`, `cargo test`, verificação de build Flatpak/manifesto e testes manuais no GNOME/Wayland antes do release. O processo de liberação verifica licença MIT, changelog, metadados AppStream e caminho de instalação.

## 10. Qualidade, testes e diagnósticos

### 10.1. Testes unitários e de integração

| Categoria | Cenários prioritários |
| --- | --- |
| Títulos | caracteres permitidos, vazios, colisões, normalização, Unicode, extensão `.md` |
| Domínio | seleção/revisão, regras independentes de GTK, erros previsíveis |
| Salvamento | debounce, reordenação de resultados, revisão nova durante I/O, retry |
| DT-05 — sincronização | Escrita/flush parcial, sync do temporário, publicação atômica quando suportada, sync do diretório, erro antes/depois do rename, resultado incerto, ambiente/portal sem sync disponível, buffer e estado coerentes |
| Filesystem | escrita temporária, substituição, falha simulada, arquivo removido, acesso negado |
| DT-03 — lixeira | Confirmação/cancelamento, sucesso, lixeira não suportada, acesso negado, pasta via portal, item alterado externamente, nenhuma exclusão permanente como fallback |
| DT-04 — instância e serialização | Lançamento repetido na mesma sessão; reativação da janela mantendo buffer, conflitos e autosave; concorrência de gravação/renomeação/lixeira/migração; recuperação após encerramento; edição externa ainda detectada |
| Conflito | alteração externa, arquivo removido/renomeado, autosave bloqueado, recarregar com confirmação, salvar como nova nota respeitando DT-01, cancelar e preservar buffer; falha de I/O durante resolução |
| Busca/ordem | correspondência por título, mudança de mtime e empates |
| Preferências | defaults, leitura/atualização, restauração, pasta de notas e entrada inválida |
| Diretório de notas | XDG Documentos em instalações pt-BR/en-US; pasta inexistente, acesso negado, seleção alternativa, restauro após reinício, arquivos `.md` preexistentes |
| DT-19 — confirmação | Pasta com `.md` preexistente na raiz no primeiro acesso: aceitar, escolher outra pasta, cancelar, pasta vazia, subdiretórios ignorados e não modificação dos arquivos |
| Troca de pasta | Escolha de manter, escolha de migrar, cancelamento, arquivos de mesmo nome, edições pendentes e falhas parciais sem perda silenciosa |
| DT-20 — cópia/validação | Cópia integral com verificação de bytes/hash; nenhuma ativação com cópia incompleta, origem modificada, destino ocupado ou acesso negado; arquivos de origem intactos durante a transferência |
| DT-20 — ativação/limpeza | Confirmação adicional para remoção, revalidação antes de cada exclusão, cancelamento da limpeza, falha parcial de remoção e persistência correta do diretório ativo |

| Exportação | conteúdo preservado, destino ocupado, cancelamento e falha parcial |

Os testes de integração de filesystem devem usar diretórios isolados e não depender do diretório real do usuário.

### 10.2. Testes manuais GTK/GNOME

- Tema do sistema e troca em tempo real; clareamento/escurecimento correto.
- Fundo com transparência em cabeçalho, lista e editor; sem transparência dos controles.
- Texto, foco, seleção e cursor legíveis com conteúdos sobrepostos de fundo variados.
- Layout compacto, redimensionamento, maximização e restauração.
- Navegação por teclado, atalhos, Orca e mensagens de erro.
- GNOME manual “Sempre no topo”, funcionamento quando outra janela está ativa.
- Instalação/execução Flatpak e acessos autorizados, inclusive leitura/gravação em XDG Documentos, escolha de pasta externa e exportação via portal.

### 10.3. Logs e privacidade

Eventos de erro e operação podem ser estruturados em logs locais. **Não registrar corpo das notas, conteúdos selecionados, caminhos externos sensíveis sem necessidade, credenciais ou informações do clipboard.** Não há telemetry endpoint.

## 11. Rastreabilidade de implementação

| Requisitos de produto | Módulos/contratos predominantes | Teste principal |
| --- | --- | --- |
| RF-01–08 | feature `notes` + porta de repositório seletiva | unitário e integração de filesystem |
| RF-09–14 | feature `notes` (SaveCoordinator + adaptador), incluindo estágios de persistência DT-05 | unitário, integração, falhas antes/depois da publicação e estados de durabilidade |
| RF-15–22 | features `notes`/`appearance`/`storage` + GSettings | teste manual de interface |
| RF-23 | feature `export` + portal | integração e teste manual |
| RF-24 | feature de UI/localização + gettext | checagem de tradução e teste manual |
| RF-25 | flatpak + CI | build, instalação e smoke test |
| RF-26 | feature de storage + GSettings + portal | seleção, persistência e permissões no Flatpak |
| RF-27 | composição da aplicação + GApplication/AdwApplication + serviço de notas | ativações repetidas, buffer preservado e serialização de operações |
| RNF-01–16 | transversal, conforme área | matriz de testes e revisão |

## 12. Decisões técnicas pendentes — não aprovadas

**Decisões concluídas:** **DT-01 — B:** sugerir nome alternativo editável em colisões, sem sobrescrever. **DT-17 — C:** oferecer manter notas na origem ou migrá-las. **DT-19 — B:** solicitar confirmação inicial para reconhecer `.md` preexistentes na raiz. **DT-20 — B:** copiar e validar integralmente antes de ativar a nova pasta; pedir confirmação adicional e revalidar antes de remover originais. **DT-02 — B:** em conflito externo, bloquear sobrescrita e oferecer recarregar com confirmação, salvar edição local como nova nota ou cancelar preservando o buffer e o estado de conflito. **DT-03 — A:** mover a nota para lixeira após confirmação, sem exclusão definitiva caso falhe ou não seja suportado. **DT-04 — A:** instância única por sessão da aplicação; novas ativações acionam a janela existente; operações internas serializadas, sem presumir proteção global contra editores externos. **DT-05 — B:** sincronizar dados do temporário, publicar via substituição atômica quando suportada e sincronizar o diretório quando disponível, distinguindo publicação de durabilidade verificada e resultados incertos. Permanecem pendentes os detalhes técnicos de detecção, recuperação e permissões, sem alterar essas decisões aprovadas.

| Prioridade | ID | Decisão a tomar | Alternativas a investigar |
| --- | --- | --- | --- |

| Alta | DT-06 | Comportamento de saída/troca com alteração não salva | impedir fluxo / preservar temporário ou cópia / confirmação explícita |
| Alta | DT-07 | Política de exportação com conflitos e falhas parciais | abortar / continuar e relatar / manter arquivos anteriores |
| Alta | DT-08 | Versão específica do runtime GNOME e toolchain | matriz de compatibilidade e manutenção |
| Média | DT-09 | Fingerprint de edição externa | metadados + hash / verificação mais forte |
| Média | DT-10 | Intervalo e contraste de transparência; superfícies excepcionais | faixa 0–70% / limite menor mediante teste |
| Média | DT-11 | Dimensões mínimas e comportamento responsivo | testes de layout GNOME |
| Média | DT-12 | Padrões de atalhos e busca de título (Unicode/casefold) | GNOME HIG + testes |
| Média | DT-13 | Ordem estável em empate de timestamp e arquivos sem mtime | título/caminho como desempate |
| Média | DT-14 | Política de arquivos inesperados e symlinks | ignorar e notificar / validar |
| Média | DT-15 | Valores finais das chaves GSettings e migração | defaults e versionamento |
| Média | DT-16 | Mecanismo de execução de I/O e retorno ao contexto GTK | GLib assíncrono / worker controlado |
| Alta | DT-18 | Permissões e operações na pasta padrão e alternativa com Flatpak | `xdg-documents` + portal; verificar persistência e escrita atômica em FUSE |
| Média | DT-21 | Como lembrar a confirmação inicial da DT-19 por pasta | GSettings com referência à pasta / confirmação por sessão; sem criar índice de notas |

**Recomendação de refinamento:** DT-01, DT-17, DT-19, DT-20, DT-02, DT-03 e DT-04 estão aprovadas. A primeira rodada de decisões críticas de integridade e a DT-05 foram aprovadas; **próxima decisão: DT-06 — saída/troca com alterações pendentes**, seguida de DT-09 — identificação de alterações externas e DT-18 — permissões de armazenamento Flatpak. Validar DT-18 em POC técnica direcionada **dentro do próprio M1**, se necessário para comprovar suporte a permissões e atomicidade no Flatpak, sem assumir garantias antecipadas.

## 13. Critérios de conclusão da PREP-02

Para declarar a PREP-02 aprovada, precisamos:
1. Verificar cobertura de RF-01–RF-27 e RNF-01–RNF-16.
2. Incorporar as DT-01, DT-17, DT-19, DT-20, DT-02, DT-03, DT-04 e DT-05 aprovadas; verificar limites de instância única, salvamento reforçado e conflitos externos, refinando as demais decisões conforme risco antes do backlog.
3. Estabelecer contratos operacionais de salvamento, renomeação, exclusão, troca de pasta e exportação suficientemente testáveis.
4. Definir a versão de runtime/SDK e principais dependências durante o planejamento, antes da implementação que delas dependa; considerar a validação de permissões e publicação atômica em pastas selecionadas.
5. Definir limites verificáveis de acessibilidade e falhas.
6. Confirmar que nenhum requisito foi inventado além da PREP-01.
7. Revisar este documento e aceitar sua baseline antes do backlog INVEST na PREP-03.

**Situação:** minuta técnica para revisão. Não é autorização para desenvolver o MVP.

## 14. Referências internas

- [Especificação do Produto (PREP-01)](product-specification.md)
- ADR-01–ADR-07: decisões aprovadas nesta fase de planejamento, ainda a registrar individualmente em `.docs/architecture/`.
- Relatórios de POC: fonte de experimentação anterior; validação manual complementar confirmada pelo mantenedor. Não transportar arquivos da POC para a `main` sem revisão expressa.

# Oak Note — Especificação Técnica e Arquitetura (MVP v0.1.0)

> **Estado:** RASCUNHO DE REFINAMENTO — PREP-02, pendente de aprovação dos pontos abertos.  
> **Data:** 2026-11-16  
> **Referência de produto:** [PREP-01 — Especificação do Produto](product-specification.md)  
> **Identificador:** `io.github.rafaelcarvalho89.oaknote`  
> **Escopo desta entrega:** somente planejamento/documentação; nenhuma implementação.

## 1. Objetivo, contexto e situação do repositório

Traduzir os requisitos RF-01–RF-26 e RNF-01–RNF-16 da PREP-01 em desenho técnico verificável, mantendo as sete ADRs aprovadas como restrições. Não antecipar código de implementação nem converter propostas ainda pendentes em decisões aprovadas.

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

**Baseline adicional:** Debian/GNOME/Wayland; Rust + GTK4 + libadwaita; POC validada manualmente pelo responsável; sobreposição exclusivamente manual pelo GNOME; notas offline em **Documentos por padrão** com pasta configurável; localização pt-BR/en-US; GitHub Releases para Flatpak. **DT-01 aprovada:** sugerir nome alternativo editável em colisões. **Diretriz transversal:** os projetos de produção seguem organização feature-based; aplicar princípios hexagonais, SOLID, YAGNI e DRY somente de forma útil e compatível com o contexto.

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
Clean       # memória corresponde à revisão persistida
Dirty       # alteração em memória não persistida
Scheduled   # debounce aguardando
Saving      # operação de persistência em andamento
Error       # última tentativa falhou, conteúdo preservado
Conflict    # alteração externa impede publicação automática
```

O usuário precisa distinguir **pendente**, **salvando**, **salvo**, **falha** e **conflito**. Estados podem ser simplificados na UI desde que nenhuma revisão não persistida seja indicada como salva.

### 5.2. Sequência nominal

1. Editor emite alteração e o `SaveCoordinator` incrementa `revision` da nota ativa.
2. Debounce cancela/substitui agendamentos anteriores; **700 ms é valor inicial sugerido**, sujeito a validação.
3. Ao disparar, coleta um snapshot imutável do conteúdo e revisão.
4. Confere estado esperado em disco antes de publicar (fingerprint/versão esperada).
5. Infraestrutura escreve **arquivo temporário no mesmo diretório**, confere erro de escrita/flush e realiza substituição atômica quando permitida.
6. Registra resultado da revisão gravada. Se surgiram edições mais recentes, mantém estado pendente e agenda nova gravação.
7. Comunica resultado ao contexto GTK sem bloquear o loop principal.

### 5.3. Garantias e limites

- Apenas uma operação de escrita de uma nota pode publicar por vez; operações de renomeação e exclusão da mesma nota também devem participar da coordenação.
- Escrita atrasada nunca marca como salva uma revisão mais nova.
- Falhas preservam o buffer e deixam a revisão não salva; **não há garantia de recuperar memória após crash** sem journal/snapshot, que estão fora do MVP.
- Evitar sobrescrever outro arquivo em caso de colisão de nome.
- Verificar mudanças externas antes de operação destrutiva; **checagem separada e posterior rename não são, por si só, garantia contra TOCTOU**. PREP-02 deve escolher e testar uma política segura diante de múltiplos processos; não prometer segurança absoluta sem mecanismos do SO.
- Publicação por rename atômico não implica durabilidade pós-perda de energia. Investigar flush/sync de arquivo e diretório e a garantia real do filesystem de referência.
- A implementação não deve deixar temporários órfãos com conteúdo sensível, sempre que limpeza segura seja possível. Eventuais resíduos após crash devem ser tratados sem arriscar notas válidas.
- Uma aplicação secundária ou edição por terceiro não autoriza sobrescrita silenciosa.

### 5.4. Troca de nota, encerramento e erros

A aplicação tenta concluir gravações pendentes na troca e no encerramento. Em caso de falha, oferece uma decisão explícita (por exemplo, manter edição aberta e tentar novamente, cancelar a operação ou descartar conscientemente). Um diálogo de falha **não garante** retenção após matar o processo.

**Ponto de decisão:** a estratégia exata de gestão da saída e seleção com `Dirty/Error/Conflict` exige protótipo de interação e teste GTK. Evitar promessa de bloquear infalivelmente o encerramento pelo gerenciador de janelas.

### 5.5. Fingerprints e modificações externas (QA-05/QA-07)

**Proposta:** um fingerprint efêmero composto por informações disponíveis do arquivo (por exemplo, tamanho, modificação, identidade do objeto e/ou hash) permite detectar mudanças usuais. Não é chave estável de domínio e não substitui coordenação atômica entre processos.

Quando houver conflito, o sistema preservará a edição em memória e deverá disponibilizar forma segura de continuar. **Estratégia de resolução ainda não aprovada**: recarregar após confirmação, salvar como nova nota ou outra ação explícita. Nunca descartar silenciosamente a edição do usuário.

## 6. Casos de uso e fluxo de operações

| Caso de uso | Camada de aplicação | Infraestrutura | Erro/contingência |
| --- | --- | --- | --- |
| Criar | Validar título, sugerir alternativa editável em colisão, coordenar seleção e salvar | Criação sem sobrescrita na pasta ativa | Nome duplicado, I/O |
| Listar | Ordenar e filtrar metadados, respeitando a confirmação inicial DT-19 | Detecção/enumeração controlada sem mutação | Entrada inválida, recusa ou escolha de outra pasta |
| Selecionar | Proteger edição pendente anterior | Leitura condicionada | Arquivo removido |
| Editar | Atualizar revisão, agendar autosave | Nenhuma escrita direta pela UI | Falha posterior de gravação |
| Renomear | Coordenar salvamento e referência mutável | Renomeação sem colisão | Conflito externo |
| Excluir | Confirmar, verificar revisão e remover | Operação condicionada | Cancelamento/falha |
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

### 6.2. Exclusão (QA-06)

**Aberto:** mover para lixeira ou apagar definitivamente. O MVP deve confirmar exclusão e preservar outros arquivos, mas não se deve presumir recuperação se o armazenamento privado não fornecer integração de lixeira. A decisão afeta UX e testes de erro.

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
| Filesystem | escrita temporária, substituição, falha simulada, arquivo removido, acesso negado |
| Conflito | modificação externa, destino ocupado, rename/delete concorrentes |
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
| RF-09–14 | feature `notes` (SaveCoordinator + adaptador) | unitário, integração e falhas |
| RF-15–22 | features `notes`/`appearance`/`storage` + GSettings | teste manual de interface |
| RF-23 | feature `export` + portal | integração e teste manual |
| RF-24 | feature de UI/localização + gettext | checagem de tradução e teste manual |
| RF-25 | flatpak + CI | build, instalação e smoke test |
| RF-26 | feature de storage + GSettings + portal | seleção, persistência e permissões no Flatpak |
| RNF-01–16 | transversal, conforme área | matriz de testes e revisão |

## 12. Decisões técnicas pendentes — não aprovadas

**Decisões concluídas:** **DT-01 — alternativa B:** para título duplicado, sugerir nome disponível e permitir aceitar ou editar, sem sobrescrever. **DT-17 — alternativa C:** ao trocar a pasta ativa, oferecer escolha entre **manter notas na origem** ou **migrá-las**, sem movimentação silenciosa. **DT-19 — alternativa B:** confirmar no primeiro acesso o reconhecimento de arquivos `.md` preexistentes na raiz ou escolher outra pasta, sem recursão e sem alterar os arquivos. **DT-20 — alternativa B:** copiar todas as notas e validar integralmente antes de ativar o novo diretório; remover os originais somente após confirmação explícita adicional e verificações de segurança. Não prometer atomicidade global; detalhar recuperação de falhas parciais. A persistência da confirmação DT-19 será refinada separadamente.

| Prioridade | ID | Decisão a tomar | Alternativas a investigar |
| --- | --- | --- | --- |

| Crítica | DT-02 | Resolução de conflitos externos sem sobrescrita | bloquear e oferecer recarregar/salvar como / cópia |
| Crítica | DT-03 | Exclusão de nota | lixeira / exclusão definitiva com confirmação |
| Crítica | DT-04 | Proteção contra concorrência de processos e operações filesystem | instância única / coordenação operacional / bloqueio e detecção |
| Alta | DT-05 | Política exata de debounce, flushing, sincronização e confirmação de salvamento | limites e operações suportadas pelo SO |
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

**Recomendação de refinamento:** DT-01, DT-17, DT-19 e DT-20 estão aprovadas. Prosseguir para **DT-02 — resolução de conflitos externos**, depois DT-03 e DT-04, uma questão de integridade por vez. Validar DT-18 em POC técnica direcionada **dentro do próprio M1**, se necessário para comprovar suporte a permissões e atomicidade no Flatpak, sem assumir garantias antecipadas.

## 13. Critérios de conclusão da PREP-02

Para declarar a PREP-02 aprovada, precisamos:
1. Verificar cobertura de RF-01–RF-26 e RNF-01–RNF-16.
2. Incorporar as DT-01, DT-17, DT-19 e DT-20 aprovadas e resolver DT-02–04, sem reabrir as ADRs desnecessariamente.
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

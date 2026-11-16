# Oak Note — Especificação Técnica e Arquitetura (MVP v0.1.0)

> **Estado:** RASCUNHO DE REFINAMENTO — PREP-02, pendente de aprovação dos pontos abertos.  
> **Data:** 2026-11-16  
> **Referência de produto:** [PREP-01 — Especificação do Produto](product-specification.md)  
> **Identificador:** `io.github.rafaelcarvalho89.oaknote`  
> **Escopo desta entrega:** somente planejamento/documentação; nenhuma implementação.

## 1. Objetivo, contexto e situação do repositório

Traduzir os requisitos RF-01–RF-25 e RNF-01–RNF-16 da PREP-01 em desenho técnico verificável, mantendo as sete ADRs aprovadas como restrições. Não antecipar código de implementação nem converter propostas ainda pendentes em decisões aprovadas.

Em 2026-11-16, a `main` do repositório `RafaelCarvalho89/oak-note` foi verificada contendo apenas `.gitignore`, `LICENSE` e `README.md`. A POC havia sido retirada da branch principal pelo mantenedor. A documentação nova não deve copiar arquivos experimentais da branch antiga. A base de implementação do MVP deverá ser estabelecida somente em M1.

## 2. Restrições arquiteturais aprovadas (ADRs)

| ADR | Restrição |
| --- | --- |
| 01 | Nome físico do arquivo Markdown derivado do título; renomear título pode alterar caminho. |
| 02 | Markdown puro, sem front matter ou índice auxiliar; título e modificação derivados do filesystem; sem ID interno persistente e sem garantia da data histórica de criação. |
| 03 | Autosave com debounce, arquivo temporário no mesmo diretório, substituição atômica quando suportada, controle de revisões/concorrência e erros explícitos. |
| 04 | Esquema GSettings único, separação lógica entre preferências e estado operacional. |
| 05 | Monólito modular de um crate, domínio independente, abstrações seletivas nas fronteiras úteis. |
| 06 | Application ID definitivo `io.github.rafaelcarvalho89.oaknote`, Flatpak com runtime GNOME versionado e atualizações controladas. |
| 07 | Transparência em **todos os fundos da janela principal**, incluindo cabeçalho, editor e barra lateral; textos e controles opacos. |

**Baseline adicional:** Debian/GNOME/Wayland; Rust + GTK4 + libadwaita; POC validada manualmente pelo responsável; sobreposição exclusivamente manual pelo GNOME; notas offline; localização pt-BR/en-US; GitHub Releases para Flatpak.

## 3. Organização do projeto e direção das dependências

### 3.1. Estrutura proposta (não implantada)

```text
oak-note/
├── .docs/
│   ├── architecture/                 # ADRs consolidadas futuramente
│   └── mvp/
│       ├── product-specification.md
│       └── technical-specification.md
├── src/
│   ├── main.rs                       # inicialização/composição
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── note.rs
│   │   └── errors.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── note_service.rs
│   │   ├── note_repository.rs        # trait seletiva
│   │   └── save_coordinator.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── filesystem.rs
│   │   ├── settings.rs
│   │   └── diagnostics.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.rs
│       ├── note_list.rs
│       ├── editor.rs
│       ├── appearance.rs
│       └── preferences.rs
├── data/                             # schema GSettings, desktop, AppStream, ícones, traduções
├── flatpak/
├── tests/
├── Cargo.toml
└── Cargo.lock
```

Os diretórios são indicativos. O desenho aprovado exige **um crate**, não que cada arquivo acima exista. Evitar modularização sem caso de uso.

### 3.2. Limites de módulos

- **domain:** `Note`, regras de título/caminho e erros próprios; não importa `gtk`, `adw`, `gio::Settings` nem manipula arquivos.
- **application:** coordena casos de uso, seleção/edição, revisão de conteúdo, estado de salvamento, conflitos, exportação e transições entre notas; depende de domínio e abstrações selecionadas.
- **infrastructure:** implementação concreta do repositório de arquivos, operações de I/O seguro, persistência de preferências e logs; não conhece widgets.
- **ui:** renderiza estados, recebe eventos GTK4 e chama casos de uso; gerencia CSS, libadwaita e interação do usuário, sem abrir/gravar notas diretamente.
- **composition root:** constrói dependências concretas e conecta sinais da interface.

**Regra de dependência:** `domain` não depende de `application`, `infrastructure` ou `ui`. Os contratos usados pelo núcleo são definidos próximos aos consumidores, não duplicados automaticamente.

### 3.3. Abstração seletiva proposta

Uma trait `NoteRepository` é candidata, cobrindo operações sem carregar detalhes GTK:

```text
list_notes() -> Result<list<NoteMetadata>, RepositoryError>
read_note(path) -> Result<NoteSnapshot, RepositoryError>
create_note(title, content) -> Result<NoteSnapshot, RepositoryError>
replace_if_unchanged(path, expected_fingerprint, content) -> Result<NewFingerprint, RepositoryError>
rename_if_available(source, target, expected_fingerprint) -> Result<NoteSnapshot, RepositoryError>
delete_note(path, expected_fingerprint) -> Result<(), RepositoryError>
export_notes(destination) -> Result<ExportReport, RepositoryError>
```

**Pseudocontrato, não API Rust final.** `expected_fingerprint` representa o estado observado, e não uma identidade persistente da nota. A camada de aplicação coordena o salvamento; filesystem executa operações condicionais seguras conforme disponibilidade do SO.

Uma implementação real de filesystem e um repositório falso de teste podem satisfazer a interface. Não criar uma trait só para encapsular GSettings sem benefício mensurável.

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

### 4.2. Diretório gerenciado

**Proposta:** utilizar `$XDG_DATA_HOME/oak-note/notes/` para execução fora de Flatpak, resolvido pelas bibliotecas XDG/GIO conforme o ambiente. Dentro do Flatpak, preferir o local de dados privado disponibilizado para a aplicação, sem presumir equivalência literal com um caminho no host.

Ao iniciar, resolver caminho de dados, criar o diretório quando necessário com permissões apropriadas e lidar com diretório indisponível com erro apresentado. Não varrer indiscriminadamente o diretório pessoal.

### 4.3. Título e nome físico (QA-01/QA-02)

**Proposta para avaliação:** derivar nome a partir de título legível, preservando Unicode válido, normalizando espaços periféricos, impedindo `/`, NUL, `.`, `..`, títulos vazios e nomes reservados pela política do produto. Manter extensão `.md` minúscula como convenção; preservar legibilidade em vez de sempre aplicar slug ASCII.

**Não definido:** política de colisões — rejeitar com aviso, sugerir título alternativo ou acrescentar sufixo automaticamente. Em qualquer alternativa, *não substituir arquivo existente*. A regra deve ser decidida antes de M2.

Listar somente arquivos regulares apropriados do diretório de notas. Definir política sobre symlinks e arquivos não regulares antes do desenvolvimento; não presumir que entradas externas sejam seguras.

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
| Criar | Validar título, coordenar seleção e salvar | Criação sem sobrescrita | Nome duplicado, I/O |
| Listar | Ordenar e filtrar metadados | Enumeração controlada | Entrada inválida/inacessível |
| Selecionar | Proteger edição pendente anterior | Leitura condicionada | Arquivo removido |
| Editar | Atualizar revisão, agendar autosave | Nenhuma escrita direta pela UI | Falha posterior de gravação |
| Renomear | Coordenar salvamento e referência mutável | Renomeação sem colisão | Conflito externo |
| Excluir | Confirmar, verificar revisão e remover | Operação condicionada | Cancelamento/falha |
| Buscar | Filtrar notas carregadas por título | Não exige índice | Nenhum resultado |
| Exportar | Coordenar lista e progresso | Cópia `.md` com destino do usuário | Parcial, colisões, permissões |

### 6.1. Exclusão (QA-06)

**Aberto:** mover para lixeira ou apagar definitivamente. O MVP deve confirmar exclusão e preservar outros arquivos, mas não se deve presumir recuperação se o armazenamento privado não fornecer integração de lixeira. A decisão afeta UX e testes de erro.

### 6.2. Exportação (QA-11)

**Proposta:** o usuário escolhe diretório usando seletor/portal adequado; exportar arquivos Markdown sem alterar originais, preservando títulos quando seguros. O relatório final identifica sucesso, falhas e cancelamento; nenhuma cópia existente deve ser sobrescrita silenciosamente. Definir antes da implementação se a operação é tudo-ou-nada ou permite resultado parcial.

## 7. Interface GTK4/libadwaita

### 7.1. Composição

Janela principal com barra de cabeçalho, lista lateral recolhível e editor `GtkTextView` simples, sem renderização Markdown. Preferências usam janela libadwaita nativa. Escolhas de UI nativa devem ser compatíveis com versões de SDK selecionadas.

A UI apresenta estados de notas e salvamento, confirma exclusão, informa conflito e abre seletor de diretório na exportação. A navegação por teclado é requisito, não adição opcional.

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
- Notas em área privada da aplicação; não conceder acesso irrestrito a `home` somente por conveniência.
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
| Preferências | defaults, leitura/atualização, restauração e entrada inválida |
| Exportação | conteúdo preservado, destino ocupado, cancelamento e falha parcial |

Os testes de integração de filesystem devem usar diretórios isolados e não depender do diretório real do usuário.

### 10.2. Testes manuais GTK/GNOME

- Tema do sistema e troca em tempo real; clareamento/escurecimento correto.
- Fundo com transparência em cabeçalho, lista e editor; sem transparência dos controles.
- Texto, foco, seleção e cursor legíveis com conteúdos sobrepostos de fundo variados.
- Layout compacto, redimensionamento, maximização e restauração.
- Navegação por teclado, atalhos, Orca e mensagens de erro.
- GNOME manual “Sempre no topo”, funcionamento quando outra janela está ativa.
- Instalação/execução Flatpak e acessos autorizados, inclusive exportação via portal.

### 10.3. Logs e privacidade

Eventos de erro e operação podem ser estruturados em logs locais. **Não registrar corpo das notas, conteúdos selecionados, caminhos externos sensíveis sem necessidade, credenciais ou informações do clipboard.** Não há telemetry endpoint.

## 11. Rastreabilidade de implementação

| Requisitos de produto | Módulos/contratos predominantes | Teste principal |
| --- | --- | --- |
| RF-01–08 | domain + application + NoteRepository + lista | unitário e integração de filesystem |
| RF-09–14 | SaveCoordinator + repository + estados UI | unitário, integração e falhas |
| RF-15–22 | ui + infrastructure/settings | teste manual de interface |
| RF-23 | application + infrastructure + portal | integração e teste manual |
| RF-24 | ui + gettext | checagem de tradução e teste manual |
| RF-25 | flatpak + CI | build, instalação e smoke test |
| RNF-01–16 | transversal, conforme área | matriz de testes e revisão |

## 12. Decisões técnicas pendentes — não aprovadas

| Prioridade | ID | Decisão a tomar | Alternativas a investigar |
| --- | --- | --- | --- |
| Crítica | DT-01 | Normalização de títulos, Unicode e colisões | rejeitar colisão / oferecer sugestão / sufixar |
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

**Recomendação de refinamento:** decidir DT-01 a DT-04 primeiro, uma decisão por interação, por afetarem garantias de integridade e contratos. As demais podem ser agrupadas nas histórias e validações dos marcos quando não alterarem o desenho.

## 13. Critérios de conclusão da PREP-02

Para declarar a PREP-02 aprovada, precisamos:
1. Verificar cobertura de RF-01–25 e RNF-01–16.
2. Resolver as decisões críticas DT-01–04, sem reabrir as ADRs desnecessariamente.
3. Estabelecer contratos operacionais de salvamento, renomeação, exclusão e exportação suficientemente testáveis.
4. Definir a versão de runtime/SDK e principais dependências durante o planejamento, antes da implementação que delas dependa.
5. Definir limites verificáveis de acessibilidade e falhas.
6. Confirmar que nenhum requisito foi inventado além da PREP-01.
7. Revisar este documento e aceitar sua baseline antes do backlog INVEST na PREP-03.

**Situação:** minuta técnica para revisão. Não é autorização para desenvolver o MVP.

## 14. Referências internas

- [Especificação do Produto (PREP-01)](product-specification.md)
- ADR-01–ADR-07: decisões aprovadas nesta fase de planejamento, ainda a registrar individualmente em `.docs/architecture/`.
- Relatórios de POC: fonte de experimentação anterior; validação manual complementar confirmada pelo mantenedor. Não transportar arquivos da POC para a `main` sem revisão expressa.

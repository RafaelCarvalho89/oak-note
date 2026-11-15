# Oak Note POC — 00: Bootstrap da aplicação

## Contexto

Estamos desenvolvendo o **Oak Note**, um aplicativo Open Source, nativo para Linux Debian, utilizando Rust + GTK4 + libadwaita.

O produto final será um bloco de anotações Markdown com múltiplas notas, salvamento automático, temas e transparência de fundo.

O repositório atual, `oak-note-poc`, é uma **cópia independente do repositório original** `oak-note`, destinada exclusivamente à investigação técnica.

O objetivo da POC é validar as tecnologias e comportamentos críticos antes de implementar o MVP.

**Ambiente principal:** Debian + GNOME (Wayland).

## Objetivo desta tarefa

Implementar a **POC-00 — Bootstrap**, entregando uma aplicação Rust + GTK4/libadwaita mínima, compilável e executável.

Esta tarefa deve estabelecer a infraestrutura inicial e preparar o repositório para os experimentos posteriores.

**Não implemente o MVP. Não antecipe funcionalidades das próximas POCs.**

## 1. Inspeção inicial

Antes de modificar o projeto:

1. Inspecione a estrutura atual do repositório.
2. Leia os arquivos existentes, especialmente README.md, LICENSE e .gitignore.
3. Verifique se já existe alguma configuração Cargo ou código Rust.
4. Verifique as versões instaladas de Rust, Cargo, GTK4 e libadwaita, quando possível.
5. Identifique dependências de sistema ausentes.
6. Preserve arquivos e configurações existentes que ainda sejam relevantes.

Não sobrescreva arquivos indiscriminadamente.

Não instale pacotes de sistema ou execute comandos privilegiados sem autorização.

## 2. Inicialização do projeto Rust

Caso o repositório ainda não seja um projeto Cargo:

- Inicialize um projeto binário Rust diretamente na raiz.
- Utilize `oak-note-poc` como nome do pacote.
- Use a edição estável de Rust compatível com o ambiente.
- Configure as dependências mínimas necessárias para GTK4 e libadwaita.
- Mantenha Cargo.lock versionado.
- Atualize .gitignore para ignorar artefatos de build.

Escolha versões compatíveis entre as crates Rust e as bibliotecas nativas presentes no Debian.

Não habilite features que exijam versões superiores às instaladas.

Evite dependências desnecessárias.

## 3. Primeira aplicação GTK4

Crie uma aplicação desktop funcional com:

- `adw::Application` como aplicação principal.
- Uma janela nativa libadwaita.
- Título "Oak Note POC".
- Dimensões iniciais aproximadas de 800 × 560.
- Uma barra de cabeçalho nativa.
- Um `GtkTextView` editável, contendo texto demonstrativo.
- Uma indicação discreta de que o aplicativo é uma prova de conceito.

Utilize um Application ID válido, específico da POC, preferencialmente `io.github.rafaelcarvalho89.OakNotePoc`.

A janela deve permitir redimensionamento e fechamento convencional.

A aplicação deve utilizar widgets nativos GTK4/libadwaita e seguir as convenções básicas do GNOME.

Não implemente persistência, gerenciamento de notas ou editor Markdown completo.

## 4. Organização do código

Priorize uma estrutura mínima:

```text
oak-note-poc/
├── src/
│   └── main.rs
├── .docs/
│   └── poc-plan.md
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── LICENSE
└── README.md
```

Crie módulos adicionais somente quando houver uma necessidade concreta.

Não utilize Cargo Workspace, múltiplos crates, arquitetura hexagonal ou abstrações desnecessárias.

Esta POC tem código experimental e potencialmente descartável.

## 5. Plano de experimentos

Crie `.docs/poc-plan.md`, documentando as seguintes etapas:

**POC-00 — Bootstrap**

- Validar dependências e toolchain.
- Compilar e executar uma janela GTK4/libadwaita.
- Confirmar funcionamento no GNOME/Wayland.

**POC-01 — Transparência seletiva**

- Investigar transparência de fundo via GTK CSS/RGBA.
- Preservar opacidade do texto e dos controles.
- Testar diferentes níveis de transparência.

**POC-02 — Sobreposição manual**

- Investigar o comportamento "Sempre no topo" no GNOME/Wayland.
- Documentar o procedimento manual e as limitações.
- Não presumir que GTK4 controla diretamente a ordem das janelas.

**POC-03 — Aparência e comportamento**

- Investigar modos claro/escuro.
- Validar redimensionamento e legibilidade.
- Avaliar interações entre libadwaita e CSS personalizado.

**POC-04 — Flatpak**

- Avaliar empacotamento.
- Validar execução com sandbox.
- Identificar possíveis diferenças em relação à execução nativa.

Para cada experimento, registre:

- Objetivo e hipótese.
- Procedimento de teste.
- Critérios de aprovação.
- Evidências esperadas.
- Resultado observado.
- Limitações e conclusão.

Experimentos ainda não executados devem permanecer explicitamente como **pendentes**.

Não apresente hipóteses como fatos validados.

## 6. Documentação do repositório

Atualize o README.md para indicar claramente que este repositório é uma prova de conceito do Oak Note.

Inclua:

- Descrição e propósito.
- Tecnologias utilizadas.
- Relação com o projeto principal.
- Pré-requisitos do Debian.
- Instruções de compilação.
- Instruções de execução.
- Comandos de validação de qualidade.
- Referência ao plano de experimentos.
- Limitações conhecidas.
- Licença MIT.

Utilize links para documentação oficial sempre que necessário.

A documentação deve ser objetiva e suficiente para reproduzir o ambiente.

## 7. Verificações obrigatórias

Execute, quando o ambiente permitir:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

Se houver acesso a uma sessão gráfica Wayland, tente verificar a abertura e o encerramento da janela.

Se não houver acesso à interface gráfica, não declare o teste visual como aprovado. Registre que a validação manual permanece pendente.

Se ocorrer erro de compilação ou incompatibilidade, investigue a causa antes de adicionar soluções alternativas.

Não silencie warnings ou erros sem justificativa.

## 8. Fora do escopo

Não implementar nesta tarefa:

- Controle de transparência.
- Janela sempre no topo por código.
- Extensões GNOME Shell.
- Salvamento ou leitura de notas.
- Armazenamento Markdown.
- SQLite ou GSettings.
- Internacionalização.
- Empacotamento Flatpak.
- Testes automatizados de interface.
- GitHub Actions.
- Funcionalidades definitivas do MVP.

As funcionalidades acima pertencem às próximas etapas.

## 9. Critérios de conclusão

Considere a POC-00 concluída somente quando:

1. O repositório estiver configurado como projeto Rust válido.
2. As dependências GTK4/libadwaita estiverem corretamente configuradas.
3. Existir uma aplicação com janela nativa e campo de texto editável.
4. A compilação e as verificações automatizadas estiverem aprovadas.
5. O README documentar como reproduzir a execução.
6. O plano de experimentos estiver registrado.
7. Os testes manuais não executados estiverem identificados como pendentes.

A abertura real da janela somente poderá ser considerada validada mediante evidência de execução em uma sessão gráfica.

## 10. Entrega

Ao finalizar, apresente:

1. Resumo do que foi implementado.
2. Estrutura final dos arquivos.
3. Dependências e versões utilizadas.
4. Comandos executados e seus resultados.
5. Eventuais problemas encontrados.
6. Critérios de aceitação atendidos ou pendentes.
7. Recomendação objetiva para iniciar a POC-01.

Não implemente a POC-01 nesta execução.

Não crie commits nem envie alterações ao GitHub sem autorização.

**Prioridade: simplicidade, execução verificável e documentação das evidências técnicas.**
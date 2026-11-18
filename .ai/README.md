# AI Configuration

## Visão geral

Este diretório centraliza as instruções, configurações e recursos utilizados por agentes de IA neste repositório.

É a fonte oficial de referência para o comportamento e a execução de tarefas por agentes de IA.

## Organização

| Diretório | Responsabilidade |
|---|---|
| `agents/` | Definições de agentes (`SOUL.md`). Hoje: `data-analyst/`, `software-engineer/`, `product-manager/`. |
| `prompts/` | Prompts reutilizáveis. Hoje: `create-prd.md`, `create-techspec.md`, `create-tasks.md`, `analyze-data.md`, `implement-software.md`, `manage-product.md`. |
| `rules/` | Regras e diretrizes que orientam o comportamento dos agentes. |
| `skills/` | Habilidades especializadas. Hoje: `ai-memory/`, `data-analysis/`, `software-engineering/`, `product-management/`. |
| `workflows/` | Procedimentos e fluxos de trabalho (vazio por enquanto). |

## Padrão de implementação

Canônico neste diretório (`.ai/`). Cursor só carrega symlink versionado (git modo `120000`).

| Camada | Caminho canônico | Carregador Cursor |
|---|---|---|
| Identidade | `agents/<papel>/SOUL.md` | nenhum (o command manda ler) |
| Skill | `skills/<atividade>/SKILL.md` | symlink `.cursor/skills/<atividade>` |
| Prompt | `prompts/<verbo>.md` | symlink `.cursor/commands/<verbo>.md` |

Regras:

- A pasta do agente contém só `SOUL.md`. Não colocar `SKILL.md` dentro de `agents/`.
- Não copiar conteúdo para `.cursor/`. Inspecionar com `ls -la` e `git ls-files -s`.
- `Read` e `Glob` seguem o symlink. Dois paths iguais não são duplicação a corrigir.
- Skill `ai-memory` é compartilhada. Não copiar.
- Symlinks novos são **relativos**: `../../.ai/prompts/<verbo>.md` e `../../.ai/skills/<atividade>`.

## Estado atual dos agentes

| Agente | Skill | Command |
|---|---|---|
| `data-analyst` | `data-analysis` | `analyze-data` |
| `software-engineer` | `software-engineering` | `implement-software` |
| `product-manager` | `product-management` | `manage-product` |

- Defeito registrado: pasta `data-analyst` versus `name: data-analysis` no frontmatter do SOUL. Não corrigir o SOUL aqui. Não propagar o defeito. Nos agentes novos, SOUL `name` = pasta (papel); skill `name` = atividade.
- Pack ampliado (financial-analyst, business-strategist, etc.) ainda não nasceu neste repositório.

## Prompts `create-*`

`create-prd`, `create-techspec` e `create-tasks` são workflow CRS. Permanecem fora do SOUL dos agentes. Product Manager e Software Engineer podem apontar para esses commands. Não absorver o texto.

## ai-memory

Escopo deste repositório: `workspace = "oaknote"`, `project = "oak-note"` (`.ai-memory.toml`).

Roteamento compacto em `AGENTS.md` (bloco marcado). Skills gerenciadas em `.agents/skills/ai-memory-*`. Skill operacional do pack em `skills/ai-memory/` (symlink Cursor em `.cursor/skills/ai-memory`).

## Orientações

- Consulte os recursos relevantes antes de executar uma tarefa.
- Respeite as instruções e regras aplicáveis ao contexto.
- Priorize a reutilização dos recursos existentes.
- Mantenha este README atualizado quando a estrutura for alterada.

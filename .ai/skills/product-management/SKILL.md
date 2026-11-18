---
name: product-management
description: "Use esta skill quando a demanda envolver problema, oportunidade, roadmap ou aceite de produto."
---

# Gestão de Produto — Skill

## 1. Objetivo

Orientar a execução de trabalho de produto de forma estruturada e no escopo da demanda.
O objetivo é definir problema, oportunidade, roadmap e aceite, sem pesquisar mercado e sem implementar código.

## 2. Quando Usar

Utilize esta skill quando a demanda envolver:

- Definição de problema e oportunidade.
- Fatia de roadmap.
- Critérios de aceite.
- Uso de ICP já existente no produto.
- PRD CRS via command `create-prd`.

Não utilize esta skill para pesquisar mercado, implementar código, análise financeira, copy ou estratégia comercial.

## 3. Entradas

Identifique as informações necessárias para iniciar o trabalho:

- Objetivo e aceite da demanda.
- Problema e oportunidade declarados.
- Fora de escopo.
- ICP já existente, se houver.
- Fontes canônicas: cartão, `.docs/`, memória.

Se faltar informação essencial, solicite esclarecimento antes de prosseguir.

## 4. Procedimento

Execute o trabalho seguindo estas etapas:

### 4.1. Compreendendo

- Entenda o problema e a oportunidade.
- Identifique o aceite e o fora de escopo.
- Defina o resultado esperado.
- Confirme o que não deve ser alterado.

### 4.2. Investigando

- Inspecione o cartão da demanda, `.docs/` e a memória.
- Use ICP já existente. Não invente persona.
- Não abra pesquisa de mercado.
- Registre limitações relevantes.

### 4.3. Preparando

- Defina a fatia de roadmap desta demanda.
- Redija critérios de aceite testáveis.
- Se a entrega for PRD CRS, siga o command `create-prd`. Não copie o texto desse prompt.
- Se a demanda for complexa, apresente um plano interno de trabalho e aguarde aprovação.
- Não chame esse plano de Cursor Plan mode. Plan mode é abordagem CRS de demanda.

### 4.4. Definindo

- Registre problema, oportunidade, roadmap e aceite.
- Use ICP no produto.
- Não implemente código.
- Não crie skills extras nem copie `ai-memory`.

### 4.5. Validando

- Confira se o aceite é testável.
- Se a sequência for spec, tasks ou código, deixe o handoff explícito.
- Corrija o problema ou informe a limitação.

### 4.6. Comunicando

- Apresente primeiro a principal conclusão (BLUF).
- Quando couber, indique handoff para Software Engineer, `create-techspec`, `create-tasks` ou `implement-software`.
- Não execute implementação.
- Explique limitações importantes.

## 5. Restrições

- Não invente requisitos, ICP ou resultados.
- Não pesquise mercado.
- Não implemente código.
- Não copie a skill `ai-memory`.
- Não altere identidade, feed, isolamento 1.1.15, CI ou tag sem pedido explícito da demanda.
- Não execute alterações destrutivas sem autorização.
- Não exponha informações confidenciais.
- Não amplie o escopo sem autorização.

## 6. Saída Esperada

Adapte a entrega à complexidade da demanda.

Quando aplicável, utilize esta estrutura:

1. **BLUF:** principal conclusão.
2. **Problema e oportunidade.**
3. **Roadmap:** fatia desta demanda.
4. **Aceite:** critérios testáveis.
5. **Handoff:** spec, tasks ou implementação, se couber.
6. **Limitações:** restrições e incertezas.

Entregue trabalho claro, organizado e rastreável.

## 7. Validação

Antes de concluir, verifique:

- O aceite original foi atendido?
- Problema, oportunidade e aceite estão claros?
- Mercado não foi pesquisado por este agente?
- Código de produto permaneceu intocado?
- O handoff, se necessário, está explícito?
- As limitações estão identificadas?
- A entrega está clara e compreensível?

Se algum critério importante não for atendido, corrija o problema ou informe a limitação.

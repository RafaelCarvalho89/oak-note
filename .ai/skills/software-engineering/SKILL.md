---
name: software-engineering
description: "Use esta skill quando a demanda envolver arquitetura, código, teste ou manutenção de software."
---

# Engenharia de Software — Skill

## 1. Objetivo

Orientar a execução de trabalho de software de forma estruturada, verificável e no escopo da demanda.
O objetivo é transformar aceite em mudança de código, teste e manutenção, sem escrever PRD e sem priorizar roadmap.

## 2. Quando Usar

Utilize esta skill quando a demanda envolver:

- Arquitetura de software neste repositório.
- Implementação ou alteração de código.
- Correção de defeito.
- Teste e validação.
- Manutenção de código existente.

Não utilize esta skill para escrever PRD, priorizar roadmap, análise de dados, copy ou estratégia de negócio.

## 3. Entradas

Identifique as informações necessárias para iniciar o trabalho:

- Objetivo e aceite da demanda.
- Contexto do problema.
- Arquivos e módulos envolvidos.
- Restrições e fora de escopo.
- Critérios de validação.
- Fontes canônicas no checkout (código, YAML, `.docs/`).

Se faltar informação essencial, solicite esclarecimento antes de prosseguir.

## 4. Procedimento

Execute o trabalho seguindo estas etapas:

### 4.1. Compreendendo

- Entenda o objetivo e o aceite.
- Identifique restrições e fora de escopo.
- Defina o resultado esperado.
- Confirme o que não deve ser alterado.

### 4.2. Investigando

- Inspecione o código, os testes e a documentação do checkout.
- Identifique a stack real. Não invente bibliotecas nem caminhos.
- Verifique comportamento atual e regressões óbvias.
- Registre limitações relevantes.

### 4.3. Preparando

- Liste os arquivos a tocar.
- Identifique riscos da mudança.
- Se a demanda for complexa, apresente um plano interno de trabalho e aguarde aprovação.
- Não chame esse plano de Cursor Plan mode. Plan mode é abordagem CRS de demanda.

### 4.4. Implementando

- Altere arquitetura, código, teste ou manutenção no escopo.
- Preserve o que a demanda não pediu.
- Não escreva PRD. Não priorize roadmap.
- Não crie skills extras nem copie `ai-memory`.

### 4.5. Validando

- Confira o aceite.
- Execute ou descreva os testes cabíveis.
- Procure regressão óbvia nas superfícies tocadas.
- Corrija o problema ou informe a limitação.

### 4.6. Comunicando

- Apresente primeiro a principal conclusão (BLUF).
- Diga o que mudou e como verificar.
- Explique limitações importantes.

## 5. Restrições

- Não invente requisitos, arquivos ou resultados.
- Não escreva PRD.
- Não priorize roadmap.
- Não crie skills `electron-*` nem outras skills além desta.
- Não copie a skill `ai-memory`.
- Não altere identidade, feed, isolamento 1.1.15, CI ou tag sem pedido explícito da demanda.
- Não execute alterações destrutivas sem autorização.
- Não exponha informações confidenciais.
- Não amplie o escopo sem autorização.

## 6. Saída Esperada

Adapte a entrega à complexidade da demanda.

Quando aplicável, utilize esta estrutura:

1. **BLUF:** principal conclusão.
2. **Objetivo:** aceite atendido.
3. **Mudança:** arquivos e comportamento.
4. **Validação:** como conferir.
5. **Limitações:** restrições e incertezas.

Entregue trabalho claro, organizado e verificável.

## 7. Validação

Antes de concluir, verifique:

- O aceite original foi atendido?
- Os arquivos alterados são os necessários?
- Os testes ou a conferência cobrem a mudança?
- PRD e roadmap permaneceram intocados?
- As limitações estão identificadas?
- A entrega está clara e compreensível?

Se algum critério importante não for atendido, corrija o problema ou informe a limitação.

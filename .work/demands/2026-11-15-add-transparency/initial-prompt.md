# Oak Note POC — 01: Transparência seletiva

## Contexto

Estamos desenvolvendo o **Oak Note**, um aplicativo Open Source nativo para Linux, utilizando Rust + GTK4 + libadwaita.

O repositório atual é `oak-note-poc`, utilizado exclusivamente para validações técnicas antes do desenvolvimento do MVP no repositório principal `oak-note`.

**Ambiente de referência:** Debian + GNOME (Wayland).

A POC-00 foi planejada para estabelecer uma aplicação GTK4/libadwaita mínima, com janela nativa e campo de texto editável.

Esta tarefa corresponde exclusivamente à **POC-01 — Transparência seletiva**.

## 1. Objetivo

Investigar, implementar e validar um mecanismo de transparência ajustável que afete somente o fundo da interface, preservando a opacidade dos textos, ícones e controles.

O resultado esperado é permitir visualizar aplicativos posicionados atrás do Oak Note sem comprometer a legibilidade das anotações.

A implementação será experimental e não deverá ser tratada como solução definitiva antes da validação.

## 2. Inspeção inicial obrigatória

Antes de modificar qualquer arquivo:

1. Inspecione a estrutura atual do repositório.
2. Leia README.md e `docs/poc-plan.md`, caso existam.
3. Analise o código Rust desenvolvido na POC-00.
4. Identifique as versões de gtk4-rs, libadwaita-rs, GTK4 e libadwaita utilizadas.
5. Verifique a configuração da janela principal e a hierarquia dos widgets.
6. Identifique como os estilos CSS podem ser aplicados no projeto atual.
7. Execute `cargo check` para verificar o estado inicial, quando o ambiente permitir.

Preserve a estrutura existente e as funcionalidades da POC-00.

Não reinicialize o projeto Cargo.

Não substitua implementações funcionais sem necessidade.

Caso a POC-00 esteja incompleta, documente o problema e implemente somente os ajustes mínimos necessários para viabilizar a POC-01.

## 3. Hipótese técnica

**H01:** É possível aplicar transparência variável ao fundo de uma aplicação GTK4/libadwaita utilizando CSS com cores RGBA, preservando a opacidade dos widgets de conteúdo no GNOME/Wayland.

Investigue prioritariamente:

- `GtkCssProvider`.
- CSS `background-color` com canal alpha.
- Classes CSS específicas da aplicação.
- Hierarquia de backgrounds no GTK4/libadwaita.
- Fundos próprios de `GtkTextView`, `GtkScrolledWindow`, header bars e containers.
- Diferenças entre transparência de uma superfície e opacidade de um widget.
- Interferências de estilos padrão do libadwaita.

**Restrição importante:** não utilize `set_opacity()` na janela ou em containers que incluam textos e controles para implementar a transparência seletiva.

Esse método pode ser estudado para comparação técnica, mas não deve ser utilizado como solução principal.

Não assuma que alterar o CSS de um único widget produzirá transparência real da janela. Verifique toda a cadeia de renderização.

Utilize APIs disponíveis nas versões efetivamente instaladas, sem exigir atualizações desnecessárias do GTK4 ou libadwaita.

## 4. Implementação experimental

Amplie a interface da POC-00 com os seguintes componentes.

### 4.1 Controle de transparência

Adicione um slider identificado como **"Transparência do fundo"**, com intervalo de 0% a 70%.

Comportamento:

- 0%: fundo completamente opaco.
- 25%: fundo levemente transparente.
- 50%: fundo semitransparente.
- 70%: fundo com transparência elevada.

A conversão esperada é:

`alpha = 1.0 - (transparencia / 100.0)`

Atualize o fundo dinamicamente quando o slider for movimentado.

A configuração inicial deve ser 0%.

Não implemente persistência nesta POC.

### 4.2 Área de demonstração

Mantenha um `GtkTextView` editável contendo texto demonstrativo, incluindo:

- Um título.
- Algumas linhas de texto.
- Uma lista Markdown simples.
- Caracteres claros e escuros para avaliar a legibilidade.

O conteúdo deve continuar editável.

A área de edição deve ser um dos principais elementos avaliados quanto à transparência.

### 4.3 Indicador do valor

Exiba o percentual selecionado próximo ao slider.

O valor deve mudar imediatamente durante a interação.

### 4.4 Preservação da opacidade

O experimento deverá tentar manter:

- Texto do editor totalmente opaco.
- Ícones e botões opacos.
- Texto dos controles opaco.
- Indicadores visuais legíveis.

A transparência deverá afetar exclusivamente os backgrounds selecionados.

Não é obrigatório tornar a header bar transparente. Ela pode permanecer opaca, desde que essa decisão esteja documentada.

Não implemente passagem de cliques através da janela.

## 5. Estratégia técnica

Priorize a abordagem mais simples que permita avaliar a hipótese H01.

Sugestão:

1. Identifique as camadas que desenham backgrounds opacos.
2. Utilize classes CSS específicas para os elementos experimentais.
3. Experimente backgrounds RGBA com diferentes valores de alpha.
4. Atualize o estilo durante a execução.
5. Preserve as cores do conteúdo e dos controles.
6. Verifique se a transparência permite visualizar outros aplicativos atrás da janela.

Evite CSS global desnecessário e alterações indiscriminadas nas classes padrão do libadwaita.

Considere os temas claro e escuro, sem pressupor suporte a variáveis CSS introduzidas em versões posteriores às instaladas.

Se forem necessárias cores distintas por tema, mantenha essa lógica pequena, explícita e documentada.

Não utilize APIs obsoletas do GTK3, como `GdkVisual` ou `gtk_widget_set_app_paintable`.

Se a primeira abordagem não funcionar, investigue alternativas compatíveis com GTK4, registre as tentativas e compare os resultados.

Não esconda falhas recorrendo automaticamente à transparência da janela inteira.

## 6. Testes automatizados

Adicione testes unitários para qualquer lógica independente da interface, especialmente:

- Conversão de percentual para alpha.
- Valor mínimo de transparência.
- Valor máximo de transparência.
- Tratamento de valores fora dos limites, caso a função permita recebê-los.
- Garantia de que 0% corresponde a alpha 1.0.
- Garantia de que 70% corresponde a alpha 0.30, com tolerância adequada para ponto flutuante.

Evite criar abstrações artificiais apenas para aumentar a cobertura.

Não implemente testes automatizados de interface nesta etapa.

## 7. Validação manual obrigatória

A validação funcional deverá ocorrer no **Debian + GNOME (Wayland)**.

Primeiro, confirme o tipo de sessão gráfica, por exemplo, utilizando:

```bash
echo "$XDG_SESSION_TYPE"
echo "$XDG_CURRENT_DESKTOP"
```

Se não houver sessão gráfica acessível, registre os testes visuais como pendentes. Não simule uma aprovação.

### Cenário 01 — Transparência dinâmica

1. Abra o Oak Note POC.
2. Posicione outro aplicativo visível atrás dele.
3. Ajuste o slider para 0%, 25%, 50% e 70%.
4. Verifique se o fundo muda dinamicamente.
5. Observe se o aplicativo atrás da janela torna-se efetivamente visível.

### Cenário 02 — Legibilidade

1. Utilize o slider nos mesmos níveis.
2. Observe títulos, texto digitado, botões e ícones.
3. Verifique se esses elementos permanecem opacos.
4. Repita o teste com fundos claros e escuros atrás da janela.
5. Registre eventuais problemas de contraste.

Não confunda opacidade do texto com legibilidade: um texto totalmente opaco também pode ficar difícil de ler sobre um fundo muito transparente.

### Cenário 03 — Redimensionamento

1. Ajuste a transparência para 50%.
2. Redimensione a janela.
3. Verifique se o fundo acompanha corretamente suas dimensões.
4. Observe eventuais artefatos, bordas opacas ou problemas de renderização.

### Cenário 04 — Interação

1. Ajuste a transparência para 70%.
2. Digite no editor.
3. Selecione texto.
4. Utilize os botões.
5. Verifique se a aplicação permanece responsiva e se os controles continuam utilizáveis.

### Cenário 05 — Aparência

1. Execute a aplicação em modo claro.
2. Repita os testes principais.
3. Execute em modo escuro.
4. Repita os testes.
5. Registre diferenças de comportamento, cores e contraste.

Se a POC-00 ainda não possuir alternância de tema, utilize a preferência do sistema, quando disponível. Não implemente uma janela completa de preferências apenas para este teste.

## 8. Critérios de avaliação

Classifique os resultados usando três estados:

- **Aprovado:** comportamento observado e comprovado no ambiente de referência.
- **Reprovado:** comportamento testado e incompatível com os critérios estabelecidos.
- **Pendente:** comportamento que não pôde ser verificado.

A hipótese H01 somente poderá ser aprovada se:

1. A transparência for ajustável durante a execução.
2. Outros aplicativos forem visíveis através do fundo.
3. Textos e controles permanecerem opacos.
4. A edição de texto continuar funcional.
5. A janela puder ser redimensionada sem falhas relevantes.
6. O comportamento funcionar no GNOME/Wayland.
7. As limitações de contraste e aparência estiverem documentadas.

Caso a transparência funcione apenas parcialmente, registre o resultado como parcialmente atendido na análise, sem classificar a hipótese inteira como aprovada.

Não declare sucesso somente porque o CSS foi aplicado ou a compilação foi concluída.

## 9. Documentação dos resultados

Atualize `docs/poc-plan.md` para refletir o andamento da POC-01.

Crie também:

`docs/poc-01-results.md`

Esse documento deverá conter:

### Identificação
- Nome da POC.
- Objetivo.
- Hipótese investigada.
- Ambiente e versões utilizadas.

### Implementação
- Abordagem adotada.
- APIs utilizadas.
- Estrutura de widgets envolvida.
- Explicação de como a transparência funciona.
- Alternativas investigadas, quando aplicável.

### Resultados
- Tabela com cada cenário de teste.
- Resultado esperado.
- Resultado observado.
- Estado: aprovado, reprovado ou pendente.
- Evidências disponíveis.

### Limitações
- Problemas de compatibilidade.
- Artefatos visuais.
- Problemas de contraste.
- Diferenças entre temas.
- Comportamentos que ainda exigem investigação.

### Conclusão
- Hipótese aprovada, reprovada ou pendente.
- Viabilidade da abordagem para o MVP.
- Riscos técnicos remanescentes.
- Recomendação sobre os próximos experimentos.

Se forem possíveis capturas de tela representativas, coloque-as em `docs/evidence/poc-01/`, evitando arquivos desnecessariamente grandes.

Não crie evidências fictícias.

## 10. README

Atualize o README.md somente no necessário para:

- Informar que a POC-01 foi implementada.
- Descrever o controle experimental de transparência.
- Apresentar instruções para executar e testar.
- Referenciar `docs/poc-01-results.md`.
- Identificar claramente o que já foi validado e o que permanece pendente.

Preserve as informações relevantes da POC-00.

## 11. Verificações finais

Execute, quando possível:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

Corrija os problemas identificados na implementação.

Não declare `cargo run` como validação gráfica se o ambiente não permitir observar a janela.

Registre os comandos executados e seus resultados.

## 12. Fora do escopo

Não implemente:

- Sobreposição automática.
- Extensões GNOME Shell.
- Gerenciamento de múltiplas notas.
- Persistência do percentual de transparência.
- GSettings.
- Janela definitiva de preferências.
- Gerenciamento de temas personalizáveis.
- Armazenamento Markdown.
- Exportação de arquivos.
- Flatpak.
- Alterações arquiteturais extensas.
- Funcionalidades do MVP não relacionadas ao experimento.

Não inicie a POC-02 nesta execução.

## 13. Critérios de conclusão da tarefa

Considere a implementação da POC-01 concluída quando:

1. O controle de transparência estiver implementado.
2. O código compilar sem erros.
3. Os testes unitários aplicáveis forem aprovados.
4. As alterações não quebrarem as funcionalidades da POC-00.
5. O documento de resultados estiver atualizado.
6. Os testes visuais executados possuírem evidências ou registros objetivos.
7. Testes não executados estiverem claramente identificados como pendentes.
8. A conclusão distinguir implementação de validação da hipótese.

A tarefa de implementação pode ser considerada concluída mesmo que a hipótese técnica seja reprovada, desde que a investigação produza evidências suficientes e explique a inviabilidade encontrada.

## 14. Entrega

Ao finalizar, apresente:

1. Resumo da implementação.
2. Arquivos criados e modificados.
3. Abordagem técnica utilizada.
4. Comandos executados e resultados.
5. Resultado de cada teste.
6. Hipótese H01: aprovada, reprovada ou pendente.
7. Limitações encontradas.
8. Recomendação para a POC-02 — Sobreposição manual.

**Não crie commits, não publique alterações e não modifique o repositório principal `oak-note` sem autorização.**

Priorize código mínimo, experimentação controlada, verificabilidade e documentação técnica objetiva.
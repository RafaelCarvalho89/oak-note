# Oak Note POC — 03: Aparência e comportamento da interface

## Contexto

Estamos desenvolvendo o **Oak Note**, um aplicativo desktop Open Source para Linux, utilizando Rust + GTK4 + libadwaita.

O repositório atual, `oak-note-poc`, é um laboratório técnico independente do produto principal, `oak-note`.

O MVP utilizará uma interface minimalista, com editor Markdown, temas Sistema/Claro/Escuro e transparência ajustável do fundo.

**Ambiente de referência:** Debian + GNOME (Wayland).

### Histórico das POCs

- **POC-00:** Bootstrap da aplicação Rust + GTK4/libadwaita.
- **POC-01:** Transparência seletiva do fundo.
- **POC-02:** Sobreposição manual no GNOME/Wayland.
- **POC-03:** Aparência e comportamento da interface — etapa atual.
- **POC-04:** Viabilidade de distribuição Flatpak — etapa futura.

As POCs são investigações técnicas, não entregas de funcionalidades definitivas.

Não presuma que as etapas anteriores foram aprovadas. Consulte suas evidências e conclusões.

## 1. Objetivo

Investigar e validar o comportamento visual e funcional da interface Oak Note nos modos claro e escuro, considerando transparência seletiva, redimensionamento e usabilidade.

A POC deve responder:

1. É possível alternar dinamicamente entre Sistema, Claro e Escuro utilizando libadwaita?
2. O modo Sistema acompanha corretamente a preferência do GNOME?
3. A transparência seletiva da POC-01 continua funcionando nos três modos?
4. Os backgrounds personalizados se adaptam corretamente ao tema?
5. Textos, ícones, botões e seleções permanecem legíveis?
6. A janela funciona adequadamente em dimensões compactas?
7. A interface permanece utilizável por teclado?
8. Quais limitações devem ser consideradas na implementação do MVP?

**Não implemente o layout definitivo do Oak Note.**

A POC deve utilizar a interface experimental existente.

## 2. Inspeção inicial obrigatória

Antes de modificar arquivos:

1. Inspecione a estrutura atual do repositório.
2. Leia README.md e `docs/poc-plan.md`.
3. Leia `docs/poc-01-results.md`, se existir.
4. Leia `docs/poc-02-results.md`, se existir.
5. Identifique os resultados efetivamente validados.
6. Analise o código da janela e dos controles existentes.
7. Identifique os estilos CSS utilizados na transparência.
8. Verifique as versões de GTK4, libadwaita e suas respectivas crates Rust.
9. Verifique se já existe alguma implementação de alternância de temas.
10. Execute `cargo check` para estabelecer o estado inicial, quando possível.

Preserve os recursos implementados nas etapas anteriores.

Não reinicialize Cargo e não altere a arquitetura sem necessidade.

Caso algum experimento anterior esteja pendente ou reprovado, registre essa condição e não apresente seus comportamentos como validados.

## 3. Hipóteses técnicas

### H03-A — Alternância de temas

É possível alternar dinamicamente entre Sistema, Claro e Escuro utilizando os mecanismos públicos do libadwaita, sem reiniciar a aplicação.

### H03-B — Compatibilidade com transparência

O mecanismo de transparência seletiva pode funcionar nos modos claro e escuro sem tornar textos e controles translúcidos.

### H03-C — Redimensionamento

A interface experimental permanece funcional e visualmente consistente durante o redimensionamento, incluindo tamanhos compactos.

### H03-D — Usabilidade

A interface mantém legibilidade e navegação básica por teclado, mesmo com diferentes temas e níveis de transparência.

As hipóteses devem ser avaliadas separadamente.

O resultado negativo de uma hipótese não invalida automaticamente as demais.

## 4. Experimento E01 — Alternância de temas

### 4.1 Implementação

Adicione à interface experimental um seletor com três opções:

- Sistema
- Claro
- Escuro

Utilize preferencialmente `AdwStyleManager`.

Considere o mapeamento recomendado pelo libadwaita:

- Sistema: `PREFER_LIGHT`, seguindo a preferência escura do sistema quando disponível.
- Claro: `FORCE_LIGHT`.
- Escuro: `FORCE_DARK`.

Utilize os nomes e APIs efetivamente disponíveis na versão de libadwaita-rs instalada.

Não utilize `GtkSettings:gtk-application-prefer-dark-theme` como mecanismo principal de controle de aparência.

A alternância deverá ocorrer imediatamente, sem fechar ou reiniciar a aplicação.

Não implemente persistência da preferência nesta POC.

### 4.2 Diagnóstico

Se a API disponível permitir, consulte o estado efetivo da aparência por meio de `AdwStyleManager`.

Diferencie:

- Preferência selecionada pelo usuário.
- Preferência informada pelo sistema.
- Aparência efetivamente aplicada.

Caso o sistema não disponibilize sua preferência de esquema de cores, registre essa limitação.

Não presuma que o modo Sistema significa sempre Claro ou sempre Escuro.

### 4.3 Validação

Execute os seguintes cenários:

**Cenário A — Modo Claro**

1. Selecione Claro.
2. Verifique a aparência da janela.
3. Inspecione editor, header bar, botões e controles.
4. Confirme que os elementos utilizam aparência clara.

**Cenário B — Modo Escuro**

1. Selecione Escuro.
2. Verifique a aparência da janela.
3. Inspecione os mesmos componentes.
4. Confirme que a interface utiliza aparência escura.

**Cenário C — Modo Sistema**

1. Selecione Sistema.
2. Verifique a preferência atual do GNOME.
3. Quando possível, alterne o tema do GNOME entre claro e escuro.
4. Confirme se o Oak Note acompanha a mudança.
5. Registre comportamentos inconsistentes.

**Critério de aprovação:** os três modos devem apresentar o comportamento esperado, sem necessidade de reinicialização e sem erros relevantes de renderização.

## 5. Experimento E02 — Temas e transparência seletiva

Este experimento depende da implementação realizada na POC-01.

Se a POC-01 estiver incompleta, reprovada ou pendente, documente a dependência.

Não reimplemente toda a POC-01 nesta tarefa.

### 5.1 Investigação

Avalie a interação entre:

- Estilos padrão do libadwaita.
- `GtkCssProvider`.
- Classes CSS específicas da aplicação.
- Backgrounds RGBA.
- Background do `GtkTextView`.
- Background do `GtkScrolledWindow`.
- Header bar e containers.
- Cores de texto e controles.

Evite sobrescrever globalmente os estilos do libadwaita.

Priorize mecanismos de cores e estilos compatíveis com a versão instalada.

Não utilize cores fixas que funcionem somente em um dos temas sem documentar a limitação.

### 5.2 Matriz de testes

Realize os testes com os seguintes níveis de transparência:

| Tema | 0% | 25% | 50% | 70% |
|---|---|---|---|---|
| Claro | Testar | Testar | Testar | Testar |
| Escuro | Testar | Testar | Testar | Testar |

Teste o modo Sistema pelo menos nos estados efetivos claro e escuro, quando o ambiente permitir.

Para cada combinação, observe:

1. Transparência real do fundo.
2. Opacidade do texto e dos controles.
3. Contraste do conteúdo.
4. Visibilidade de janelas posicionadas atrás.
5. Alterações inesperadas de cor.
6. Artefatos gráficos e bordas opacas.

### 5.3 Critérios de avaliação

- A transparência deve modificar somente os backgrounds previstos.
- Textos e controles devem permanecer opacos.
- A troca de tema não deve alterar o valor de transparência selecionado.
- O usuário deve conseguir continuar utilizando o editor.
- Deficiências de legibilidade devem ser identificadas e documentadas.

Não considere a hipótese aprovada apenas porque os widgets continuam visíveis.

Se a legibilidade ficar inadequada em determinados níveis, registre quais combinações apresentam o problema.

Não introduza automaticamente limites de transparência diferentes por tema. Isso exigirá uma decisão posterior.

## 6. Experimento E03 — Redimensionamento da janela

### 6.1 Objetivo

Verificar se a interface permanece utilizável quando a janela é reduzida ou ampliada.

### 6.2 Dimensões de referência

Teste inicialmente:

| Cenário | Dimensões aproximadas |
|---|---|
| Janela padrão | 800 × 560 |
| Janela intermediária | 500 × 400 |
| Janela compacta | 360 × 280 |
| Janela maximizada | Conforme resolução disponível |

As dimensões são parâmetros experimentais, não requisitos definitivos do MVP.

Se o GTK4 ou algum widget impedir a redução até determinado tamanho, registre o limite efetivamente observado.

Não force dimensões inferiores às necessárias para funcionamento seguro dos widgets apenas para cumprir a tabela.

### 6.3 Procedimento

Para cada dimensão:

1. Verifique se o editor permanece visível.
2. Verifique se os controles permanecem acessíveis.
3. Observe o comportamento do texto.
4. Confirme o funcionamento da rolagem.
5. Inspecione cortes, sobreposições e desalinhamentos.
6. Alterne entre Claro e Escuro.
7. Verifique se a transparência continua funcionando.

Observe também o redimensionamento contínuo com o mouse.

### 6.4 Critérios de aprovação

A interface deve:

- Permanecer funcional.
- Permitir edição.
- Não apresentar controles essenciais inacessíveis.
- Evitar sobreposições indevidas.
- Preservar a rolagem do conteúdo.
- Não apresentar falhas de renderização relevantes.

Caso sejam necessárias mudanças mínimas no layout experimental para realizar os testes, implemente apenas o necessário.

Não desenvolva a barra lateral definitiva nem o modo compacto do MVP.

## 7. Experimento E04 — Legibilidade e acessibilidade básica

### 7.1 Legibilidade

Utilize conteúdo de teste contendo:

- Títulos.
- Parágrafos longos.
- Listas Markdown.
- Texto selecionado.
- Trechos com caracteres especiais.
- Texto suficiente para ativar rolagem.

Verifique:

1. Contraste em claro e escuro.
2. Legibilidade com transparência elevada.
3. Cor e visibilidade da seleção.
4. Cursor de edição.
5. Estados de foco.
6. Legibilidade de rótulos e ícones.

Quando possível, realize testes com um aplicativo de fundo claro e outro de fundo escuro.

Não presuma que texto totalmente opaco garante contraste adequado.

### 7.2 Navegação por teclado

Valide:

- `Tab` e `Shift+Tab` para navegar entre controles.
- Acesso ao editor pelo teclado.
- Alternância entre opções de tema.
- Ajuste do slider por teclado.
- Identificação visual do foco.
- Ausência de bloqueios inesperados de navegação.

### 7.3 Tecnologias assistivas

Caso o Orca esteja disponível, realize uma verificação manual básica dos nomes e estados dos controles.

Não instale ou configure ferramentas assistivas globalmente sem autorização.

Caso o teste não possa ser executado, registre-o como pendente.

Não declare conformidade integral com padrões de acessibilidade apenas com esses testes.

## 8. Implementação mínima

A interface final desta POC deverá conter somente os elementos necessários para os experimentos:

- Janela GTK4/libadwaita existente.
- Editor de texto demonstrativo.
- Slider de transparência da POC-01.
- Seletor experimental Sistema/Claro/Escuro.
- Rótulos ou indicadores úteis para diagnóstico, quando necessários.

### Requisitos de implementação

- Utilize APIs públicas do GTK4/libadwaita.
- Preserve o comportamento existente.
- Evite código duplicado.
- Mantenha a lógica de seleção de tema simples.
- Não implemente uma janela definitiva de preferências.
- Não introduza GSettings.
- Não desenvolva temas personalizados.
- Não implemente persistência de configurações.
- Não acrescente abstrações arquiteturais desnecessárias.

A POC deve continuar pequena, experimental e potencialmente descartável.

## 9. Testes automatizados

Preserve os testes existentes das POCs anteriores.

Adicione testes unitários somente se houver lógica independente da interface que justifique essa validação.

Exemplos:

- Mapeamento das opções de tema para os valores de esquema de cores.
- Transformações puras de configurações visuais.
- Validações de parâmetros, quando aplicáveis.

Não crie abstrações artificiais apenas para testar chamadas simples de widgets GTK4.

Não implemente testes automatizados de interface nesta etapa.

## 10. Verificações técnicas

Execute, quando o ambiente permitir:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

Registre o resultado de cada comando.

Caso não exista sessão gráfica acessível, não classifique os experimentos visuais como aprovados.

Evite modificar dependências do sistema sem autorização.

## 11. Documentação dos resultados

Crie:

`docs/poc-03-results.md`

O documento deverá conter:

### 1. Identificação
- Nome e objetivo da POC.
- Ambiente utilizado.
- Versões de Rust, GTK4, libadwaita e GNOME.
- Hipóteses H03-A, H03-B, H03-C e H03-D.

### 2. Implementação
- Abordagem adotada.
- APIs utilizadas.
- Alterações realizadas na interface.
- Estratégia de alternância de temas.
- Tratamento dos estilos CSS e transparência.

### 3. Resultados experimentais
- Resultados do E01.
- Matriz de testes do E02.
- Resultados do E03.
- Resultados do E04.

Para cada teste, documente:

- Resultado esperado.
- Resultado observado.
- Estado: aprovado, reprovado ou pendente.
- Evidências disponíveis.

### 4. Limitações
- Incompatibilidades encontradas.
- Problemas de contraste.
- Diferenças entre claro e escuro.
- Problemas com tamanhos compactos.
- Comportamentos dependentes do ambiente.
- Limitações de acessibilidade.

### 5. Conclusões
- Resultado individual de cada hipótese.
- Viabilidade das soluções para o MVP.
- Riscos remanescentes.
- Recomendações técnicas.

### 6. Evidências

Quando possível, armazene capturas representativas em:

`docs/evidence/poc-03/`

Utilize nomes descritivos que identifiquem tema, transparência e dimensão.

Evite capturas redundantes ou muito grandes.

Não gere evidências fictícias.

## 12. Atualização do plano e README

Atualize `docs/poc-plan.md` com o estado real da POC-03.

Atualize README.md apenas no necessário para:

- Apresentar os novos controles experimentais.
- Explicar como testar temas e redimensionamento.
- Referenciar `docs/poc-03-results.md`.
- Informar limitações identificadas.

Preserve a documentação das POCs anteriores.

Não reescreva todo o README sem necessidade.

## 13. Critérios de aprovação

Avalie cada hipótese separadamente.

**H03-A — Temas**

Aprovada quando Sistema, Claro e Escuro funcionarem corretamente e a alternância ocorrer sem reiniciar a aplicação.

**H03-B — Transparência**

Aprovada quando a transparência seletiva permanecer funcional nos temas claro e escuro, preservando a opacidade do conteúdo e sem defeitos relevantes de composição.

**H03-C — Redimensionamento**

Aprovada quando a interface permanecer funcional nas dimensões efetivamente suportadas, inclusive no cenário compacto requerido, sem tornar controles essenciais inacessíveis.

**H03-D — Usabilidade**

Aprovada quando a navegação por teclado, visibilidade do foco e legibilidade nos cenários obrigatórios forem verificadas sem falhas impeditivas.

Classifique cada hipótese como:

- **Aprovada:** todos os critérios obrigatórios comprovados.
- **Reprovada:** comportamento testado e incompatível com um ou mais critérios obrigatórios.
- **Pendente:** evidências insuficientes.

Identifique separadamente resultados parcialmente atendidos.

A implementação da POC pode ser considerada concluída mesmo que alguma hipótese seja reprovada, desde que a investigação tenha sido documentada adequadamente.

## 14. Fora do escopo

Não implementar nesta tarefa:

- Layout definitivo do Oak Note.
- Barra lateral de notas.
- Gerenciamento de arquivos Markdown.
- Salvamento automático.
- GSettings.
- Preferências persistentes.
- Temas personalizados.
- Internacionalização.
- Extensões GNOME Shell.
- Controle automático de sobreposição.
- Empacotamento Flatpak.
- Refatorações arquiteturais extensas.
- Funcionalidades do MVP não relacionadas aos experimentos.

Não inicie a POC-04.

## 15. Entrega

Ao finalizar, apresente:

1. Resumo das alterações.
2. Arquivos criados ou modificados.
3. Abordagem de temas utilizada.
4. Resultado dos experimentos E01–E04.
5. Estado das hipóteses H03-A, H03-B, H03-C e H03-D.
6. Comandos executados e respectivos resultados.
7. Limitações encontradas.
8. Impactos das conclusões no MVP do Oak Note.
9. Recomendação para a POC-04 — Viabilidade Flatpak.

Se a validação gráfica não puder ser realizada no ambiente do Codex, entregue as instruções exatas para execução manual e deixe os resultados pendentes.

**Não crie commits, não envie alterações ao GitHub e não modifique o repositório principal `oak-note` sem autorização.**

Priorize experimentação controlada, código mínimo, evidências verificáveis e documentação objetiva.
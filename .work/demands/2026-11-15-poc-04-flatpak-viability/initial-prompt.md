# Oak Note POC — 04: Viabilidade Flatpak

## Contexto

Estamos desenvolvendo o **Oak Note**, uma aplicação desktop Open Source para Linux Debian, utilizando Rust + GTK4 + libadwaita.

O repositório atual, `oak-note-poc`, é um laboratório técnico independente do repositório principal `oak-note`.

O objetivo da POC é reduzir incertezas técnicas antes do início do MVP.

**Ambiente de referência:** Debian + GNOME (Wayland).

### Histórico das POCs

- **POC-00:** Bootstrap da aplicação Rust + GTK4/libadwaita.
- **POC-01:** Transparência seletiva do fundo.
- **POC-02:** Sobreposição manual no GNOME/Wayland.
- **POC-03:** Temas, aparência e comportamento da interface.
- **POC-04:** Viabilidade de distribuição Flatpak — etapa atual.

Não presuma que as POCs anteriores foram aprovadas. Consulte seus relatórios e evidências.

## 1. Objetivo

Investigar, implementar e validar o empacotamento Flatpak da aplicação experimental Oak Note POC.

A POC deverá responder:

1. A aplicação Rust compila utilizando o SDK Flatpak?
2. É possível fornecer as dependências Cargo sem acesso à rede durante a compilação?
3. O aplicativo pode ser instalado e executado em sandbox?
4. A interface GTK4/libadwaita mantém o comportamento observado na execução nativa?
5. A transparência seletiva continua funcional?
6. A sobreposição manual permanece disponível?
7. Os modos claro e escuro funcionam corretamente?
8. A aplicação consegue operar com permissões mínimas?
9. É possível distribuir e instalar um arquivo `.flatpak` gerado localmente?
10. Quais limitações precisam ser consideradas no MVP?

**Não transforme a POC em um aplicativo definitivo.**

O objetivo é validar viabilidade técnica, empacotamento e compatibilidade.

## 2. Inspeção inicial obrigatória

Antes de modificar arquivos:

1. Inspecione o repositório atual.
2. Leia README.md.
3. Leia `docs/poc-plan.md`.
4. Leia os relatórios das POCs 01, 02 e 03, quando disponíveis.
5. Identifique funcionalidades implementadas e efetivamente validadas.
6. Inspecione Cargo.toml e Cargo.lock.
7. Identifique o Application ID utilizado.
8. Identifique as versões de GTK4 e libadwaita necessárias.
9. Verifique se Flatpak e flatpak-builder estão disponíveis.
10. Inspecione alterações locais existentes antes de modificar o projeto.

Execute as verificações iniciais quando possível:

```bash
cargo check
cargo test
flatpak --version
flatpak-builder --version
flatpak remotes
flatpak list --runtime
```

Não reinicialize Cargo.

Não sobrescreva alterações existentes sem necessidade.

Não instale pacotes de sistema, adicione remotes ou execute comandos privilegiados sem autorização.

Se alguma dependência estiver ausente, documente o requisito e solicite a instalação ao usuário quando necessário.

## 3. Hipóteses técnicas

### H04-A — Compilação

A aplicação Rust + GTK4/libadwaita pode ser compilada utilizando o SDK Flatpak com dependências Cargo preparadas para build offline.

### H04-B — Execução

O aplicativo pode ser instalado e executado em uma sessão Debian + GNOME (Wayland) utilizando o runtime GNOME.

### H04-C — Compatibilidade

As funcionalidades experimentais anteriormente validadas permanecem funcionais quando executadas dentro do sandbox Flatpak.

### H04-D — Isolamento

A aplicação consegue executar suas funcionalidades atuais sem permissões amplas sobre o filesystem, rede ou serviços do sistema.

### H04-E — Distribuição

É possível gerar um bundle `.flatpak`, instalá-lo localmente e executar a aplicação a partir desse pacote.

Avalie as hipóteses individualmente.

Não considere que um build concluído comprova automaticamente o funcionamento gráfico ou as demais hipóteses.

## 4. Experimento E01 — Ambiente de build Flatpak

### 4.1 Seleção de runtime e SDK

Investigue e selecione:

- `org.gnome.Platform`
- `org.gnome.Sdk`
- `org.freedesktop.Sdk.Extension.rust-stable`

A versão escolhida deve:

- Estar disponível em uma fonte oficial confiável.
- Possuir suporte ativo.
- Incluir GTK4/libadwaita compatíveis com o projeto.
- Ser compatível com as dependências Rust existentes.
- Permitir a compilação sem mudanças desnecessárias no código.

Não escolha automaticamente a versão mais recente sem verificar compatibilidade.

Documente a versão efetivamente selecionada e o motivo.

### 4.2 Application ID

Utilize o Application ID já definido nas POCs anteriores, caso seja válido e adequado.

O identificador inicialmente proposto foi:

`io.github.rafaelcarvalho89.OakNotePoc`

Verifique sua conformidade com as convenções Flatpak.

Garanta consistência entre:

- Identificador utilizado no código GTK.
- Manifesto Flatpak.
- Arquivo desktop.
- Metadados da aplicação.
- Ícone exportado.

Se for necessário corrigir o identificador, documente o motivo e o impacto da alteração.

Não utilize o mesmo ID da aplicação definitiva `oak-note`.

### 4.3 Dependências Cargo

Investigue a utilização do projeto oficial `flatpak-cargo-generator`, mantido em:

https://github.com/flatpak/flatpak-builder-tools

Prepare as dependências do Cargo.lock para utilização no ambiente Flatpak.

Requisitos:

1. Utilize o Cargo.lock existente como referência.
2. Gere as fontes com suas verificações de integridade.
3. Integre o resultado ao manifesto.
4. Configure Cargo para execução offline durante a compilação.
5. Evite downloads de crates durante o comando de build no sandbox.
6. Documente o procedimento de atualização quando Cargo.lock mudar.

É permitido realizar o download das dependências durante a etapa preparatória do flatpak-builder.

Não configure acesso irrestrito à rede apenas para permitir que Cargo baixe dependências durante a compilação.

Utilize a extensão Rust estável do SDK quando compatível.

Não utilize a toolchain Rust instalada no host como substituta silenciosa do compilador do SDK.

### 4.4 Critérios de aprovação

- Dependências corretamente resolvidas.
- Compilação realizada utilizando a toolchain do ambiente Flatpak.
- Cargo funcionando em modo offline durante o build.
- Binário gerado para instalação em `/app/bin`.
- Ausência de erros impeditivos de compilação.

## 5. Estrutura de arquivos

Organize os recursos Flatpak em um diretório específico, preferencialmente:

```text
oak-note-poc/
├── src/
├── docs/
│   ├── poc-plan.md
│   ├── poc-01-results.md
│   ├── poc-02-results.md
│   ├── poc-03-results.md
│   └── poc-04-results.md
├── flatpak/
│   ├── io.github.rafaelcarvalho89.OakNotePoc.yml
│   ├── cargo-sources.json
│   ├── io.github.rafaelcarvalho89.OakNotePoc.desktop
│   ├── io.github.rafaelcarvalho89.OakNotePoc.metainfo.xml
│   └── io.github.rafaelcarvalho89.OakNotePoc.svg
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

Essa estrutura é uma sugestão. Preserve as convenções existentes quando fizer sentido.

O manifesto pode utilizar YAML ou JSON.

O ícone pode ser provisório e minimalista, desde que seja válido e identificado como recurso experimental. Reutilize um recurso já existente, se houver.

Não desenvolva identidade visual definitiva nesta etapa.

## 6. Experimento E02 — Manifesto e instalação

### 6.1 Manifesto Flatpak

Crie um manifesto funcional contendo:

- Application ID.
- Runtime GNOME.
- Runtime version.
- SDK GNOME.
- Extensão Rust, quando necessária.
- Comando de execução.
- Módulo de compilação Rust.
- Fontes Cargo preparadas.
- Permissões mínimas.

Evite configurações sem justificativa.

O manifesto deve instalar corretamente:

- Binário da aplicação.
- Arquivo `.desktop`.
- Ícone.
- Metadados mínimos AppStream.

Utilize caminhos adequados sob `/app`.

Não copie o binário compilado no host como substituto do build dentro do SDK.

### 6.2 Permissões

Comece com as permissões mínimas necessárias para uma aplicação GTK4/libadwaita.

Investigue:

- `--socket=wayland`
- `--socket=fallback-x11`, se houver justificativa para compatibilidade
- `--device=dri`, quando necessário para renderização

Adicione permissões complementares somente quando um teste demonstrar necessidade.

Não conceda inicialmente:

- `--filesystem=host`
- `--filesystem=home`
- `--share=network`
- `--socket=session-bus`
- `--socket=system-bus`
- `--device=all`

Não exponha diretórios externos apenas para simplificar testes.

Caso alguma permissão adicional seja necessária, registre:

1. Funcionalidade dependente.
2. Problema observado sem a permissão.
3. Permissão escolhida.
4. Justificativa técnica.
5. Alternativas menos permissivas consideradas.

### 6.3 Build e instalação

Utilize o `flatpak-builder`.

Documente os comandos exatos para:

1. Preparar dependências.
2. Compilar a aplicação.
3. Exportar o resultado para um repositório local.
4. Instalar a aplicação no escopo do usuário.
5. Executar a aplicação instalada.

Prefira instalação com `--user`.

Não exija privilégios administrativos para instalar a aplicação construída.

Se a instalação de runtimes exigir downloads ou configuração adicional, identifique claramente essa dependência.

### 6.4 Validação

Confirme:

- Build bem-sucedido.
- Instalação local bem-sucedida.
- Application ID correto.
- Arquivo desktop reconhecido.
- Ícone disponível.
- Aplicação iniciada via `flatpak run`.
- Encerramento convencional.

Se não houver sessão gráfica, registre a validação da janela como pendente.

## 7. Experimento E03 — Isolamento e permissões

### 7.1 Inspeção do sandbox

Utilize ferramentas Flatpak para identificar as permissões efetivamente concedidas.

Exemplo:

```bash
flatpak info --show-permissions io.github.rafaelcarvalho89.OakNotePoc
```

Adapte o ID caso o projeto utilize outro identificador válido.

Documente:

- Permissões de filesystem.
- Sockets disponíveis.
- Acesso à rede.
- Dispositivos expostos.
- Permissões D-Bus relevantes.

Não presuma que o manifesto representa todas as permissões efetivas: verifique também eventuais overrides locais.

### 7.2 Diretórios XDG

Investigue como o Flatpak disponibiliza:

- `XDG_DATA_HOME`
- `XDG_CONFIG_HOME`
- `XDG_CACHE_HOME`

Verifique seus valores dentro do sandbox, quando possível.

O objetivo é confirmar que a futura aplicação poderá utilizar diretórios privados sem acesso amplo à home do usuário.

Não implemente armazenamento definitivo de notas nesta POC.

Se necessário, utilize um pequeno arquivo temporário ou uma operação de diagnóstico isolada para comprovar a disponibilidade de um diretório privado.

Qualquer arquivo criado para diagnóstico deve ser claramente identificado e não deve afetar arquivos pessoais existentes.

Não crie uma camada de persistência nesta etapa.

### 7.3 Acesso a arquivos externos

Investigue conceitualmente a utilização de XDG Desktop Portals para a futura exportação das notas.

Documente:

- Como a aplicação poderá solicitar a escolha de um destino.
- Por que o portal é preferível a conceder acesso amplo ao filesystem.
- Quais APIs GTK/GIO podem ser utilizadas futuramente.
- Quais verificações deverão ser realizadas no MVP.

Não implemente a funcionalidade de exportação nesta POC.

### 7.4 Critérios de aprovação

- A aplicação executa com permissões mínimas.
- Não necessita de acesso irrestrito ao filesystem.
- Não necessita de rede para suas funcionalidades atuais.
- O sandbox disponibiliza diretórios privados adequados.
- As permissões efetivas foram verificadas e documentadas.

## 8. Experimento E04 — Compatibilidade funcional

Execute a versão Flatpak no GNOME/Wayland.

Compare seu comportamento com a versão nativa das POCs anteriores.

### 8.1 Interface básica

Verifique:

- Abertura da janela.
- Renderização GTK4/libadwaita.
- Campo de texto editável.
- Redimensionamento.
- Fechamento.
- Ausência de falhas gráficas relevantes.

### 8.2 Transparência seletiva

Se a POC-01 tiver resultado positivo, repita:

| Nível | Validação |
|---|---|
| 0% | Fundo opaco |
| 25% | Transparência leve |
| 50% | Transparência intermediária |
| 70% | Transparência elevada |

Confirme:

- Transparência real do fundo.
- Texto e controles opacos.
- Possibilidade de visualizar janelas atrás.
- Ausência de problemas adicionais causados pelo Flatpak.

Se a POC-01 não estiver aprovada, registre a dependência.

Não tente reimplementar a transparência nesta etapa.

### 8.3 Sobreposição manual

Se a POC-02 tiver resultado positivo:

1. Execute o Oak Note pelo Flatpak.
2. Abra outro aplicativo.
3. Ative manualmente "Sempre no topo" pelo GNOME.
4. Alterne o foco para o outro aplicativo.
5. Confirme se o Oak Note permanece visível.
6. Desative a sobreposição.

Compare o resultado com a execução nativa.

Não implemente sobreposição programática.

### 8.4 Temas

Se a POC-03 tiver implementado o seletor de aparência:

1. Teste Sistema.
2. Teste Claro.
3. Teste Escuro.
4. Verifique a integração com a preferência do GNOME.
5. Observe eventuais diferenças causadas pelo sandbox.

### 8.5 Redimensionamento

Repita os testes com:

- Janela padrão.
- Janela intermediária.
- Janela compacta.
- Janela maximizada.

Registre os resultados e as dimensões efetivamente suportadas.

### 8.6 Comparação

Crie uma matriz:

| Funcionalidade | Nativo | Flatpak | Diferença |
|---|---|---|---|
| Inicialização | ... | ... | ... |
| Editor | ... | ... | ... |
| Transparência | ... | ... | ... |
| Sobreposição | ... | ... | ... |
| Temas | ... | ... | ... |
| Redimensionamento | ... | ... | ... |

Utilize apenas resultados observados.

Não classifique comportamentos como aprovados por inferência.

## 9. Experimento E05 — Distribuição por bundle

### 9.1 Objetivo

Gerar um arquivo `.flatpak` capaz de instalar a aplicação a partir de um artefato local.

Utilize o repositório Flatpak produzido no experimento E02.

Investigue e execute, quando possível, o comando oficial `flatpak build-bundle`.

O artefato esperado poderá receber o nome:

`oak-note-poc.flatpak`

Armazene-o em um diretório de artefatos de build, como `dist/`, ignorado pelo Git.

### 9.2 Validação

Verifique:

1. O arquivo `.flatpak` foi gerado.
2. O artefato possui formato válido.
3. Pode ser instalado com `flatpak install --user`.
4. A instalação identifica o Application ID correto.
5. A aplicação pode ser iniciada após a instalação.
6. O pacote pode ser removido pelo Flatpak.

Quando viável, realize o teste de reinstalação em ambiente limpo ou isolado, sem destruir dados existentes.

Não remova automaticamente outras instalações ou aplicativos do usuário.

### 9.3 Dependências

Documente explicitamente que um bundle Flatpak não inclui necessariamente o runtime GNOME.

A instalação em outra máquina poderá exigir a obtenção do runtime por um repositório configurado.

Diferencie:

- Arquivo de instalação gerado.
- Runtime necessário.
- Repositório de distribuição.
- Procedimento de atualização.

Não declare que o bundle é completamente autossuficiente ou independente de rede para uma primeira instalação sem comprovação.

### 9.4 Fora do escopo do experimento

Não publique:

- No Flathub.
- No GitHub Releases.
- Em repositórios Flatpak públicos.

A finalidade é validar a distribuição local.

## 10. Organização dos artefatos

Adicione ao .gitignore os diretórios gerados pelo processo de build, conforme necessário.

Exemplos:

```gitignore
/build-dir/
/.flatpak-builder/
/repo/
/dist/
```

Adapte os caminhos à estrutura realmente utilizada.

Não ignore o manifesto Flatpak ou os arquivos-fonte necessários para reproduzir a compilação.

Versione o arquivo gerado de fontes Cargo, quando necessário para o processo de build reproduzível.

Não versione caches, binários ou diretórios temporários.

## 11. Testes automatizados e verificações

Preserve os testes Rust existentes.

Execute, quando possível:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Execute também as verificações específicas do Flatpak:

- Validação do manifesto.
- Compilação usando flatpak-builder.
- Verificação dos metadados desktop/AppStream quando houver ferramentas disponíveis.
- Instalação local.
- Execução com flatpak run.
- Inspeção das permissões.
- Geração do bundle.

Registre o comando exato e o resultado de cada etapa.

Se um teste exigir interação gráfica não disponível ao Codex, registre-o como pendente e forneça instruções para execução manual.

Não crie testes artificiais apenas para aumentar a cobertura.

## 12. Documentação dos resultados

Crie:

`docs/poc-04-results.md`

O documento deverá conter:

### 1. Identificação
- Nome da POC.
- Objetivo.
- Hipóteses H04-A a H04-E.
- Ambiente utilizado.

### 2. Configuração de build
- Versões do runtime e SDK.
- Versão da extensão Rust.
- Dependências utilizadas.
- Ferramentas de build.
- Application ID.

### 3. Manifesto
- Estrutura adotada.
- Comando de execução.
- Permissões configuradas.
- Justificativa das permissões.
- Recursos instalados.

### 4. Resultados dos experimentos
- E01: compilação.
- E02: empacotamento e instalação.
- E03: sandbox.
- E04: compatibilidade.
- E05: distribuição por bundle.

Para cada cenário, registre:

- Resultado esperado.
- Resultado observado.
- Evidências.
- Estado: aprovado, reprovado ou pendente.

### 5. Comparação com execução nativa
- Comportamentos equivalentes.
- Diferenças identificadas.
- Incompatibilidades.
- Impactos para o MVP.

### 6. Limitações
- Dependências de runtime.
- Problemas de compilação.
- Permissões adicionais.
- Diferenças gráficas.
- Restrições de sandbox.
- Questões de distribuição.

### 7. Conclusões
- Estado de cada hipótese.
- Viabilidade do Flatpak como formato inicial.
- Riscos remanescentes.
- Recomendações para o MVP.

### 8. Evidências

Quando possível, armazene evidências em:

`docs/evidence/poc-04/`

Evite capturas redundantes e arquivos muito grandes.

Não crie evidências fictícias.

## 13. Atualização da documentação

Atualize `docs/poc-plan.md` com o estado efetivo da POC-04.

Atualize README.md com:

- Pré-requisitos Flatpak.
- Instruções para preparação das dependências.
- Comandos de build.
- Instalação local.
- Execução.
- Geração do bundle.
- Limitações conhecidas.
- Referência aos resultados da POC-04.

Preserve toda a documentação relevante das POCs anteriores.

Não reescreva o README indiscriminadamente.

## 14. Critérios de aprovação

### H04-A — Compilação

Aprovada quando o aplicativo compilar no SDK Flatpak utilizando as dependências Cargo preparadas para build offline.

### H04-B — Execução

Aprovada quando o aplicativo instalado pelo Flatpak abrir e funcionar em sessão Debian + GNOME (Wayland).

### H04-C — Compatibilidade

Aprovada quando as funcionalidades das POCs anteriores efetivamente validadas apresentarem comportamento equivalente no Flatpak, sem regressões impeditivas.

Funcionalidades anteriores reprovadas ou pendentes devem permanecer explicitamente identificadas como dependências.

### H04-D — Isolamento

Aprovada quando o aplicativo funcionar com permissões mínimas justificadas, sem acesso irrestrito ao filesystem ou à rede.

### H04-E — Distribuição

Aprovada quando o bundle `.flatpak` puder ser gerado, instalado e executado em ambiente compatível.

Classifique cada hipótese como:

- **Aprovada:** critérios obrigatórios comprovados.
- **Reprovada:** critérios testados e não atendidos.
- **Pendente:** evidências insuficientes.

Registre resultados parcialmente atendidos na análise, sem confundi-los com aprovação completa.

A implementação da POC pode ser concluída mesmo que alguma hipótese seja reprovada, desde que os experimentos tenham sido realizados e documentados adequadamente.

## 15. Fora do escopo

Não implemente:

- Gerenciamento definitivo de notas Markdown.
- Salvamento automático.
- Exportação de notas.
- Configurações GSettings.
- Preferências persistentes.
- Internacionalização.
- Integração automática com GNOME Shell.
- Novas funcionalidades de transparência.
- Refatorações extensas do código Rust.
- Pipeline de publicação GitHub Actions.
- Publicação no Flathub.
- Publicação no GitHub Releases.
- Assinatura de releases.
- Sistema de atualização automática.
- Funcionalidades adicionais do MVP.

Não modifique o repositório principal `oak-note`.

## 16. Relatório consolidado das POCs

Esta é a última POC prevista no planejamento inicial.

Além de `docs/poc-04-results.md`, crie:

`docs/poc-summary.md`

Esse documento deverá consolidar os resultados das POCs 00–04.

Inclua:

1. Objetivo geral da investigação.
2. Hipóteses investigadas.
3. Resultados efetivamente comprovados.
4. Tecnologias e versões utilizadas.
5. Soluções técnicas recomendadas.
6. Limitações identificadas.
7. Riscos ainda não resolvidos.
8. Decisões que podem ser aproveitadas no MVP.
9. Experimentos adicionais eventualmente necessários.

Não declare a POC geral concluída com sucesso se existirem hipóteses críticas reprovadas ou pendentes.

Diferencie claramente:

- Implementação realizada.
- Teste executado.
- Evidência obtida.
- Hipótese validada.
- Decisão recomendada.

O relatório deverá funcionar como referência técnica para o planejamento do MVP no repositório principal.

## 17. Entrega

Ao finalizar, apresente:

1. Resumo das alterações.
2. Arquivos criados e modificados.
3. Runtime, SDK e toolchain selecionados.
4. Procedimento de build Flatpak.
5. Procedimento de instalação.
6. Permissões efetivas do sandbox.
7. Resultados dos experimentos E01–E05.
8. Estado das hipóteses H04-A a H04-E.
9. Limitações e problemas encontrados.
10. Localização do bundle gerado, caso exista.
11. Resumo das conclusões das POCs 00–04.
12. Recomendação objetiva para a próxima fase do projeto.

Caso não seja possível executar os testes no ambiente atual, forneça o roteiro de execução manual e mantenha os resultados pendentes.

**Não crie commits, não publique alterações e não modifique o repositório principal `oak-note` sem autorização.**

Priorize simplicidade, segurança, reproducibilidade, código experimental mínimo e documentação baseada em evidências.
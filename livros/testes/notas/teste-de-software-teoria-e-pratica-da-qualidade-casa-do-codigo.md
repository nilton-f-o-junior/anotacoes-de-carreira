# Sumário

## 1 Introdução

- [x] 1.1 Introdução

## 2 Um pouco de Engenharia de Software

- [x] 2.1 O que é software?
- [x] 2.2 O que é Engenharia de Software?
- [x] 2.3 O que da Engenharia de Software será abordado neste livro?
- [x] 2.4 Conclusão

## 3 Qualidade de software

- [x] 3.1 O que é qualidade de software?
- [x] 3.2 Como atingir qualidade de software?
- [x] 3.3 Alcançando a qualidade
- [x] 3.4 Conclusão

## 4 Conceitos de teste de software

- [x] 4.1 O que é testar um software?
- [x] 4.2 Erro vs. defeito vs. falha
- [x] 4.3 Os pilares de um teste
- [x] 4.4 Dimensões do teste
- [x] 4.5 Conclusão

## 5 Aplicação de exemplo

- [x] 5.1 Apresentação
- [x] 5.2 Componentes para os testes
- [x] 5.3 Conclusão

## 6 Técnicas de teste: Como

- [x] 6.1 Teste de Caixa Branca (estrutural)
- [x] 6.2 Teste de Caixa Preta (funcional)
- [x] 6.3 Teste de Caixa Cinza
- [x] 6.4 Conclusão

## 7 Níveis de teste: Quando

- [x] 7.1 Unidade
- [x] 7.2 Integração
- [x] 7.3 Sistema
- [x] 7.4 Aceitação
- [x] 7.5 Conclusão

## 8 Tipos de teste: O quê

- [] 8.1 Funcional
- [] 8.2 Regressão
- [] 8.3 Performance
- [] 8.4 Usabilidade
- [] 8.5 Segurança
- [] 8.6 Acessibilidade
- [] 8.7 Portabilidade
- [] 8.8 Como + Quando + O que: como testes de fato são realizados
- [] 8.9 Fatores McCall e Atributos ISO 9126 vs. Tipos de Teste
- [] 8.10 Conclusão

## 9 Como realizar testes

- [x] 9.1 Planejar
- [x] 9.2 Projetar
- [x] 9.3 Implementar
- [x] 9.4 Executar
- [x] 9.5 Avaliar
- [x] 9.6 Conclusão

## 10 Testes ágeis

- [x] 10.1 O manifesto ágil dos testes
- [x] 10.2 Aplicando testes ágeis
- [x] 10.3 Conclusão

## 11 Apêndice I: Padrões de validação de código

- [] 11.1 FindBugs
- [] 11.2 CheckStyle
- [] 11.3 Sonar
- [] 11.4 Conclusão

## 12 Apêndice II: Modelos de testes

- [] 12.1 TDD — Test Driven Development
- [] 12.2 BDD — Behavior Driven Development
- [] 12.3 ATDD — Acceptance Test-Driven Development
- [] 12.4 Conclusão

## 13 Apêndice III: Exemplo de Plano de Teste e Cronograma de Teste

- [] 13.1 Plano de Teste
- [] 13.2 Cronograma de teste

## 14 Apêndice IV: Exemplo de uso do Testlink e Mantis

- [] 14.1 Testlink
- [] 14.2 Mantis
- [] 14.3 Conclusão

## 1 Introdução

Contexto! Pula!

"O teste tem a característica de mostrar a presença de erros e não sua ausência." Dijkstra (1972)

## 2 Um pouco de Engenharia de Software

### 2.1 O que é software?

- Conjunto de instruções que automatizam tarefas, que engloba:
  - O programa em si (código);
  - Toda a sua documentação;
  - O hardware sobre o qual funciona;
  - As pessoas envolvidas em sua criação e utilização

### 2.2 O que é Engenharia de Software?

- O Processo de Software:
  - Especificação: etapa de entendimento do que deve ser feito;
  - Desenvolvimento: etapa de transformação "do que deve ser feito" em "como deve ser feito";
  - Validação: etapa de avaliação de que "o que foi feito" está de acordo com o "entendido e desejado";
  - Evolução: etapa de melhorias, que serão constantes durante a vida útil do software.

### 2.3 O que da Engenharia de Software será abordado neste livro?

- Pula!

### 2.4 Conclusão

- Pula!

## 3 Qualidade de software

### 3.1 O que é qualidade de software?

Qualidade é entregar tudo o que o cliente quer, funcionando da forma desejada e dentro dos prazos e custos acordados.

- Desafios:
  - Comunicação: a dificuldade do cliente em expressar claramente suas necessidades;
  - Interpretação: diferentes percepções dos desenvolvedores sobre os requisitos;
  - Fatores Externos: mudanças constantes de requisitos e desafios tecnológicos que impactam prazos e custos.

### 3.2 Como atingir qualidade de software?

- Contexto!

### 3.3 Alcançando a qualidade

Validação e Verificação (V & V):

- Validação ("Estamos construindo o software certo?"): foca em garantir que os requisitos reflitam as reais necessidades do cliente;
  - Validade: garante que as funções especificadas realmente atendam ao que o usuário precisa;
  - Consistência: verifica se não existem conflitos ou contradições entre os diferentes requisitos;
  - Completeza: assegura que todas as funções solicitadas foram incluídas de forma clara e integral;
  - Realismo: avalia se os requisitos podem ser implementados com o orçamento e as tecnologias disponíveis;
  - Verificabilidade: define se é possível criar testes práticos para checar cada requisito.

- Verificação ("Estamos construindo certo o software?"): foca em garantir que o código reflita fielmente o que foi especificado nos requisitos.
  - Perspectiva Funcional: avalia se o sistema gera as respostas corretas para entradas de dados específicas;
  - Perspectiva Não Funcional: testa o comportamento sob estresse, carga de dados e capacidade de recuperação;
  - Verificação Estática: analisa o código-fonte e documentos sem precisar executar o software;
  - Verificação Dinâmica: avalia o comportamento do software enquanto ele está em pleno funcionamento.

Fatores de McCall:

- Revisão: enfatiza a facilidade de modificação do software.
  - Manutenção: esforço para corrigir erros ou evoluir funções;
  - Flexibilidade: esforço para modificar o software em operação;
  - Testabilidade: esforço necessário para testar o software.

- Transição: Foca na capacidade de adaptabilidade.
  - Portabilidade: esforço para levar o software a várias plataformas;
  - Reusabilidade: quanto do software pode ser reaproveitado em outros módulos ou sistemas;
  - Interoperabilidade: esforço para integrar o software com outros sistemas.
- Operação: Analisa o ponto de vista operacional.
  - Correção: conformidade com as especificações;
  - Confiabilidade: geração de resultados esperados a partir de entradas acordadas;
  - Usabilidade: facilidade de uso;
  - Integridade: proteção contra danos aos dados;
  - Eficiência: uso racional dos recursos computacionais.

ISO 9126:

- Funcionalidade: refere-se à capacidade do software de prover funções que satisfaçam as necessidades do usuário.
  - Adequação: avalia se as funcionalidades disponibilizadas estão de acordo com as especificações definidas pelos usuários;
  - Acurácia: verifica se o software se comporta conforme o esperado, gerando os resultados ou efeitos definidos;
  - Interoperabilidade: observa se o sistema interage adequadamente com todos os outros softwares especificados;
  - Segurança: analisa a capacidade de proteger informações e dados, permitindo acesso apenas a usuários autorizados;
  - Conformidade: garante que o software obedece a normas, convenções ou regulamentações legais.

- Confiabilidade: avalia a capacidade do software de manter seu comportamento esperado, mesmo em situações adversas.
  - Maturidade: foca na capacidade do sistema de evitar o surgimento de outras falhas após a ocorrência de erros;
  - Tolerância a falhas: verifica se o software consegue continuar íntegro mesmo após a ocorrência de falhas;
  - Recuperabilidade: mede a capacidade de retomar o funcionamento normal e recuperar dados após uma falha;
  - Conformidade: assegura a obediência às normas e regulamentações relativas à confiabilidade.

- Usabilidade: mede a facilidade de compreensão, aprendizado e operação do software quando usado sob condições especificadas.
  - Inteligibilidade: avalia se o software é de fácil compreensão para permitir seu uso da forma como foi projetado;
  - Apreensibilidade: observa se o sistema possibilita ao usuário aprender sobre seu funcionamento e o fluxo de trabalho ao qual pertence;
  - Operacionalidade: analisa se o usuário consegue operar e controlar o software de forma fácil e intuitiva;
  - Proteção frente a erros de usuários: verifica se o sistema consegue prevenir operações equivocadas por parte de quem o utiliza;
  - Estética/Atratividade: Considera se o software consegue angariar novos usuários pela interface gráfica e capacidade de automação;
  - Acessibilidade: garante que usuários com necessidades especiais consigam operar o software normalmente;
  - Conformidade: cerifica a obediência a convenções, guias ou regulamentações de usabilidade.

- Eficiência: foca no fornecimento de tempos de resposta satisfatórios e no uso racional de recursos.
  - Tempo de resposta: avalia se os resultados são fornecidos dentro dos prazos definidos nas especificações dos usuários;
  - Consumo de recursos: verifica se o uso de processador e memória ocorre de forma otimizada para evitar interrupções;
  - Conformidade: garante o acordo com normas, guias de estilo ou regulamentações de eficiência.

- Manutenibilidade: avalia a facilidade de realizar manutenções corretivas ou evolutivas no software.
  - Analisabilidade: mede a facilidade e rapidez em identificar a causa de erros no sistema;
  - Modificabilidade: analisa a facilidade de alterar o comportamento das funcionalidades existentes;
  - Estabilidade: verifica a capacidade de evitar efeitos colaterais indesejados resultantes de modificações realizadas;
  - Testabilidade: avalia a possibilidade de realizar testes tanto no que foi alterado quanto no que permaneceu igual;
  - Conformidade: garante que o software segue normas e convenções de manutenibilidade.

- Portabilidade: verifica a capacidade de o software ser disponibilizado em diferentes plataformas e ambientes.
  - Adaptabilidade: mede se o software pode ser implantado em diversas plataformas sem a necessidade de adaptações manuais;
  - Instalabilidade: avalia a facilidade de instalar o sistema em um novo ambiente;
  - Coexistência: analisa se o software interage de forma segura e harmônica com outros sistemas operando no mesmo ambiente;
  - Substituibilidade: verifica se o produto é capaz de substituir outro software dentro de um mesmo contexto de uso;
  - Conformidade: assegura que o sistema está de acordo com as normas e regulamentações de portabilidade.

### 3.4 Conclusão

- Pula!

## 4 Conceitos de teste de software

### 4.1 O que é testar um software?

Teste de software consiste na verificação dinâmica do comportamento de um software, por meio de um conjunto finito de casos de teste, selecionados a partir de um conjunto infinito de possibilidades, contra um comportamento esperado e especificado.

- Verificação dinâmica: consiste na realização de testes com o produto ou componente de software em efetiva execução;
- Conjunto finito: refere-se à aplicação de uma quantidade limitada de casos de teste selecionados pelo equilíbrio entre recursos e requisitos;
- Comportamento esperado: define os resultados aceitáveis para comparar o que foi observado no software com sua especificação ou expectativa.

### 4.2 Erro vs. defeito vs. falha

- Erro: é uma ação humana incorreta (na codificação ou nos requisitos);
- Defeito: é a manifestação do erro no código ou documento (o "bug" interno);
- Falha: é o comportamento inesperado percebido pelo usuário final quando o defeito é executado.

Exemplo:

- Erro: Você se distrai e mede a madeira errado (é a ação humana incorreta);
- Defeito: A madeira fica com um furo no lugar errado (é o "bug" escondido no objeto);
- Falha: Quando você coloca um livro, a prateleira quebra (é o problema aparecendo para o usuário).

### 4.3 Os pilares de um teste

- Verificação Dinâmica: é como testar um carro com o motor ligado e andando, em vez de apenas olhar o manual;
- Conjunto Finito: é impossível testar todas as combinações do mundo, você escolhe apenas os cenários mais importantes para economizar tempo e dinheiro;
- Comportamento Esperado: você já deve saber qual é a resposta certa. Se você soma 2+2, o esperado é 4; qualquer outra coisa é um defeito.

Enfoque em dados:

- Particionamento de Equivalência: consiste em criar conjuntos de estados válidos e inválidos, assumindo que o software terá o mesmo comportamento para qualquer dado pertencente a uma mesma classe;

- Análise de Valores Limites: foca especificamente nos limites (mínimo e máximo) das classes de equivalência, pois é onde existe a maior probabilidade de comportamentos inadequados.

Exemplo:

- Particionamento de Equivalência: se o sistema aceita idades de 18 a 45 anos, você não testa todos os números. Você testa o 20 e assume que, se ele funcionar, todos os outros desse grupo também funcionarão;
- Análise de Valores Limites: em vez de testar o 30, você testa o 17 (inválido), o 18 (mínimo), o 45 (máximo) e o 46 (inválido).

Enfoque nas regras de negócio:

- Tabela de Decisão: é como uma lista de condições. "SE o cliente tem cupom E é primeira compra, ENTÃO ganha 20% de desconto". Você organiza isso em uma tabela para não esquecer nenhuma combinação;

- Pairwise: quando há milhares de combinações (tipo de frete + cor do produto + forma de pagamento), você usa uma técnica matemática para testar apenas os pares de combinações mais diferentes, reduzindo drasticamente o trabalho.

Enfoque em seleção de casos de teste:

- Baseado em Modelo: você desenha um mapa (fluxograma) de como o software deve agir e usa ferramentas para criar os testes automaticamente a partir desse desenho;

- Baseado em Caso de Uso: você segue o "manual do usuário". Passo 1: Abre o site. Passo 2: Digita o nome. Passo 3: Clica em salvar. O teste verifica se esse roteiro funciona;

- Teste Exploratório: é o teste "vontade própria". O testador mexe livremente no software como se fosse um usuário curioso, aprendendo como ele funciona e tentando "quebrá-lo" sem um roteiro fixo.

### 4.4 Dimensões do teste

1ª Dimensão:

Técnicas de Teste (Como testar?): esta dimensão define a perspectiva avaliativa e o modo como o teste será realizado, dependendo do acesso que se tem ao interior do software.

- Estrutural (Caixa Branca): avalia as estruturas internas e o código-fonte. O testador cria cenários para exercitar laços, decisões e unidades de código específicas;

- Funcional (Caixa Preta): avalia o comportamento do software como um todo através de suas interfaces, sem acesso ao código. Foca nos requisitos e na visão do usuário final;

- Caixa Cinza: uma técnica intermediária que usa o conhecimento do código ou da arquitetura aliado às saídas esperadas em alto nível, como consultas ao banco de dados para validar telas.

2ª Dimensão:

Níveis de Teste (Quando testar?): refere-se ao momento e à abrangência do teste dentro da estrutura do software, evoluindo da menor parte até o sistema completo.

- Unidade: testa a menor porção de código executável, como métodos e funções, visando prevenir bugs de lógica interna;
- Integração: verifica se as classes, módulos e camadas funcionam bem em conjunto, além de validar conexões com APIs externas;
- Sistema: executado com o software totalmente integrado em um ambiente similar ao de produção, buscando identificar falhas sob a ótica do usuário;
- Aceitação: o nível mais alto, realizado pelo cliente ou usuários finais (homologação), para avaliar a qualidade percebida antes da implantação definitiva.

3ª Dimensão:

Tipos de Teste (O que testar?): define a perspectiva comportamental e as condições específicas sob as quais o software deve ser validado.

- Funcional: garante que as funções cumprem seus requisitos e regras de negócio;
- Performance: avalia o comportamento em cenários de estresse, dividindo-se em desempenho (picos previstos), carga (picos não previstos) e volume (massa de dados no banco);
- Usabilidade: mede o grau de facilidade de uso, manuseio e interatividade para o usuário final;
- Segurança: identifica brechas que possam comprometer o sigilo e a integridade dos dados;
- Acessibilidade: garante que o software possa ser utilizado por qualquer pessoa, incluindo portadores de deficiências;
- Portabilidade: valida se o sistema funciona corretamente em diferentes plataformas, navegadores e dispositivos;
- Regressão: estratégia de reexecutar testes já realizados após modificações para garantir que não surgiram novos defeitos ("efeitos colaterais").

### 4.5 Conclusão

- Pula!

## 5 Aplicação de exemplo

### 5.1 Apresentação

- Leia!

### 5.2 Componentes para os testes

- Leia!

### 5.3 Conclusão

- Leia!

## 6 Técnicas de teste: Como

### 6.1 Teste de Caixa Branca (estrutural)

- Contexto!

### 6.2 Teste de Caixa Preta (funcional)

- Contexto!

### 6.3 Teste de Caixa Cinza

- Contexto!

### 6.4 Conclusão

Técnicas de Teste (Como testar?): esta dimensão define a perspectiva avaliativa e o modo como o teste será realizado, dependendo do acesso que se tem ao interior do software.

- Estrutural (Caixa Branca): avalia as estruturas internas e o código-fonte. O testador cria cenários para exercitar laços, decisões e unidades de código específicas;
- Funcional (Caixa Preta): avalia o comportamento do software como um todo através de suas interfaces, sem acesso ao código. Foca nos requisitos e na visão do usuário final;
- Caixa Cinza: uma técnica intermediária que usa o conhecimento do código ou da arquitetura aliado às saídas esperadas em alto nível, como consultas ao banco de dados para validar telas.

## 7 Níveis de teste: Quando

### 7.1 Unidade

É o nível mais baixo e foca na menor porção de código executável, como métodos e funções.

- Responsabilidade: deve ser realizado pelas próprias pessoas desenvolvedoras;
- Objetivo: garantir que a lógica interna do código esteja correta para entradas específicas, prevenindo bugs de escrita e falta de tratamento de exceções.

### 7.2 Integração

Verifica a execução combinada de classes, módulos, componentes e camadas, além de conexões com APIs externas.

- Abordagens: pode ser não incremental (Big-Bang, testando tudo ao final) ou incremental (Cima-baixo ou Baixo-cima, testando partes durante a construção);
- Objetivo: assegurar que as partes do sistema funcionem bem em conjunto e troquem informações corretamente.

### 7.3 Sistema

Acontece quando o software está completamente integrado (ou em versão MVP) e é testado como um todo.

- Responsabilidade: deve ser feito por uma equipe de testes independente para evitar testes "viciados";
- Objetivo: identificar falhas sob a ótica do usuário final em um ambiente similar ao de produção, usando requisitos funcionais e não funcionais;
- Modalidades: pode ser manual (seguindo scripts passo a passo) ou automatizado (simulando interações humanas).

### 7.4 Aceitação

É o nível mais alto, realizado quando o software está pronto para implantação.

- Responsabilidade: realizado pelo cliente ou usuários finais (também chamado de homologação);
- Objetivo: avaliar a qualidade percebida e verificar se o sistema atende às necessidades reais de quem o solicitou;
- Fases: pode ocorrer na fase alfa (ambiente de desenvolvimento) ou beta (ambiente que emula a produção).

### 7.5 Conclusão

- Leia!

## 8 Tipos de teste: O quê

### 8.1 Funcional

Avalia se as funcionalidades do software se comportam de acordo com suas especificações e requisitos.

- Execução: são realizados cenários de uso com massas de dados (reais ou fictícias) para verificar se as regras de negócio e casos de uso foram codificados adequadamente;
- Roteiros: uma forma simples de aplicação é seguir roteiros de teste, como validar se um login é efetuado com sucesso ou se exibe a mensagem correta para senhas inválidas;
- Teste de Fumaça (Smoke Test): uma estratégia complementar que foca em testar funcionalmente apenas as partes principais e críticas do sistema;
- Responsabilidade: geralmente realizado pela equipe de desenvolvimento para checar verificações mínimas antes de liberar o software para uma equipe de testes independente.

### 8.2 Regressão

Embora tratado como um tipo, funciona mais como uma estratégia de execução.

- Propósito: consiste em reexecutar todos os testes do software após qualquer modificação ou correção em uma funcionalidade específica;
- Efeitos colaterais: o objetivo principal é garantir que as novas alterações não criaram, inadvertidamente, problemas ou erros em outros pontos que já estavam funcionando
- Automação: como a abordagem manual se torna inviável conforme o software cresce, o uso de scripts automáticos acionados via ferramentas de Integração Contínua (CI) é a prática mais comum.

### 8.3 Performance

Foca nos requisitos não funcionais, avaliando o comportamento do software em cenários atípicos de execução. Ele é subdividido em três categorias:

- Desempenho: testa o software em situações de pico máximo de acesso previsto para identificar gargalos em conexões HTTP, bancos de dados ou APIs lentas;

- Carga: avalia o comportamento sob situações de estresse não previstas (ex: acessos 5 vezes maiores que o normal) ou mudanças bruscas de infraestrutura, como redução de memória disponível;

- Volume: foca especificamente no comportamento do banco de dados ao lidar com grandes quantidades de dados, seja para armazenamento (inserts) ou recuperação (selects).

### 8.4 Usabilidade

Avalia o nível de facilidade de uso, manuseio e interatividade do software sob a ótica do usuário final.

- Heurísticas: Baseia-se frequentemente nos 10 quesitos de Nielsen, que incluem a visibilidade do status do sistema, prevenção de erros e estética minimalista;
- Métodos: Pode ser realizado via rastreamento ocular (eye-tracking) para mapear o foco visual, ou através de testes exploratórios e de avaliação com tarefas predefinidas;
- Abordagens: Os testes podem ser moderados (com acompanhamento) ou não moderados, além de serem realizados de forma presencial ou remota.

### 8.5 Segurança

O foco deste tipo de teste é garantir que o software e seus dados sejam acessados apenas por usuários autorizados, detectando brechas que comprometam o sigilo.

- OWASP: utiliza as diretrizes do projeto OWASP para mitigar vulnerabilidades críticas como SQL Injection e Cross-Site Scripting (XSS);
- SAST e DAST: aplica o SAST para análise estática do código-fonte (caixa branca) e o DAST para avaliar o software em execução simulando ataques externos (caixa preta);
- Times Coloridos: a segurança pode envolver times especializados: Red Team (ataque), Blue Team (defesa) e Purple Team (integração entre ambos);
- PenTest: realização de Testes de Penetração, que são ataques simulados para encontrar falhas em aplicações, redes, hardware ou no fator humano.

### 8.6 Acessibilidade

Visa garantir que o software possa ser utilizado por todas as pessoas de forma autônoma, incluindo portadores de necessidades especiais.

- Padrões W3C: segue as diretrizes internacionais da W3C/WCAG, organizadas em três níveis de conformidade: A (mínimo), AA (padrão recomendado) e AAA (máximo);
- Princípios: baseia-se em quatro pilares fundamentais: Perceptibilidade, Operacionalidade, Compreensibilidade e Robustez;
- Contexto Brasileiro: no Brasil, utiliza-se também o modelo eMAG e a ferramenta ASES para validar a acessibilidade em sítios do governo.

### 8.7 Portabilidade

Visa garantir que o software funcione corretamente e mantenha a usabilidade em diferentes plataformas e versões.

- Cenário: essencial para aplicações que precisam rodar em múltiplos navegadores (Chrome, Firefox, etc.) e diferentes sistemas operacionais móveis (iOS e Android);
- Execução: o software deve ser testado em dispositivos físicos variados para detectar comportamentos inesperados, podendo combinar testes funcionais e de desempenho durante essa validação.

### 8.8 Como + Quando + O que: como testes de fato são realizados

Na prática, as três dimensões de teste se completam e são utilizadas simultaneamente.

- Sinergia: não se trabalha de forma isolada; cada dimensão provê uma porção necessária para cobrir as partes eleitas para teste. Exemplo?:
  - Testar um método de classe envolve técnica de caixa branca (Como), nível unitário (Quando) e tipo funcional (O que).

### 8.9 Fatores McCall e Atributos ISO 9126 vs. Tipos de Teste

- Leia!

### 8.10 Conclusão

- Pula!

## 9 Como realizar testes

### 9.1 Planejar

- Escopo: consiste em selecionar as partes do software que serão testadas, priorizando as funcionalidades com maior valor de negócio para o cliente. Também define os limites do sistema, identificando o que é interno ao domínio e o que está fora dele para evitar falhas na qualidade final;

- Recursos: envolve a escolha de ferramentas e componentes adequados (como JUnit, Mockito e Selenium para Java, ou Jasmine e Karma para Angular), além da definição da equipe e da infraestrutura necessária. O ambiente de teste deve, preferencialmente, ser igual ao de produção para garantir resultados fidedignos;

- Estimativas: busca mensurar o tempo e os custos para a execução de todas as etapas (planejamento, projeto, implementação, execução e avaliação). Embora prazos em software sejam delicados, essas estimativas são essenciais para a criação de cronogramas.

- Estratégias e técnicas: define quais estratégias de teste e técnicas (como Caixa Preta, Branca ou Cinza) serão aplicadas em cenários específicos.

Para realizar o planejamento, os insumos necessários:

- Requisitos do software (casos de uso, histórias de usuário);;
- Cronograma geral do projeto;
- Configurações de hardware e software.

Resultados

- Plano de Testes, que detalha como os testes serão executados;
- Cronograma de Teste, com os períodos de cada tarefa.

### 9.2 Projetar

- Projetar os casos de teste: as funcionalidades são analisadas para definir se serão validadas por testes unitários, de integração ou sistema manual, além da identificação dos dados de entrada e saída (falsos, reais ou embaralhados);

- Avaliar possibilidade de reúso: verifica-se se scripts de testes de outros softwares podem ser adaptados para reduzir tempo e custos;

- Identificar produtos e componentes de apoio: selecionam-se ferramentas para simular integrações externas (como mocks) ou para configurar limites de infraestrutura que viabilizem os cenários reais;

- Elaborar modelo de performance: caso necessário, identificam-se características de arquitetura, rede e hardware que impactam a eficiência do sistema;

- Projetar ambiente de teste: definem-se os ambientes de implementação (máquina do desenvolvedor) e de execução (similar à produção), especificando sistemas operacionais e versões de componentes;

- Projetar massa de teste: define-se a coleção de dados considerando quatro atributos de qualidade: profundidade (volume), largura (variância), escopo (relevância) e arquitetura (estrutura física);

- Verificar modelo de teste: realização de uma revisão final pela equipe para garantir que o projeto atenderá às necessidades do software.

Para realizar essa etapa, os insumos necessários são:

- Requisitos de software;
- Modelo de análise e projeto;
- Configurações de hardware/software;
- Plano de Teste.

Resultados

- Modelo de Teste (casos, dados e carga);
- Configurações do Ambiente de Teste;
- Simuladores de Teste.

### 9.3 Implementar

- Criar scripts de teste: elaboração do passo a passo para testes manuais (Sistema ou Aceitação) ou de pseudocódigos que guiarão a codificação de testes automatizados (Unitário, Integração ou Sistema);

- Implementar massa de teste: criação física dos dados em arquivos ou bancos de dados, definindo sua origem (produção ou externa), critérios (aleatórios ou reais embaralhados) e objetivos (funcionalidade, segurança ou performance);

- Implementar ambiente de teste: montagem (instalação de SO e navegadores) e configuração (ajuste de CPU e memória) da infraestrutura para que seja o mais fiel possível ao ambiente de produção;

- Implementar produtos e componentes de apoio: codificação de mocks para simular funcionalidades internas ainda não desenvolvidas ou integrações com softwares externos;

- Montar suíte de teste: agrupamento de vários Casos de Teste interdependentes que cobrem diferentes caminhos de uma mesma funcionalidade (ex: fluxos de sucesso e fluxos de erro no cadastro).

Para realizar essa etapa, os insumos necessários são:

- Modelo de Análise e Projeto;
- Configurações de hardware/software;
- Plano de Teste;
- Casos de Teste projetados.

Resultados

- Modelo de Teste.

### 9.4 Executar

- Executar teste: consiste na rodagem dos scripts manuais (utilizando o software diretamente) ou disparo dos códigos automatizados via ferramentas de integração contínua, sempre verificando se a versão disponibilizada (build) atende aos requisitos mínimos;

- Registrar defeitos: as falhas detectadas devem ser documentadas em ferramentas de Bug Tracker, detalhando obrigatoriamente o roteiro executado e os dados usados para que a equipe de desenvolvimento consiga reproduzir e corrigir o erro;

- Analisar defeitos dos testes: envolve uma análise crítica dos relatos para determinar se os defeitos permanecem abertos para correção ou se são descartados por inconformidade, podendo resultar na reexecução de todos os testes caso os resultados sejam insatisfatórios.

Para realizar essa etapa, os insumos necessários são:

- Modelo de Teste (contendo as suítes, scripts e dados de massa);
- Build do software.

Resultados

- Relatórios de execução;
- Registros formais de bugs.

### 9.5 Avaliar

- Avaliar a completude dos testes: analisa se a execução seguiu os critérios de aceitação definidos e o Modelo de Teste projetado;

- Avaliar cobertura dos testes: verifica se os testes exercitaram as linhas de código responsáveis pelas funcionalidades e se os comportamentos previstos nos requisitos foram validados;

- Avaliar resultados de testes: mensura a qualidade do software e documenta as conclusões no Relatório de Avaliação de Testes;

- Avaliar atividades de testes: revisa todo o processo, desde o planejamento até a execução, com o intuito de aprimorar a metodologia para futuros projetos.

Para realizar essa etapa, os insumos necessários são:

- Relatório de Execução de Teste;
- Plano de Teste;
- Modelo de Teste;
- Requisitos de software.

Resultados

- Relatório de Avaliação de Teste: detalha a severidade dos problemas, quantitativos de erros e a relação entre defeitos e funcionalidades.

### 9.6 Conclusão

- Pula!

## 10 Testes ágeis

### 10.1 O manifesto ágil dos testes

- Testar durante todo o ciclo, e não apenas ao final do desenvolvimento;
- Prevenir bugs em vez de apenas encontrá-los;
- Testar o valor de negócio e não apenas o comportamento técnico;
- Buscar a melhoria constante do software em vez de apenas apontar suas fraquezas;
- Responsabilidade coletiva, tornando a qualidade um dever de todo o time e não apenas da pessoa testadora.

### 10.2 Aplicando testes ágeis

Testador Ágil, a pessoa testadora deixa de ser apenas uma executora para se tornar uma ponte estratégica entre o cliente e o time de desenvolvimento. Ela participa desde a descoberta de requisitos até a entrega final, auxiliando desenvolvedores a criarem códigos mais testáveis e agregando valor de negócio ao produto.

### 10.3 Conclusão

- Pula!

## 11 Apêndice I: Padrões de validação de código

- [] 11.1 FindBugs
- [] 11.2 CheckStyle
- [] 11.3 Sonar
- [] 11.4 Conclusão

## 12 Apêndice II: Modelos de testes

- [] 12.1 TDD — Test Driven Development
- [] 12.2 BDD — Behavior Driven Development
- [] 12.3 ATDD — Acceptance Test-Driven Development
- [] 12.4 Conclusão

## 13 Apêndice III: Exemplo de Plano de Teste e Cronograma de Teste

- [] 13.1 Plano de Teste
- [] 13.2 Cronograma de teste

## 14 Apêndice IV: Exemplo de uso do Testlink e Mantis

### 14.1 Testlink

### 14.2 Mantis

### 14.3 Conclusão

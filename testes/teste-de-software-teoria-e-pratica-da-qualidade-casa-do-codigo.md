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

- [] 4.1 O que é testar um software?
- [] 4.2 Erro vs. defeito vs. falha
- [] 4.3 Os pilares de um teste
- [] 4.4 Dimensões do teste
- [] 4.5 Conclusão

## 5 Aplicação de exemplo

- [] 5.1 Apresentação
- [] 5.2 Componentes para os testes
- [] 5.3 Conclusão

## 6 Técnicas de teste: Como

- [] 6.1 Teste de Caixa Branca (estrutural)
- [] 6.2 Teste de Caixa Preta (funcional)
- [] 6.3 Teste de Caixa Cinza
- [] 6.4 Conclusão

## 7 Níveis de teste: Quando

- [] 7.1 Unidade
- [] 7.2 Integração
- [] 7.3 Sistema
- [] 7.4 Aceitação
- [] 7.5 Conclusão

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

- [] 9.1 Planejar
- [] 9.2 Projetar
- [] 9.3 Implementar
- [] 9.4 Executar
- [] 9.5 Avaliar
- [] 9.6 Conclusão

## 10 Testes ágeis

- [] 10.1 O manifesto ágil dos testes
- [] 10.2 Aplicando testes ágeis
- [] 10.3 Conclusão

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

- [] 7.1 Unidade
- [] 7.2 Integração
- [] 7.3 Sistema
- [] 7.4 Aceitação
- [] 7.5 Conclusão

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

- [] 9.1 Planejar
- [] 9.2 Projetar
- [] 9.3 Implementar
- [] 9.4 Executar
- [] 9.5 Avaliar
- [] 9.6 Conclusão

## 10 Testes ágeis

- [] 10.1 O manifesto ágil dos testes
- [] 10.2 Aplicando testes ágeis
- [] 10.3 Conclusão

## 11 Apêndice I: Padrões de validação de código

- [] 11.1 FindBugs
- [] 11.2 CheckStyle
- [] 11.3 Sonar
- [] 11.4 Conclusão

## 12 Apêndice II: Modelos de testes

- []12.1 TDD — Test Driven Development
- []12.2 BDD — Behavior Driven Development
- []12.3 ATDD — Acceptance Test-Driven Development
- []12.4 Conclusão

## 13 Apêndice III: Exemplo de Plano de Teste e Cronograma de Teste

- [] 13.1 Plano de Teste
- [] 13.2 Cronograma de teste

## 14 Apêndice IV: Exemplo de uso do Testlink e Mantis

- [] 14.1 Testlink
- [] 14.2 Mantis
- [] 14.3 Conclusão

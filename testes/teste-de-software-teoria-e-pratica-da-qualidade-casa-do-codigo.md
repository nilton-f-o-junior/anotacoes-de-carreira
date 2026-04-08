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

- Validação e Verificação (V & V):
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

- Fatores de McCall:
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

- ISO 9126:
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

### 4.2 Erro vs. defeito vs. falha

### 4.3 Os pilares de um teste

### 4.4 Dimensões do teste

### 4.5 Conclusão

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

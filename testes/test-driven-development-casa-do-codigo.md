# Sumário

## 1 Introdução

- [x] 1.1 Era uma vez um projeto sem testes
- [x] 1.2 Por que devemos testar?
- [x] 1.3 Por que não testamos?
- [x] 1.4 Testes automatizados e TDD
- [x] 1.5 Conclusão

## 2 Testes de Unidade

- [x] 2.1 O que é um teste de unidade?
- [x] 2.2 Preciso mesmo escrevê-los?
- [x] 2.3 O Primeiro Teste de Unidade
- [x] 2.4 Continuando a testar
- [x] 2.5 Conclusão

## 3 Introdução ao Test-Driven Development

- [x] 3.1 O problema dos números romanos
- [x] 3.2 O primeiro teste
- [x] 3.3 Refletindo sobre o assunto
- [x] 3.4 Quais as vantagens?
- [x] 3.5 Um pouco da história de TDD
- [x] 3.6 Conclusão

## 4 Simplicidade e Baby Steps

- [x] 4.1 O Problema do Cálculo de Salário
- [x] 4.2 Implementando da maneira mais simples possível
- [x] 4.3 Passos de Bebê (ou Baby Steps)
- [x] 4.4 Usando baby steps de maneira consciente
- [x] 4.5 Conclusão

## 5 TDD e Design de Classes

- [x] 5.1 O Problema do Carrinho de Compras
- [x] 5.2 Testes que influenciam no design de classes
- [x] 5.3 Diferenças entre TDD e testes da maneira tradicional
- [x] 5.4 Testes como rascunho
- [x] 5.5 Conclusão

## 6 Qualidade no Código do Teste

- [x] 6.1 Repetição de código entre testes
- [x] 6.2 Nomenclatura dos testes
- [x] 6.3 Test Data Builders
- [x] 6.4 Testes Repetidos
- [x] 6.5 Escrevendo boas asserções
- [x] 6.6 Testando listas
- [x] 6.7 Separando as Classes de Teste
- [x] 6.8 Conclusão

## 7 TDD e a Coesão

- [] 7.1 Novamente o Problema do Cálculo de Salário
- [] 7.2 Ouvindo o feedback dos testes
- [] 7.3 Testes em métodos privados?
- [] 7.4 Resolvendo o Problema da Calculadora de Salário
- [] 7.5 O que olhar no teste em relação a coesão?
- [] 7.6 Conclusão

## 8 TDD e o Acoplamento

- [] 8.1 O Problema da Nota Fiscal
- [] 8.2 Mock Objects
- [] 8.3 Dependências explícitas
- [] 8.4 Ouvindo o feedback dos testes
- [] 8.5 Classes estáveis
- [] 8.6 Resolvendo o Problema da Nota Fiscal
- [] 8.7 Testando métodos estáticos
- [] 8.8 TDD e a constante criação de interfaces
- [] 8.9 O que olhar no teste em relação ao acoplamento?
- [] 8.10 Conclusão

## 9 TDD e o Encapsulamento

- [] 9.1 O Problema do Processador de Boleto
- [] 9.2 Ouvindo o feedback dos testes
- [] 9.3 Tell, Don’t Ask e Lei de Demeter
- [] 9.4 Resolvendo o Problema do Processador de Boletos
- [] 9.5 O que olhar no teste em relação ao encapsulamento?
- [] 9.6 Conclusão

## 10 Testes de Integração e TDD

- [] 10.1 Testes de unidade, integração e sistema
- [] 10.2 Quando não usar mocks?
- [] 10.3 Testes em DAOs
- [] 10.4 Devo usar TDD em testes de integração?
- [] 10.5 Testes em aplicações Web
- [] 10.6 Conclusão

## 11 Quando não usar TDD?

- [] 11.1 Quando não praticar TDD?
- [] 11.2 100% de cobertura de código?
- [] 11.3 Devo testar códigos simples?
- [] 11.4 Erros comuns durante a prática de TDD
- [] 11.5 Como convencer seu chefe sobre TDD?
- [] 11.6 TDD em Sistemas Legados
- [] 11.7 Conclusão

## 12 E agora?

- [] 12.1 O que ler agora?
- [] 12.2 Dificuldade no aprendizado
- [] 12.3 Como interagir com outros praticantes?
- [] 12.4 Conclusão Final

## 13 Sumário Casa do Código

- 13 Apêndice: Princípios SOLID
- 13.1 Sintomas de Projetos de Classes em Degradação
- 13.2 Princípios de Projeto de Classes
- 13.3 Conclusão

## 1 Introdução

1.1 Era uma vez um projeto sem testes

O custo da falta de testes: O autor relata uma experiência real em um projeto de automação de postos de gasolina. O software, que não havia sido testado sob condições de alto volume de uso, falhou no primeiro dia de operação em um posto piloto, bloqueando as bombas por 12 horas. Esse exemplo ilustra como bugs podem causar prejuízos financeiros significativos e até situações críticas, como falhas em hospitais ou foguetes.

1.2 Por que devemos testar?

Porque tem problemas.

1.3 Por que não testamos?

A barreira do custo manual: O principal motivo para a falta de testes é que o teste manual é caro e demorado. Pagar uma pessoa para testar todo o sistema a cada pequena mudança é inviável para a maioria das equipes.

1.4 Testes automatizados e TDD

Testes Automatizados e TDD: A solução proposta é escrever programas que testam seus próprios programas. Isso traz vantagens como:

- Velocidade e frequência: Máquinas executam testes muito mais rápido que humanos, permitindo rodá-los constantemente.
- Feedback precoce: Problemas são descobertos mais cedo, o que reduz o custo de correção do bug.
- Nova visão de produtividade: A produtividade não deve ser medida por linhas de código escritas, mas pela quantidade de código de produção sem defeitos entregue.

  1.5 Conclusão

Analogia com a Medicina: O autor compara o desenvolvimento de software com a cirurgia. Assim como um médico jamais deixaria de lavar as mãos para ganhar tempo, um desenvolvedor não deve abrir mão da qualidade e dos testes para entregar uma funcionalidade mais rápido ou usar "gambiarras".

## 2 Testes de Unidade

2.1 O que é um teste de unidade?

Definição de Teste de Unidade: Diferente de um teste que cobre o sistema todo, o teste de unidade foca em uma pequena parte isolada, que em sistemas orientados a objetos geralmente é uma classe.

2.2 Preciso mesmo escrevê-los?

A Necessidade do Teste (Exemplo Prático): O autor apresenta uma classe chamada MaiorEMenor, que deveria encontrar o produto de maior e menor valor em um carrinho de compras. Embora pareça funcionar em um primeiro teste manual, o código apresenta um erro de lógica (um else mal posicionado) que causa uma falha (NullPointerException) se os produtos forem inseridos em ordem decrescente.

2.3 O Primeiro Teste de Unidade

Estrutura de um Teste Automatizado: Um teste automatizado segue a mesma lógica de um teste manual, dividindo-se em três etapas: Cenário (preparar os dados), Ação (executar o método) e Validação (verificar se a saída é a esperada).

CENÁRIO > AÇÃO > VALIDAÇÃO

- Cenário (Setup)
  - É a fase de preparação. Nela, você define as condições iniciais e os dados necessários para que o teste ocorra.
  - Exemplo: Se você vai testar um carrinho de compras, o cenário seria criar o objeto CarrinhoDeCompras e adicionar produtos específicos a ele (como um liquidificador e uma geladeira)

- Ação (Act)
  - É o momento em que você executa o comportamento que deseja testar. Geralmente, consiste na chamada de um método específico da classe sob teste.
  - Exemplo: Invocar o método algoritmo.encontra(carrinho) para que ele processe os dados do cenário montado.

- Validação (Assert)
  - É a etapa onde se verifica se o resultado obtido é o esperado. Nos testes manuais, um humano olha para a tela e confere o valor; nos automatizados, utilizamos ferramentas para que a máquina faça essa comparação.
  - Exemplo: Usar o comando Assert.assertEquals("Geladeira", algoritmo.getMaior().getNome()) para garantir que o sistema identificou o produto correto.

    2.4 Continuando a testar

Vantagens Práticas:

- Velocidade: O teste automatizado é executado em frações de segundo (ex: 0.007s), permitindo que o desenvolvedor o rode várias vezes ao dia.
- Testes de Regressão: Eles garantem que alterações futuras no código não quebrem funcionalidades que já estavam funcionando anteriormente.
- Segurança para Evolução: Mesmo cenários que parecem "óbvios" (como um carrinho com apenas um produto) devem ser testados para dar segurança a futuros desenvolvedores que venham a mexer no código.

  2.5 Conclusão

Nada de novo, pode seguir!

## 3 Introdução ao Test-Driven Development

3.1 O problema dos números romanos

Ele fala dos algarismos romanos! Pula!

3.2 O primeiro teste

Aqui ele fala sobre implementar um código que lê algarismos romanos.

- I = 1
- II = 2
- ...

  3.3 Refletindo sobre o assunto

Ele contextualiza o problema e implementa o código.

3.4 Quais as vantagens?

O Ciclo TDD (Red-Green-Refactor): O autor detalha o fluxo de trabalho repetitivo que define o TDD:

- Red (Vermelho): Escreve-se um teste de unidade para uma nova funcionalidade e observa-se o teste falhar.
- Green (Verde): Implementa-se o código mais simples possível para fazer o teste passar.
- Refactor (Refatorar): Melhora-se o código, removendo duplicidades e aprimorando o design, garantindo que os testes continuem passando

Exemplo Prático (Números Romanos): Para demonstrar a técnica, o autor desenvolve um conversor de numerais romanos para inteiros

- O processo começa com o caso mais simples (o símbolo "I") e evolui gradualmente para cenários mais complexos (como "V", "II" e "IX"), sempre seguindo o ciclo de testar primeiro.

Vantagens da Prática:

- Foco no Comportamento: Ao começar pelo teste, o desenvolvedor foca no que a classe deve fazer (seu contrato e interface) antes de se preocupar com como ela fará.
- Código 100% Testado: Como cada linha de produção só é escrita para satisfazer um teste, o software já nasce com uma alta cobertura de testes.
- Simplicidade: O TDD combate a tendência de criar soluções excessivamente complexas ou "over-engineering", pois incentiva apenas o necessário para o teste passar.
- Feedback Constante: Diferente da abordagem tradicional (testar ao final), o TDD fornece feedback contínuo sobre o design e a funcionalidade, permitindo correções rápidas e de baixo custo.

Ciclo de Feedback:

- TDD:
  - Ciclo 1: [teste + feedback] + [código]
  - Ciclo 2: [teste + feedback] + [código]
  - Ciclo 3: [teste + feedback] + [código]
  - Ciclo 4: [teste + feedback] + [código]
  - Ciclo 5: [teste + feedback] + [código]

- Sem TDD
  - Ciclo 1: [código]
  - Ciclo 2: [código]
  - Ciclo 3: [código]
  - Ciclo 4: [código]
  - Ciclo 5: [teste + feedback] + [código]

    3.5 Um pouco da história de TDD

História e Origens: O capítulo menciona que a prática foi popularizada por Kent Beck no início dos anos 2000, embora a ideia de separar o que o programa faz da sua implementação já existisse anteriormente.

3.6 Conclusão

Pula!

## 4 Simplicidade e Baby Steps

4.1 O Problema do Cálculo de Salário

Mostra o problema: O Problema do Cálculo de Salário: O autor apresenta um cenário onde o salário de um funcionário é calculado com base em seu cargo (Desenvolvedor, DBA ou Testador) e faixas salariais específicas que determinam a porcentagem de desconto.

4.2 Implementando da maneira mais simples possível

Explica como implementar o código de forma simples.

4.3 Passos de Bebê (ou Baby Steps)

Ele complementa a explicação e mostra como simnplificar a solução.

4.4 Usando baby steps de maneira consciente

- Mudança Simples vs. Solução Simples: A modificação mais simples (como adicionar vários ifs e valores fixos) nem sempre leva à solução mais simples ou ao melhor design.
  - O autor alerta que o excesso de condicionais pode tornar o código difícil de manter, mesmo que os testes estejam passando.
- Uso Consciente da Velocidade: O TDD não exige que o desenvolvedor dê passos minúsculos o tempo todo. O autor sugere que o programador deve:
  - Desacelerar e usar Baby Steps quando enfrentar problemas complexos ou quando o design não estiver claro.
  - Acelerar quando a solução for óbvia e ele estiver seguro do que está fazendo, aumentando assim a produtividade.
- Remoção de Duplicidade: O capítulo destaca que o TDD ajuda a identificar lógica duplicada entre o código de teste e o código de produção. Assim que um teste passa, o desenvolvedor deve refatorar para generalizar a solução e eliminar essas repetições.

  4.5 Conclusão

Em suma, o capítulo ensina que a simplicidade deve ser buscada tanto no código quanto no design das classes, e que os Baby Steps são uma ferramenta para aprender sobre o problema à medida que se desenvolve.

## 5 TDD e Design de Classes

5.1 O Problema do Carrinho de Compras

Ele contextualiza o problema e implementa o código.

5.2 Testes que influenciam no design de classes

- A prática de TDD não guia o desenvolvedor para um bom projeto de classes de forma automática; a experiência e conhecimento do desenvolvedor são fundamentais ao criar software orientado a objetos.

  5.3 Diferenças entre TDD e testes da maneira tradicional

Contexto! Não encha o código e fique criando classes como se isso fosse pokemon, evolua a que criou.

5.4 Testes como rascunho

- O teste serve como um rascunho para o desenvolvedor, onde ele pode experimentar as diferentes maneiras de se projetar a classe.

  5.5 Conclusão

Passo a passo:

- A escrita do cenário;
- A execução da ação sob teste;
- A garantia que o comportamento foi executado de acordo com o esperado.

## 6 Qualidade no Código do Teste

6.1 Repetição de código entre testes

- Se a mesma linha se repete várias vezes, organize isso melhor para quando precisar alterar, isso aconteça de forma tranquila.

  6.2 Nomenclatura dos testes

Esse é aquele ponto em que um nome maior e que ajuda a você entender melhor o que está sendo testado, é melhor que um nome menor e que não vai entender, além de que separar as palavras facilita a visualização do nome.

- Os nomes dos métodos de teste devem deixar claro o comportamento esperado:
- Não use isso: deveRetornarZeroSeCarrinhoVazio;
- Use isso: deve_retornar_zero_se_carrinho_vazio;

  6.3 Test Data Builders

- Se usa junit leia!

  6.4 Testes Repetidos

- Mantenha testes que fazem sentido dentro do seu código;
- Atualizou o código? Atualize os teste;
- Retire os testes que dentro da sua aplicação não fazem mais sentido;
- Não repita o óbvio:
  - Em vez de teste: 1 + 1, 2 + 2;
  - Faça: soma de 2 números positivos;

    6.5 Escrevendo boas asserções

- A ordem no assertEquals deve ser sempre (esperado, atual) para que as mensagens de erro do JUnit façam sentido.
- Os testes devem ser curtos e validar apenas uma única responsabilidade;
- `Testar múltiplos comportamentos em um só método torna o feedback confuso e o cenário difícil de montar.

  6.6 Testando listas

- O teste não deve válidar apenas o tamanho da lista, mas também o conteúdo de cada um dos objetos pertencentes à lista.

  6.7 Separando as Classes de Teste

- Recomenda-se criar pastas distintas:
  - Produção: src/main/java
  - Testes: src/test/java
- Manutenção nos mesmos Pacotes: Embora fiquem em pastas físicas diferentes, as classes de teste devem pertencer ao mesmo pacote da classe de produção (ex: ambos no pacote br.com.caelum.leilao);
- Organize bem a ordem de testes.

Privados e Públicos

- Um cliente externo de uma classe só enxerga o que é público:
  - Se o teste precisa acessar as "entranhas" da classe (métodos privados) para garantir que ela funciona, isso significa que a classe não está encapsulando bem sua lógica.

    6.8 Conclusão

Pula!

## 7 TDD e a Coesão

7.1 Novamente o Problema do Cálculo de Salário
7.2 Ouvindo o feedback dos testes
7.3 Testes em métodos privados?
7.4 Resolvendo o Problema da Calculadora de Salário
7.5 O que olhar no teste em relação a coesão?
7.6 Conclusão

## 8 TDD e o Acoplamento

8.1 O Problema da Nota Fiscal
8.2 Mock Objects
8.3 Dependências explícitas
8.4 Ouvindo o feedback dos testes
8.5 Classes estáveis
8.6 Resolvendo o Problema da Nota Fiscal
8.7 Testando métodos estáticos
8.8 TDD e a constante criação de interfaces
8.9 O que olhar no teste em relação ao acoplamento?
8.10 Conclusão

## 9 TDD e o Encapsulamento

9.1 O Problema do Processador de Boleto
9.2 Ouvindo o feedback dos testes
9.3 Tell, Don’t Ask e Lei de Demeter
9.4 Resolvendo o Problema do Processador de Boletos
9.5 O que olhar no teste em relação ao encapsulamento?
9.6 Conclusão

## 10 Testes de Integração e TDD

10.1 Testes de unidade, integração e sistema
10.2 Quando não usar mocks?
10.3 Testes em DAOs
10.4 Devo usar TDD em testes de integração?
10.5 Testes em aplicações Web
10.6 Conclusão

## 11 Quando não usar TDD?

11.1 Quando não praticar TDD?
11.2 100% de cobertura de código?
11.3 Devo testar códigos simples?
11.4 Erros comuns durante a prática de TDD
11.5 Como convencer seu chefe sobre TDD?
11.6 TDD em Sistemas Legados
11.7 Conclusão

## 12 E agora?

12.1 O que ler agora?
12.2 Dificuldade no aprendizado
12.3 Como interagir com outros praticantes?
12.4 Conclusão Final

## 13 Sumário Casa do Código

13 Apêndice: Princípios SOLID
13.1 Sintomas de Projetos de Classes em Degradação
13.2 Princípios de Projeto de Classes
13.3 Conclusão

# Fundamentos da arquitetura de software: uma abordagem de engenharia - Mark Richards e Neal Ford

Eu resolvi começar esse livro de forma diferente do que normalmente eu faço, comecei a leitura pelo capítulo 1, mas o resumo pelo capítulo 2, nada contra, apenas vou e depois eu volto, já que é um assunto que não domino nada. 

## Capítulo 2 - Pensamento Arquitetônico

### Arquitetura Versus Design

Abordar as diferenças entre arquitetura e desenvolvimento é interessante, mas acredito que não é muito difícil definir, pois o arquiteto normalmente está vendo o projeto como um todo e o desenvolvedor atua em uma parcela menor de cada problema.

- Arquiteto: é responsável por coisas como analisar os requisitos comerciais para extrair e definir as características da arquitetura (os “atributos”), selecionar quais padrões e estilos da arquitetura se encaixariam no domínio do problema e criar componentes (blocos de construção do sistema);

- Desenvolvedor: responsável por criar diagramas de classes para cada componente, criar telas de interface do usuário e desenvolver e testar o código-fonte.


### Amplitude Técnica

É interessante pensar no escopo de conhecimentos que você possui numa posição e que difere ao ocupar outra, o exemplo da pirâmide de:

- O que você sabe: tudo aquilo que você domina tecnicamente falando;
- O que você sabe que não sabe: aqui entra tudo que você ouviu, leu, mas não se aprofundou a entender;
- O que você não sabe que não sabe: aqui entra tudo que você realmente não sabe, nunca leu e nunca nem chegou a cogitar ou ouvir falar.

Um desenvolvedor, à parte do que você sabe, é sempre uma parcela muito significativa, pois você está sempre buscando se especializar e seu conhecimento em resolver alguns problemas é tão grande, que essa parcela aumenta bastante.

Porém, na posição de arquiteto, você tem que ter um domínio grande de conhecimento. Porém, a parte relacionada ao que você não sabe está constantemente sendo mexida, pois você tem que estar disposto a ler e entender temas diversos, para ver a possibilidade de aplicar isso na prática.


### Analisando os Trade-offs

A posição de arquiteto é baseada constantemente em escolher algo e implementar algo que, diante do que você possui, é a melhor opção. De forma simples: você está constantemente tentando decidir o que, naquela situação, vai ter a menor chance de dar errado, pois todas as escolhas vão possuir vantagens e desvantagens que devem ser levadas em consideração.

Isso é interessante. Pense em escolher uma linguagem: escolher a mais antiga te dá estabilidade, porém te coloca numa posição em que, em 10 anos, terá menos pessoas com conhecimento para atuar nela. Escolher a mais atual te coloca numa situação de imprevistos: não atender aquela especificidade da melhor forma ou não ter sido testada em determinado uso.

É sempre ver os pontos positivos e, em contrapartida, entender quais pontos negativos terá que lidar.

### Equilibrando Arquitetura e Codificação

Um arquiteto tem que conhecer todo o projeto e, muitas vezes, acaba se afastando do código. Para ter mais contato com o que está sendo desenvolvido, algumas coisas podem ajudar:

- Frequentemente fazer POCs (prova de conceitos): isso ajuda ele a testar ideias e ver o que pode ser implementado com antecedência, além de escrever o melhor código para produção, que pode ser usado como referência para orientar outros a seguirem;

- Trabalhar nas correções de erros dentro de uma iteração: resolver pequenos problemas não só ajuda a equipe, mas coloca o arquiteto em contato direto com o código constantemente;

- Automação: acredito que é uma das coisas que causa o maior impacto em qualquer projeto. Quanto mais automações importantes o projeto possui, menor a possibilidade de cometer erros e menor o tempo gasto fazendo tarefas repetitivas.

## Capítulo 3 - Modularidade





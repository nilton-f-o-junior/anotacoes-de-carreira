# Sumário

## ] 1 Uma Filosofia Pragmática

- [x] 1. O gato comeu meu código-fonte
- [x] 2. Entropia de software
- [x] 3. Sopa de pedras e sapos cozidos
- [x] 4. Software satisfatório
- [x] 5. Sua carteira de conhecimentos
- [x] 6. Comunique-se!

## 2 Uma Abordagem Pragmática

- [x] 7. Os males da duplicação
- [x] 8. Ortogonalidade
- [x] 9. Reversibilidade
- [] 10. Projéteis luminosos
- [] 11. Protótipos e notas post-it
- [] 12. Linguagens de domínio
- [] 13. Estimando

## 3 As Ferramentas Básicas

- [x] 14. O poder do texto simples
- [x] 15. Jogos de shell
- [x] 16. Edição avançada
- [x] 17. Controle do código-fonte
- [x] 18. Depurando
- [x] 19. Manipulação de texto
- [x] 20. Geradores de código

## 4 Paranoia Pragmática

- [] 21. Projeto por contrato
- [] 22. Programas mortos não contam mentiras
- [] 23. Programação assertiva
- [] 24. Quando usar exceções
- [] 25. Como balancear recursos

## 5 Seja Flexível 159

- [] 26. A desvinculação e a Lei de Deméter
- [] 27. Metaprogramação
- [] 28. Vinculação temporal
- [] 29. Apenas um modo de ver
- [] 30. Quadros-negros

## 6 Enquanto Você Está Codificando 193

- [] 31. Programação baseada no acaso
- [] 32. Velocidade do algoritmo
- [] 33. Refatoração
- [] 34. Código que seja fácil de testar
- [] 35. Assistentes do mal

## 7 Antes do Projeto 223

- [] 36. O abismo dos requisitos
- [] 37. Resolvendo problemas impossíveis
- [] 38. Não antes de você estar pronto
- [] 39. A armadilha das especificações
- [] 40. Círculos e setas

## 8 Projetos Pragmáticos

- [] 41. Equipes pragmáticas
- [] 42. Automação onipresente
- [] 43. Testando incansavelmente
- [] 44. Tudo se resume a escrever
- [] 45. Grandes expectativas
- [] 46. Orgulho e preconceito

## A Recursos 283

- [] Sociedades profissionais
- [] Construindo uma biblioteca
- [] Recursos da Internet
- [] Bibliografia

## 1 Uma Filosofia Pragmática 23

### 1 O gato comeu meu código-fonte

- Responsabilidade Profissional
  - Assuma o controle de sua carreira e admita honestamente sua ignorância ou erros cometidos;
  - Comprometa-se com o resultado e ser responsabilize diretamente pelas decisões e ações tomadas.

- Soluções em vez de Desculpas
  - Evite culpar ferramentas ou colegas e foque em apresentar opções e soluções viáveis para resolver os problemas.

- Gestão de Riscos e Contingência
  - Avalie riscos fora de seu controle e tenha planos de reserva, como backups, para evitar falhas que seriam sua responsabilidade;
  - Você tem o direito de não assumir compromissos em situações impossíveis, baseando-se em sua própria ética e julgamento profissional.

- Autocrítica na Comunicação
  - Antes de falar com seu gestor, ouça sua própria justificativa e não apenas uma desculpa tola.
  - Antecipe as perguntas que serão feitas e tente considerar todas as possibilidades de correção antes de dar uma má notícia.

### 2 Entropia de software

- Não Tolere Janelas Quebradas
  - A regra é consertar cada problema assim que descoberto;
  - Ações rápidas, mesmo que temporárias, mostram que você tem a situação sob controle e impedem que a negligência acelere a deterioração do sistema.

- Preservação do Código Limpo
  - Trabalhar em um projeto com código elegante e bem escrito incentiva a equipe a ter um cuidado especial para não ser a primeira a causar danos.

### 3 Sopa de pedras e sapos cozidos

- Sopa de Pedras e o Catalisador
  - Ao mostrar o sucesso de uma pequena parte funcional, as pessoas sentirão mais facilidade em se associar ao progresso e contribuir para o resultado.

- O Perigo do Sapo Cozido
  - Tenha atenção ao crescimento do projeto, aos poucos pequenas features vão inchando o projeto e quando menos se espera ele não termina.

- Visão do Cenário em Larga Escala
  - Olhe para o projeto como um todo e não apenas para as próprias tarefas, isso evita o problema anterior.

### 4 Software satisfatório

- A Ilusão da Perfeição
  - Tentar alcançar a perfeição absoluta pode ser frustrante e prejudicar a produtividade geral da equipe e do projeto.
  - Ignorar prazos de entrega e restrições de fluxo de caixa apenas para refinar excessivamente o código é considerado uma atitude amadora.

- Saber Quando Parar
  - O aprimoramento excessivo e a adição constante de detalhes podem acabar arruinando um programa que já funciona perfeitamente.

### 5 Sua carteira de conhecimentos

- Análise Crítica e Oportunidades
  - Assim que nem você, seu conhecimento envelhece, então busque novos conhecimentos para está se aprimorando;
  - Aprenda uma nova linguagem;
  - Leia livros técnicos e não técnicos;
  - Analise criticamente o que lê e ouve, evitando ser influenciado por dogmas, propagandas de fornecedores ou modismos da mídia especializada.

### 6 Comunique-se!

- Bom senso
  - Conheça as necessidades do seu público para garantir que sua ideia seja bem compreendida e aceita;

  - Escolha o momento ideal para falar, tornando o conteúdo relevante para as prioridades e o estado emocional de quem o ouve;

  - Adapte seu estilo de comunicação, seja formal ou casual, para atender cada destinatário ou grupo de pessoas;

  - Mantenha as pessoas informadas e dê retorno constante, respondendo prontamente a e-mails e mensagens para que ninguém se sinta ignorado;

  - Escute as pessoas, transformar reuniões em diálogos eficientes, encorajando as pessoas a falarem para que você também possa falar.

## 2 Uma Abordagem Pragmática 47

### 7 Os males da duplicação

O Princípio NSR: cada bloco de conhecimento deve ter uma representação única, exclusiva e sem ambiguidades dentro de um sistema para evitar inconsistências. A duplicação é o "pesadelo da manutenção", pois se você altera uma informação em um lugar, deve lembrar de alterá-la em todos os outros.

- Duplicação Imposta: ocorre quando o ambiente, as ferramentas ou as linguagens parecem exigir que a mesma informação seja repetida em múltiplos locais.
  - Para combatê-la, podem-se usar geradores de código ou filtros para derivar representações diferentes a partir de uma única fonte autorizada.

- Duplicação Inadvertida e Impaciente: a inadvertida surge por erros de projeto ou dados não normalizados, enquanto a impaciente vem de "atalhos" tomados sob pressão de prazos. Atalhos como copiar e colar código economizam segundos agora, mas causam grandes atrasos e erros no futuro.

- Duplicação entre Desenvolvedores: acontece quando membros diferentes de uma equipe implementam a mesma funcionalidade de forma isolada por falta de comunicação.
  - A solução envolve encorajar a comunicação ativa, compartilhar conhecimentos e designar um "bibliotecário" do projeto para facilitar a reutilização.

### 8 Ortogonalidade

O Conceito de Independência:

- Sistema ortogonal é quando seus componentes, como o banco de dados e a interface do usuário, são independentes entre si;
- Sistemas não ortogonais são difíceis de mudar, pois cada alteração gera efeitos secundários complexos.

Vantagens:

- Ganho de Produtividade: mudanças localizadas reduzem o tempo de desenvolvimento e teste, permitindo que componentes pequenos e autônomos sejam codificados isoladamente, favorecendo a reutilização e gera mais funcionalidade por esforço unitário ao combinar componentes independentes;

- Redução de Riscos: seções de código danificadas ficam isoladas, diminuindo a probabilidade de uma falha se espalhar por todo o sistema;

- Aplicação em Equipes e Projetos: equipes ortogonais são organizadas com responsabilidades bem definidas e mínima sobreposição, o que reduz a necessidade de reuniões constantes;

- Elimine efeitos entre elementos não relacionados: mantenha seu código desvinculado seguindo a Lei de Deméter e evite o uso de dados globais que criam dependências desnecessárias;

- Ortogonalidade no Teste: sistemas ortogonais são mais fáceis de testar em nível de unidade, pois não exigem que grande parte do resto do sistema seja carregada para validar um módulo;

- Ortogonalidade na documentação: a ortogonalidade é aplicada ao separar o conteúdo da apresentação, permitindo alterar a aparência sem mexer no texto original;

### 9 Reversibilidade

### 10 Projéteis luminosos

### 11 Protótipos e notas post-it

### 12 Linguagens de domínio

### 13 Estimando

## 3 As Ferramentas Básicas 93

Essa parte foi difícil, de verdade nada contra o livro, mas tem momento em que ele escreve texto demais para falar o óbvio, em diversos momentos isso é ótimo, pois fica didático, porém quando isso se repeti diversas versos em diversos momentos em que não é necessário o texto apenas fica cansativo e te faz pegar um longo caminho para chegar num ponto que você chegaria com 3x menos texto.

### 14 O poder do texto simples

- Use texto sempre que possível, melhor doque códigos complexo, um bom texto facilita muito sua vida.

### 15 Jogos de shell

Comece a usar:

- Linux;
- Terminal;
- Automação.

### 16 Edição avançada

- Não complique, use um bom editor de código e aprenda de verdade o que ele pode oferecer;
- Recomendo que tenha um segunda opção, nunca se sabe.

### 17 Controle do código-fonte

- Use git.

### 18 Depurando

- Pula!

### 19 Manipulação de texto

- Pula!

### 20 Geradores de código

Não apenas use IA, seja inteligente! Antes de tudo isso as pessoas sempre buscavam forma de fazer o trabalho repetitivo de formas a resolver o problema acertivamente, não se limite a IA ou a usar ela de uma forma repetitiva sempre.

## 4 Paranoia Pragmática 129

### 21 Projeto por contrato

### 22 Programas mortos não contam mentiras

### 23 Programação assertiva

### 24 Quando usar exceções

### 25 Como balancear recursos

## 5 Seja Flexível 159

### 26 A desvinculação e a Lei de Deméter

### 27 Metaprogramação

### 28 Vinculação temporal

### 29 Apenas um modo de ver

### 30 Quadros-negros

## 6 Enquanto Você Está Codificando 193

### 31 Programação baseada no acaso

### 32 Velocidade do algoritmo

### 33 Refatoração

### 34 Código que seja fácil de testar

### 35 Assistentes do mal

## 7 Antes do Projeto 223

### 36 O abismo dos requisitos

### 37 Resolvendo problemas impossíveis

### 38 Não antes de você estar pronto

### 39 A armadilha das especificações

### 40 Círculos e setas

## 8 Projetos Pragmáticos 245

### 41 Equipes pragmáticas

### 42 Automação onipresente

### 43 Testando incansavelmente

### 44 Tudo se resume a escrever

### 45 Grandes expectativas

### 46 Orgulho e preconceito

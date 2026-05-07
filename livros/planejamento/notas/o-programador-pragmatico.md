# Sumário

## 1 Uma Filosofia Pragmática

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
- [x] 10. Projéteis luminosos
- [x] 11. Protótipos e notas post-it
- [x] 12. Linguagens de domínio
- [x] 13. Estimando

## 3 As Ferramentas Básicas

- [x] 14. O poder do texto simples
- [x] 15. Jogos de shell
- [x] 16. Edição avançada
- [x] 17. Controle do código-fonte
- [x] 18. Depurando
- [x] 19. Manipulação de texto
- [x] 20. Geradores de código

## 4 Paranoia Pragmática

- [x] 21. Projeto por contrato
- [x] 22. Programas mortos não contam mentiras
- [x] 23. Programação assertiva
- [x] 24. Quando usar exceções
- [x] 25. Como balancear recursos

## 5 Seja Flexível

- [x] 26. A desvinculação e a Lei de Deméter
- [x] 27. Metaprogramação
- [x] 28. Vinculação temporal
- [x] 29. Apenas um modo de ver
- [x] 30. Quadros-negros

## 6 Enquanto Você Está Codificando

- [x] 31. Programação baseada no acaso
- [x] 32. Velocidade do algoritmo
- [x] 33. Refatoração
- [x] 34. Código que seja fácil de testar
- [x] 35. Assistentes do mal

## 7 Antes do Projeto

- [x] 36. O abismo dos requisitos
- [x] 37. Resolvendo problemas impossíveis
- [x] 38. Não antes de você estar pronto
- [x] 39. A armadilha das especificações
- [x] 40. Círculos e setas

## 8 Projetos Pragmáticos

- [x] 41. Equipes pragmáticas
- [x] 42. Automação onipresente
- [x] 43. Testando incansavelmente
- [x] 44. Tudo se resume a escrever
- [x] 45. Grandes expectativas
- [x] 46. Orgulho e preconceito

## 1 Uma Filosofia Pragmática

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

- Bom senso: 
  - Conheça as necessidades do seu público para garantir que sua ideia seja bem compreendida e aceita;
  - Escolha o momento ideal para falar, tornando o conteúdo relevante para as prioridades e o estado emocional de quem o ouve;
  - Adapte seu estilo de comunicação, seja formal ou casual, para atender cada destinatário ou grupo de pessoas;
  - Mantenha as pessoas informadas e dê retorno constante, respondendo prontamente a e-mails e mensagens para que ninguém se sinta ignorado;
  - Escute as pessoas, transformar reuniões em diálogos eficientes, encorajando as pessoas a falarem para que você também possa falar.

## 2 Uma Abordagem Pragmática

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

- Incerteza e Mudança: o mundo real é imprevisível e fatos que parecem certos hoje, como a escolha de um banco de dados, podem mudar amanhã.

- Arquitetura Flexível e Desvinculada: mantenha a flexibilidade isolando partes do projeto de mudanças de plataforma ou fornecedor através de interfaces abstratas.
  - Seguir princípios como o NSR (Não Se Repita) e a desvinculação permite trocar componentes críticos sem que o custo seja proibitivo.

### 10 Projéteis luminosos

- Pula! Leia ...

### 11 Protótipos e notas post-it

- Objetivo: protótipos servem para analisar riscos e testar ideias críticas de forma barata e rápida antes do desenvolvimento real;

- Valor do Protótipo: o valor real reside nas lições aprendidas durante o processo e não no código produzido, que deve ser considerado descartável;

- Diversidade de Materiais: protótipos não precisam ser apenas código, notas post-it são ideais para fluxos de trabalho, e quadros brancos servem para interfaces; se for código, prefira linguagens de alto nível para ignorar detalhes triviais e focar apenas no que está sendo investigado;

- Detalhes a Ignorar: ignore precisão (use dados fictícios), completude (funcionalidade limitada) e robustez (verificação mínima de erros). Sacrifique também o estilo de codificação e a documentação profunda para acelerar a resposta a perguntas específicas do projeto;

- Gerenciamento de Expectativas: deve-se deixar claro para todos que o protótipo é código descartável e incompleto para evitar que a gerência tente implantá-lo prematuramente.

### 12 Linguagens de domínio

- Pula! Leia ...

### 13 Estimando

- Estime o tempo em dias ou semanas, isso ajuda a dar uma perspectiva melhor para o problema. Um bom cronograma pode evitar certas dores de cabeça, mas ajuda a mensurar.

## 3 As Ferramentas Básicas

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

## 4 Paranoia Pragmática

### 21 Projeto por contrato

- Defina direitos e responsabilidades entre módulos através de pré-condições, pós-condições e invariantes de classe. Essa técnica garante que o programa seja preciso, facilitando a documentação e a verificação automática de que o código cumpre o que promete.

### 22 Programas mortos não contam mentiras

- É preferível encerrar um programa imediatamente ao detectar um erro "impossível" do que permitir que ele continue operando com dados corrompidos;
- Encerrar precocemente evita danos maiores, como a gravação de informações inválidas em sistemas ou bancos de dados.

### 23 Programação assertiva

- Use asserções para verificar ativamente suposições do tipo "isso nunca vai acontecer", protegendo o código de estados e dados inválidos;
- Mantenha as asserções ativadas mesmo em ambiente de produção para detectar erros reais que os testes podem não ter capturado.

### 24 Quando usar exceções

- Reserve o uso de exceções apenas para eventos verdadeiramente inesperados e excepcionais, evitando usá-las como parte do fluxo normal do programa;
- O uso excessivo de exceções pode comprometer a legibilidade e o encapsulamento.

### 25 Como balancear recursos

- Siga a regra de "acabar o que começou": a rotina ou objeto que aloca um recurso (memória, arquivos, transações) deve ser o responsável por desalocá-lo;
- Desaloque recursos na ordem inversa da alocação e utilize mecanismos da linguagem, como blocos finally ou classes encapsuladoras.

## 5 Seja Flexível

### 26 A desvinculação e a Lei de Deméter

- Redução da Vinculação: organize o código em módulos independentes (como células) para que a substituição de uma parte não comprometa o sistema todo. Evite percorrer longas hierarquias de objetos para obter serviços, reduzindo o risco de mudanças em terceiros afetarem seu código;

- A Lei de Deméter: restrinja as chamadas de métodos apenas ao próprio objeto, seus parâmetros, objetos criados por ele ou seus componentes diretos. Embora possa exigir métodos encapsuladores extras, essa prática torna o sistema muito mais adaptável, robusto e fácil de manter.

### 27 Metaprogramação

- Configuração Dinâmica: mova detalhes como algoritmos, bancos de dados e preferências para metadados, configurando o sistema em vez de integrá-lo rigidamente. O uso de metadados permite alterar o comportamento do aplicativo em tempo de execução sem a necessidade de recompilar o código.

- Abstração e Detalhes: mantenha a lógica abstrata no código e os detalhes voláteis (como regras de negócio) em arquivos externos ou bancos de dados de configuração. Essa separação força a desvinculação do projeto e cria programas "leves" que se adaptam rapidamente a mudanças de requisitos ou ambiente.

### 28 Vinculação temporal

- Concorrência e Ordem: o tempo é um elemento de projeto que envolve concorrência e a ordem relativa das ações no sistema. Desvincular a dependência de tempo permite que as operações ocorram em paralelo, aumentando a flexibilidade e a escalabilidade;

Análise de Fluxo de Trabalho: use diagramas de atividades para identificar tarefas que podem ser executadas simultaneamente em vez de linearmente. Ao maximizar o paralelismo no fluxo de trabalho, você reduz gargalos e melhora o desempenho geral do sistema;

- Projeto para Concorrência: projete interfaces e serviços pensando em ambientes com vários segmentos, garantindo estados válidos em qualquer momento. Evitar suposições de tempo linear resulta em códigos mais limpos, robustos e fáceis de adaptar para diferentes modelos de implantação.

### 29 Apenas um modo de ver

- Publicação e Assinatura: se eventos para permitir que objetos se comuniquem sem conhecer detalhes internos uns dos outros, reduzindo a vinculação. O protocolo de publicação/assinatura garante que os ouvintes recebam apenas as notificações de interesse, mantendo o encapsulamento;

- Model-View-Controller: separe o modelo de dados de sua representação visual para permitir múltiplas visualizações simultâneas e independentes. Essa desvinculação facilita a manutenção, permitindo alterar a interface ou o controle sem afetar a lógica de negócio subjacente.

### 30 Quadros-negros

- Desvinculação e Anonimato: sistemas de quadro-negro funcionam como um ponto de encontro onde agentes trocam dados de forma anônima e assíncrona. Os participantes não precisam saber da existência uns dos outros, permitindo que colaborem em um problema comum;

- Coordenação de Fluxo de Trabalho: use quadros-negros para gerenciar processos complexos onde a ordem de chegada dos dados é imprevisível ou distribuída. Essa abordagem substitui fluxos rígidos por um mecanismo de regras dinâmico que se adapta conforme novos fatos são publicados.

## 6 Enquanto Você Está Codificando

### 31 Programação baseada no acaso

- Programação Deliberada: esteja sempre consciente do que está fazendo, agindo de acordo com um plano definido e confiando apenas em elementos comprovadamente confiáveis. Teste ativamente suas suposições e documente-as para garantir que o sucesso do seu código seja fruto de um projeto intencional e não do acaso.

### 32 Velocidade do algoritmo

- Notação do Grande: utilize a notação O() para estimar como o tempo de execução e o consumo de memória aumentam conforme o tamanho dos dados de entrada cresce. Compreender a ordem de complexidade ajuda a identificar gargalos potenciais e a escolher o algoritmo mais adequado para as restrições do seu sistema;

- Bases Teóricas e Práticas: combine a análise matemática com testes práticos no ambiente real, pois fatores físicos como memória disponível e cache impactam o desempenho final. Use geradores de perfil (profilers) para validar suas estimativas teóricas e evite desperdiçar tempo com otimizações prematuras em partes não críticas.

### 33 Refatoração

- Metáfora da Jardinagem: software é mais parecido com um jardim do que com uma construção civil, exigindo ajustes orgânicos e constantes conforme o entendimento do problema evolui. Refatorar significa reorganizar o código existente para eliminar duplicações, melhorar a ortogonalidade ou adaptar-se a novos requisitos sem alterar o comportamento externo;

- Refatoração Segura e Constante: adote a prática de "refatorar cedo e sempre", tratando o código degradado como um "tumor" que deve ser removido antes que se espalhe e comprometa o projeto. Para refatorar com segurança, realize mudanças em etapas curtas e deliberadas, mantendo uma suíte sólida de testes de regressão para validar cada passo.

### 34 Código que seja fácil de testar

- Teste de Unidade e Contratos: o teste de unidade verifica se um módulo honra seu contrato, testando funcionalidades e condições limítrofes de forma isolada. Ao testar em relação ao contrato, garante-se que o código atenda aos requisitos e que o contrato seja bem compreendido;

- Ferramentais e Janelas de Teste: utilize ferramentais de teste (test harnesses) padronizados e automáticos que permitam analisar saídas e compor testes de diversos níveis. Crie "janelas" para visualizar o estado interno do software em produção, como arquivos de log e servidores Web embutidos.

### 35 Assistentes do mal

- O Risco da Programação Cega: usar assistentes sem entender o código gerado é uma forma de programação baseada no acaso, onde o sucesso é apenas acidental. Se o código gerado não estiver totalmente correto ou precisar de adaptação futura, o desenvolvedor perderá o controle sobre o próprio aplicativo;

## 7 Antes do Projeto

### 36 O abismo dos requisitos

- O objetivo é descobrir a necessidade real do negócio, focando no que precisa ser feito e não apenas em como as tarefas são executadas atualmente;

- Requisitos são declarações gerais e estáveis, enquanto políticas são regras de negócio específicas e voláteis que mudam com frequência. Documentar políticas separadamente ou tratá-las como metadados permite que o sistema seja muito mais flexível e adaptável a mudanças futuras;

- Utilize casos de uso para descrever objetivos específicos em formato textual, facilitando a comunicação entre patrocinadores, usuários e desenvolvedores;

- Mantenha os requisitos abstratos para evitar a armadilha da especificação excessiva, focando nas necessidades e não na arquitetura ou implementação.

Exemplo:

“só o departamento de pessoal pode ver um registro de funcionário”, o desenvolvedor pode acabar codificando um teste explícito sempre que o aplicativo acessar esses arquivos. No entanto, se a declaração for “só usuários autorizados podem acessar um registro de funcionário”, provavelmente o desenvolvedor projetará e implementará algum tipo de sistema de controle de acesso.

### 37 Resolvendo problemas impossíveis

- Esqueça tudo que o livro diz, guarde apenas essa dica: divida o código na metade e teste ambas as partes, achou a metade com erro? Divida outra vez na metade e teste outra vez! Dividir para consquistar. 

### 38 Não antes de você estar pronto

- Crie protótipos.

### 39 A armadilha das especificações

- Algumas coisas são fáceis de fazer, mas não de descrever. Evite o excesso de prescrição, pois a especificação deve guiar e não eliminar a arte da programação.

### 40 Círculos e setas

- Não seja escravo dos métodos formais. Diagramas de "círculos e setas" representam apenas a interpretação dos projetistas e não devem ser adotados
cegamente;

- Ferramentas caras não produzem projetos melhores. O programador deve extrair o melhor de cada método e adaptá-lo às necessidades reais de sua equipe.
## 8 Projetos Pragmáticos

### 41 Equipes pragmáticas

- Tenha uma boa liderança;
- Divida bem a equipe de forma que uma não atrapalhe o trabalho da outra;
- Deixe pessoas responsáveis por partes importantes como documentação e testes (fale com ...).

### 42 Automação onipresente

- Automatize as tarefas repetidas, seja um agendamento de reunião ou o backup de um projeto.

### 43 Testando incansavelmente

- O texto se extende bastante para falar que você deve testar seu software durante o processo de desenvolvimento. Não espere finalizar o projeto inteiro para poder testar, a medida que vai escrevendo código, vá também criando testes unitários e de integração, assim você tem a garantia que ele foi testado o máximo possível.

> Eu recomendo que você leia um livro sobre testes.

### 44 Tudo se resume a escrever

- Documentação é importante e deve ser feita de forma a ajudar você e outros desenvolvedores a entender o que foi feito;
- Trate o idioma natural como se fosse outra linguagem de programação, aplicando conceitos como NSR, metadados e automação;
- Construa a documentação diretamente no código em vez de deixá-la como um complemento externo propenso a desatualizações;
- Os comentários devem focar no "porquê" de algo ser feito e em sua finalidade, já que o código já demonstra "como" é feito.
- Utilize nomes de variáveis significativos e evite ambiguidades, lembrando que o código será lido muito mais vezes do que escrito;
- Mantenha uma única fonte de informações autorizada para gerar automaticamente esquemas de banco de dados, códigos e especificações;
- Prefira a publicação na Web e o uso de hiperlinks para garantir que a documentação permaneça atualizada e acessível a todos.

### 45 Grandes expectativas

- Tenha controle sobre seu projeto e entregue aquilo que foi definido;
- Não crie um projeto que não pode entregar (seja realista).

### 46 Orgulho e preconceito

- Faça um código bem escrito;
- Assuma a responábilidade sobre o que escreveu;
- Respeite o código dos outros, se vai dar opnião, saiba como o fazer;

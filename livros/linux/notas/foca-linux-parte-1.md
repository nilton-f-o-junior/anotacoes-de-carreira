# Introdução

- Cápitulo 1. Introdução
- Capítulo 2. Explicações Básicas


## Cápitulo 1. Introdução

### Sistema Operacional

- Definição: conjunto de programas que funcionam como a interface entre o usuário (e seus programas) e o computador;

- Linux: o sistema operacional é composto pelo Kernel (sua base principal) somado a um conjunto de ferramentas GNU;

- Criação: 1991 por Linus Torvalds na Finlândia, o Linux é um sistema operacional de código aberto distribuído gratuitamente;

- Software Livre: ele é licenciado sob a GPLv2, o que permite que qualquer pessoa o use sem pagar nada e incentiva a realização de cópias para instalação em outros computadores;

- Padrão POSIX: segue o padrão POSIX, o mesmo utilizado por sistemas UNIX e suas variantes, o que facilita a operação de outros sistemas similares;

- Desenvolvimento Comunitário: é mantido e desenvolvido por milhares de pessoas ao redor do mundo, o que contribui para seu rápido crescimento, estabilidade e ampla compatibilidade com periféricos novos e antigos.

### Características do Linux

- Multitarefa e Multiusuário Real: permite a execução de vários programas e o acesso de múltiplos usuários simultaneamente;

- Modularização: o sistema carrega para a memória apenas o que está sendo usado, liberando-a assim que o programa ou dispositivo é finalizado;

- Segurança (Hardening): possui mecanismos avançados contra vírus e malwares, tornando-os muitas vezes inúteis devido à separação de privilégios entre processos e restrições de acesso ao sistema de arquivos;

- Escalabilidade: funciona desde dispositivos extremamente simples (como Android, Raspberry Pi e geladeiras inteligentes) até gigantescos clusters em nuvem;

- Conectividade: possui suporte nativo a redes TCP/IP (sendo frequentemente mais rápido que o Windows nesse aspecto) e conecta-se com diversas plataformas como Apple, Windows e Unix;

- Flexibilidade de Interface: embora possua interface gráfica, seus melhores recursos e maior flexibilidade são encontrados na linha de comando;

- Sistemas de Arquivos Inteligentes: utiliza formatos (como Ext2, Ext3, etc.) que organizam os arquivos de forma a evitar a fragmentação;

- O Linux permite a montagem de servidores de publicação Web (Apache) e de E-mail (Sendmail) com baixo custo e alta performance.

  - Esses aplicativos são distribuídos gratuitamente com a maioria das distribuições, sendo o Apache o servidor Web mais usado no mundo e o Sendmail parte da base de 72% dos servidores de e-mail atuais,.


### Distribuições Linux

- Debian: desenvolvida inteiramente por voluntários, é focada em estabilidade e segurança;

- Slackware: criada em 1993, é a distribuição mais antiga em atividade. Ela preza pela simplicidade técnica e por ser o mais parecida possível com o sistema UNIX original;

- Red Hat Enterprise: focada no mercado corporativo e em servidores de grandes empresas, sendo uma distribuição paga e com suporte especializado;

- NixOS: é única por seu gerenciamento de configuração declarativo e puramente funcional, permitindo que todo o sistema seja descrito em um arquivo de configuração e possibilitando reversões (rollbacks) atômicas caso algo falhe.


### Software Livre

O termo Software Livre refere-se à liberdade dos usuários de executar, distribuir, estudar, mudar e melhorar o software, não estando relacionado ao preço. O Linux é um kernel livre que, ao ser combinado com o sistema GNU, resultou em uma variante funcional frequentemente denominada GNU/Linux.

### Processamento de Dados

O Processamento de Dados consiste no envio de informações ao computador, as quais são processadas para gerar um resultado de saída que seja útil ao utilizador.

### Computador

O computador é uma máquina eletrônica projetada para processar e armazenar dados, executando programas para atender às necessidades do usuário.

#### Alguns componentes da placa mãe

A placa mãe é a placa principal do sistema e contém componentes essenciais como:

- Processador: responsável pelo processamento das instruções matemáticas e lógicas;
- RAM: memória de armazenamento temporário e rápida onde os programas são executados;
- Cache: memória auxiliar de alta velocidade que serve para aumentar o desempenho do processamento;
- BIOS: memória ROM que contém as instruções básicas para a inicialização e reconhecimento de periféricos;
- CMOS: Memória temporária alimentada por bateria que armazena as configurações de hardware.

#### Memória

A memória do computador é dividida em duas categorias principais:

- Memória Principal (RAM): é eletrônica, muito rápida e volátil, o que significa que perde os dados quando a energia é desligada;
- Memória Auxiliar: não depende de energia para manter os dados e é usada para evitar a perda de informações ao desligar a máquina.

#### Discos

Os discos são dispositivos de armazenamento de memória auxiliar:

- Disco Rígido (HD): localizado internamente, possui alta capacidade e velocidade no acesso aos dados;
- SSD (Disco de Estado Sólido): armazena dados em chips eletrônicos em vez de discos magnéticos, oferecendo alta performance.

#### Dispositivos de entrada e saída

- Entrada: são dispositivos que enviam dados ao computador para processamento, como o teclado (stdin padrão no Linux), mouse e scanner;
- Saída: permitem que o usuário visualize os resultados do processamento, como o monitor (stdout padrão no Linux), impressoras e som.


## Capítulo 2. Explicações Básicas

### Básico

- Hardware: representa a parte física do computador, como a placa mãe e discos.
- Software: são os programas e sistemas utilizados, como o próprio Linux.
- Arquivos: são locais onde dados como textos e músicas são gravados, sendo que cada um deve possuir um nome para identificação.

No Linux, os arquivos são Case Sensitive, o que significa que o sistema diferencia letras maiúsculas de minúsculas.

### Extensão de arquivos

A extensão serve para identificar o tipo do arquivo. A extensão são as letras após um "." no nome de um arquivo, explicando melhor:

- relatório.txt - O .txt indica que o conteúdo é um arquivo texto;
- script.sh - Arquivo de Script (interpretado por /bin/sh);
- system.log - Registro de algum programa no sistema;
- arquivo.gz - Arquivo compactado pelo utilitário gzip;
- index.html - Página de Internet (formato Hypertexto).


### Tamanho de arquivos

A unidade de medida padrão nos computadores é o bit. A um conjunto de 8 bits nós chamamos de byte.

O Tamanho de arquivos é medido em bytes, onde cada byte equivale a um caractere. Para facilitar a leitura de grandes volumes, utilizam-se múltiplos como Kbytes (1024 bytes).

### Diretório

Diretório é o local utilizado para armazenar conjuntos arquivos para melhor organização e localização. O diretório, como o arquivo, também é "Case Sensitive" (diretório /teste é completamente diferente do diretório /Teste).

A estrutura de diretórios também é chamada de Árvore de Diretórios porque é parecida com uma árvore de cabeça para baixo. Cada diretório do sistema tem seus respectivos arquivos que são armazenados conforme regras definidas pela FHS (FileSystem Hierarchy Standard - Hierarquia Padrão do Sistema de Arquivos).

### Estrutura básica de diretórios do Sistema Linux

O sistema GNU/Linux possui a seguinte estrutura básica de diretórios organizados segundo o FHS (Filesystem Hierarchy Standard):

- /bin: contém arquivos programas do sistema que são usados com freqüência pelos usuários;

- /boot: contém arquivos necessários para a inicialização do sistema;

- /cdrom: ponto de montagem da unidade de CD-ROM;

- /media: ponto de montagem de dispositivos diversos do sistema (rede, pen-drives, CD-ROM em distribuições mais novas);

- /dev: contém arquivos usados para acessar dispositivos (periféricos) existentes no computador;

- /etc: arquivos de configuração de seu computador local;

- /floppy: ponto de montagem de unidade de disquetes;

- /home: diretórios contendo os arquivos dos usuários;

- /lib: bibliotecas compartilhadas pelos programas do sistema e módulos do kernel;

- /lost+found: local para a gravação de arquivos/diretórios recuperados pelo utilitário fsck.ext2. Cada partição possui seu próprio diretório lost+found;

- /mnt: ponto de montagem temporário;

- /proc: sistema de arquivos do kernel. Este diretório não existe em seu disco rígido, ele é colocado lá pelo kernel e usado por diversos programas que fazem sua leitura, verificam configurações do sistema ou modificar o funcionamento de dispositivos do sistema através da alteração em seus arquivos;

- /sys: sistema de arquivos do kernel. Este diretório não existe em seu disco rígido, ele é colocado lá pelo kernel e usado por diversos programas que fazem sua leitura, verificam configurações do sistema ou modificar o funcionamento de dispositivos do sistema através da alteração em seus arquivos;

- /root: diretório do usuário root;

- /sbin: diretório de programas usados pelo superusuário (root) para administração e controle do funcionamento do sistema;

- /tmp: diretório para armazenamento de arquivos temporários criados por programas;

- /usr: contém maior parte de seus programas. Normalmente acessível somente como leitura;

- /var: contém maior parte dos arquivos que são gravados com freqüência pelos programas do sistema, e-mails, spool de impressora, cache, etc.

### Comandos

Comandos são ordens passadas ao sistema operacional para a execução de tarefas específicas, são separados de suas opções (que controlam como o comando será executado) e de seus parâmetros (que indicam caminhos, origens ou destinos) por um espaço. Caso o nome do comando seja digitado incorretamente, o sistema exibirá a mensagem command not found.

- Comandos Internos: estão localizados dentro do interpretador de comandos e são carregados na memória RAM junto com ele (ex: cd, echo, exit);
- Comandos Externos: estão localizados no disco e são procurados através do caminho definido no PATH.

### Interpretador de comandos (Shell)

O Interpretador de comandos, ou shell, é o programa que faz a ligação principal entre o usuário e o Kernel, interpretando as instruções enviadas pelo teclado ou por arquivos executáveis. O Bash é o shell mais utilizado no Linux.

- Interativo: o usuário digita comandos um a um no prompt;
- Não-interativo: o computador executa sequências de comandos contidas em arquivos chamados scripts. 

### Aviso de comando (Prompt)

O Aviso de comando, ou Prompt, é a linha onde os comandos são digitados para serem processados (traço piscante).  

- \# (Tralha): identifica o aviso de comando do superusuário (root).
- $ (Cifrão): identifica o aviso de comando de usuários comuns. 

### Terminal Virtual (console)

Linux permite o uso de múltiplos terminais virtuais, que são seções de trabalho independentes.

Modo texto: utiliza-se ALT + F1 a F6 para alternar entre os seis terminais iniciais;
Modo gráfico: deve-se usar CTRL + ALT + F1 a F6 para ir ao modo texto e CTRL + ALT + F7 para retornar aos gráficos.

### Coringas

Coringas (ou referência global) são recursos para especificar diversos arquivos ou diretórios de uma só vez, facilitando filtragens, cópias e exclusões. Existem 4 tipos principais:

- * (Asterisco): faz referência ao nome completo ou ao restante de um nome;
- ? (Interrogação): substitui um único caractere naquela posição específica;
- [padrão]: referencia uma faixa ou intervalo de caracteres (ex: [a-z], ). Se precedido por ^, exclui aqueles caracteres;
- {padrões}: expande e gera strings para pesquisa, sendo que a existência do arquivo é opcional (muito útil para criar diretórios).

Esses coringas podem ser utilizados em conjunto para criar filtragens muito exatas.

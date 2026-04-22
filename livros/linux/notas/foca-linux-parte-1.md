# Introdução

- Cápitulo 1. Introdução
- Capítulo 2. Explicações Básicas
- Capítulo 3. Para quem esta migrando (ou pensando em migrar) do DOS/Windows para o Linux
- Capítulo 4. Discos e Partições
- Capítulo 5. Execução de programas

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

```
/                          # Raiz do sistema de arquivos — tudo parte daqui
├── bin/                   # Binários essenciais de uso geral (ls, cp, bash…)
├── boot/                  # Arquivos de inicialização do sistema (kernel, grub…)
├── cdrom/                 # Ponto de montagem da unidade de CD-ROM (legado)
├── dev/                   # Arquivos de dispositivos/periféricos (hd, usb, tty…)
├── etc/                   # Arquivos de configuração locais do sistema
├── floppy/                # Ponto de montagem de disquetes (legado)
├── home/                  # Diretórios pessoais dos usuários (/home/usuario)
├── lib/                   # Bibliotecas compartilhadas e módulos do kernel
├── lost+found/            # Arquivos recuperados pelo fsck após falhas de disco
├── media/                 # Montagem automática de mídias removíveis (pen-drive, CD…)
├── mnt/                   # Ponto de montagem temporário para uso manual
├── proc/                  # Sistema de arquivos virtual do kernel (não existe no disco)
├── root/                  # Diretório pessoal do superusuário (root)
├── sbin/                  # Binários de administração do sistema (somente root)
├── sys/                   # Sistema de arquivos virtual do kernel (dispositivos e drivers)
├── tmp/                   # Arquivos temporários criados por programas (limpo no boot)
├── usr/                   # Programas e dados de uso geral (geralmente somente leitura)
└── var/                   # Dados variáveis: logs, e-mails, spool, cache…
```

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

| Coringa | Nome | Descrição | Exemplo |
|---------|------|-----------|---------|
| `*` | Asterisco | Faz referência ao nome completo ou ao restante de um nome | `*.txt` → todos os arquivos `.txt` |
| `?` | Interrogação | Substitui exatamente um único caractere na posição indicada | `foto?.jpg` → `foto1.jpg`, `fotoA.jpg` |
| `[padrão]` | Colchetes | Referencia uma faixa ou conjunto de caracteres; prefixado com `^` exclui os caracteres listados | `[a-z]*` → arquivos que começam com letra minúscula |
| `{padrões}` | Chaves | Expande e gera múltiplas strings para pesquisa ou criação (a existência do arquivo é opcional) | `{src,bin,lib}/` → cria/referencia três diretórios de uma vez |

Esses coringas podem ser utilizados em conjunto para criar filtragens muito exatas.


## Capítulo 3. Para quem esta migrando (ou pensando em migrar) do DOS/Windows para o Linux

- Ler!

## Capítulo 4. Discos e Partições

- Leia!

## Capítulo 5. Execução de programas

### path

Path é o caminho de procura dos arquivos/comandos executáveis. O path (caminho) é armazenado na variável de ambiente PATH. Você pode ver o conteúdo desta variável com o comando echo $PATH.

Caso o interpretador de comandos chegue até o último diretório do path e não encontre o arquivo/comando digitado, é mostrada a seguinte mensagem:

```bash
pd # command not found (comando não encontrado).
```
Se deseja alterar o path para um único usuário, modifique o arquivo .bash_profile em seu diretório de usuário (home).

### Tipos de Execução de Comandos/Programas

- Primeiro Plano (Foreground): é o modo padrão, onde o usuário deve esperar o término da execução do programa para poder digitar um novo comando no prompt;
  - Para iniciar um programa em primeiro plano, basta digitar seu nome normalmente.

- Segundo Plano (Background): permite que o sistema fique livre para outras tarefas enquanto o programa roda internamente.
  - Para iniciar um programa em segundo plano, acrescente o caracter "&" após o final do comando.

### Executando Programas em Seqüência

Os comandos podem ser executados em seqüência (um após o término do outro) se os separarmos com ";". Por exemplo: echo primeiro;echo segundo;echo terceiro.

#### ps

Processos estão sendo executados no computador e também nos mostra qual usuário executou o programa, hora que o processo foi iniciado, etc.

ps \[opções]

| Opção | Descrição |
|-------|-----------|
| `a` | Mostra os processos criados por você e por outros usuários do sistema |
| `x` | Mostra processos que não são controlados pelo terminal |
| `u` | Exibe o nome do usuário que iniciou o processo e a hora de início |
| `m` | Mostra a memória ocupada por cada processo em execução |
| `f` | Exibe a árvore de execução de comandos (comandos chamados por outros comandos) |
| `e` | Mostra variáveis de ambiente no momento da inicialização do processo |
| `w` | Continua a linha atual na próxima linha ao invés de cortar o que não couber na tela |
| `--sort:[coluna]` | Organiza a saída por coluna. Colunas disponíveis: `pid`, `utime`, `ppid`, `rss`, `size`, `user`, `priority` |

#### top

O comando top é uma ferramenta utilizada para monitorar em tempo real os processos que estão sendo executados no computador e os recursos que eles utilizam.

top \[opções]

| Opção | Descrição |
|-------|-----------|
| `-d [tempo]` | Atualiza a tela após o intervalo de `[tempo]` segundos |
| `-s` | Executa o top em modo seguro |
| `-i` | Inicia o top ignorando o tempo de processos zumbis |
| `-c` | Mostra a linha de comando completa ao invés do nome do programa |

Manual do comando pode ser obtida dentro do programa pressionando a tecla h ou man top:

| Tecla | Ação |
|-------|------|
| `Espaço` | Atualiza imediatamente a tela |
| `CTRL+L` | Apaga e atualiza a tela |
| `h` | Exibe a tela de ajuda com todas as teclas disponíveis |
| `i` | Ignora o tempo ocioso de processos zumbis |
| `q` | Sai do programa |
| `k` | Finaliza um processo (similar ao `kill`); solicita o PID — indisponível com a opção `-s` |
| `n` | Muda o número de linhas mostradas na tela (0 = usa toda a tela) |

### Controle de execução de processos

- Interrompendo a execução de um processo: CTRL + C;
- Parando momentaneamente a execução de um processo: CTRL + Z | Retorna usando "fg" ou "bg";
- jobs: processos que estão parados ou rodando em segundo plano;
- fg: faz um programa rodando em segundo plano ou parado, rodar em primeiro plano. Use o comando jobs para pegar o número do processo: fg \[número];
- bg: faz um programa rodando em primeiro plano ou parado, rodar em segundo plano. Execução do comando com CTRL+ Z, será mostrado o número da tarefa interrompida: bg \[número]

#### kill

Enviará um sinal de término ao processo sendo executado.

kill \[opções] \[sinal] \[número]

- número: o número de identificação obtido com o comando “ps”. Pode ser o número após o sinal de % obtido pelo comando jobs;
- sinal: sinal que será enviado ao processo. Se omitido usa -15 como padrão;
- opções, -9: envia um sinal de destruição ao processo ou programa. Ele é terminado imediatamente sem chances de salvar os dados ou apagar os arquivos temporários criados por ele;

Você precisa ser o dono do processo ou o usuário root para termina-lo ou destruí-lo. Você pode verificar se o processo foi finalizado através do comando ps. Os tipos de sinais aceitos:

- Exemplo: kill 500, kill -9 500, kill %1.

> Nota: Killall e killall5 - Leia!

### Eliminando caracteres estranhos

- reset: comando pode ser usado para lidar com programas que não estão funcionando direito, muito comum em scripts mal configurados;

## Capítulo 6. Comandos para manipulação de diretório

#### ls

Lista os arquivos de um diretório.

ls \[opções] \[caminho/arquivo] \[caminho1/arquivo1]

- caminho/arquivo: diretório/arquivo que será listado;
- caminho1/arquivo1: Outro Diretório/arquivo que será listado. Podem ser feitas várias listagens de uma só vez.
- -a, --all: Lista todos os arquivos (inclusive os ocultos).
- -A, --almost-all: Lista todos os arquivos (inclusive os ocultos) de um diretório, exceto o diretório atual e o de nível anterior.
- -B, --ignore-backups: Não lista arquivos que terminam com ~ (Backup).
- --color=PARAM: Mostra os arquivos em cores diferentes, conforme o tipo de arquivo. PARAM pode ser:
  - never: Nunca lista em cores (padrão).
  - always: Sempre lista em cores conforme o tipo de arquivo.
  - auto: Somente colore a listagem se estiver em um terminal.
- -d, --directory: Lista os nomes dos diretórios ao invés do conteúdo.
- -f: Não classifica a listagem.
- -F: Insere um caracter após arquivos executáveis ('*'), diretórios ('/'), soquete ('='), link simbólico ('@') e pipe ('|'). Seu uso é útil para identificar de forma fácil tipos de arquivos nas listagens de diretórios.
- -G, --no-group: Oculta a coluna de grupo do arquivo.
- -h, --human-readable: Mostra o tamanho dos arquivos em Kbytes, Mbytes, Gbytes.
- -H: Faz o mesmo que -h, mas usa unidades de 1000 ao invés de 1024 para especificar Kbytes, Mbytes, Gbytes.
- -l: Usa o formato longo para listagem de arquivos. Lista as permissões, data de modificação, donos, grupos, etc.
- -n: Usa a identificação de usuário e grupo numérica ao invés dos nomes.
- -L, --dereference: Lista o arquivo original e não o link referente ao arquivo.
- -o: Usa a listagem longa sem os donos dos arquivos (mesma coisa que -lG).
- -p: Mesma coisa que -F, mas não inclui o símbolo '*' em arquivos executáveis. Esta opção é típica de sistemas Linux.
- -R: Lista diretórios e sub-diretórios recursivamente.
- --full-time: Lista data e hora completa.

Classificação da listagem A listagem pode ser classificada usando-se as seguintes opções:

- -f: Não classifica, e usa -au para listar os arquivos.
- -r: Inverte a ordem de classificação.
- -c: Classifica pela data de alteração.
- -X: Classifica pela extensão.
- -U: Não classifica, lista os arquivos na ordem do diretório.
- -Z: Exibe o contexto SELinux de cada arquivo.

Uma listagem feita com o comando ls -la normalmente é mostrada da seguinte maneira

```
-rwxr-xr-- 1 gleydson user 8192 nov 4 16:00 teste
```

Abaixo as explicações de cada parte:

- -rwxr-xr-- São as permissões de acesso ao arquivo teste. A primeira letra (da esquerda) identifica o tipo do arquivo, se tiver um d é um diretório, se tiver um "-" é um arquivo normal.

As permissões de acesso é explicada em detalhes em Capítulo 11, Permissões de acesso
a arquivos e diretórios.

- 1: Se for um diretório, mostra a quantidade de sub-diretórios existentes dentro dele. Caso for um arquivo, será 1.
- gleydson: Nome do dono do arquivo teste.
- user: Nome do grupo que o arquivo teste pertence.
- 8192: Tamanho do arquivo (em bytes).
- nov: Mês da criação/ última modificação do arquivo.
- 4: Dia que o arquivo foi criado.
- 16:00: Hora em que o arquivo foi criado/modificado. Se o arquivo foi criado há mais de um ano, em seu lugar é mostrado o ano da criação do arquivo.
- teste: Nome do arquivo.

#### cd

Entra em um diretório. Você precisa ter a permissão de execução para entrar no diretório.

cd \[diretório]

- cd / : retornará ao diretório raíz.
- cd - : retornará ao diretório anteriormente acessado.
- cd .. : sobe um diretório

#### pwd

Mostra o nome e caminho do diretório atual.

#### mkdir

Cria um diretório (pasta) no sistema.

mkdir \[opções] \[caminho/diretório] \[caminho1/diretório1]

- caminho: Caminho onde o diretório será criado.
- diretório: Nome do diretório que será criado.
- opções:, -p: Caso os diretórios dos níveis acima não existam, eles também serão criados.
- --verbose: Mostra uma mensagem para cada diretório criado. As mensagens de erro serão mostradas mesmo que esta opção não seja usada.

#### rmdir

Remove um diretório do sistema. Este comando faz exatamente o contrário do mkdir.

rmdir \[opções] \[caminho/diretório] \[caminho1/diretório1]

- caminho: Caminho do diretório que será removido.
- diretório: Nome do diretório que será removido.

Para remover diretórios que contenham arquivos, use o comando rm com a opção -r

## Capítulo 7. Comandos para manipulação de Arquivos

#### cat

Mostra o conteúdo de um arquivo binário ou texto.

cat \[opções] \[diretório/arquivo] \[diretório1/arquivo1]

- diretório/arquivo: Localização do arquivo que deseja visualizar o conteúdo.
- opções, -n, --number: Mostra o número das linhas enquanto o conteúdo do arquivo é mostrado.
- -s, --squeeze-blank: Não mostra mais que uma linha em branco entre um parágrafo e outro.
- - Lê a entrada padrão.

O comando cat trabalha com arquivos texto. Use o comando zcat para ver diretamente arquivos compactados com gzip.

Exemplo: cat /usr/doc/copyright/GPL

#### tac

Mostra o conteúdo de um arquivo binário ou texto (como o cat) só que em ordem inversa.

tac \[opções] \[diretório/arquivo] \[diretório1/arquivo1]

- diretório/arquivo: Localização do arquivo que deseja visualizar o conteúdo
- opções, -s [string]: Usa o [string] como separador de registros.
- - Lê a entrada padrão.

Exemplo: tac /usr/doc/copyright/GPL.

#### rm

Apaga arquivos. Também pode ser usado para apagar diretórios e sub-diretórios vazios ou que contenham arquivos.

rm \[opções]\[caminho]\[arquivo/diretório] \[caminho1]\[arquivo1/diretório1]

- caminho: Localização do arquivo que deseja apagar. Se omitido, assume que o arquivo esteja no diretório atual.
- arquivo/diretório: Arquivo que será apagado.
- opções, -i, --interactive: Pergunta antes de remover, esta é ativada por padrão.
- -v, --verbose: Mostra os arquivos na medida que são removidos.
- -r, --recursive: Usado para remover arquivos em sub-diretórios. Esta opção também pode ser usada para remover sub-diretórios.
-f, --force: Remove os arquivos sem perguntar.
-- arquivo: Remove arquivos/diretórios que contém caracteres especiais. O separador "--" funciona com todos os comandos do shell e permite que os caracteres especiais como "*", "?", "-", etc. sejam interpretados como caracteres comuns.

Use com atenção o comando rm, uma vez que os arquivos e diretórios forem apagados, eles não poderão ser mais recuperados.

Exemplos:

- rm teste.txt - Apaga o arquivo teste.txt no diretório atual.
- rm *.txt - Apaga todos os arquivos do diretório atual que terminam com .txt.
- rm *.txt teste.novo - Apaga todos os arquivos do diretório atual que terminam com .txt e também o arquivo teste.novo.
- rm -rf /tmp/teste/* - Apaga todos os arquivos e sub-diretórios do diretório /tmp/teste mas mantém o sub-diretório /tmp/teste.
- rm -rf /tmp/teste - Apaga todos os arquivos e sub-diretórios do diretório /tmp/teste, inclusive /tmp/teste.
- rm -f -- --arquivo-- - Remove o arquivo de nome --arquivo--.

#### cp

Copia arquivos.

cp \[opções] \[origem] \[destino]

- origem: Arquivo que será copiado. Podem ser especificados mais de um arquivo para ser copiado usando "coringas" (veja “coringas”).
- destino: O caminho ou nome de arquivo onde será copiado. Se o destino for um diretório, os arquivos de origem serão copiados para dentro do diretório.
- opções, i, --interactive: Pergunta antes de substituir um arquivo existente.
- -f, --force: Não pergunta, substitui todos os arquivos caso já exista.
- -r: Copia arquivos dos diretórios e subdiretórios da origem para o destino. É recomendável usar -R ao invés de -r.
- -R, --recursive: Copia arquivos e sub-diretórios (como a opção -r) e também os arquivos especiais FIFO e dispositivos.
- -v, --verbose: Mostra os arquivos enquanto estão sendo copiados.
- -s, --simbolic-link: Cria link simbólico ao invés de copiar.
- -l, --link: Faz o link no destino ao invés de copiar os arquivos.
- -p, --preserve: Preserva atributos do arquivo, se for possível.
- -u, --update: Copia somente se o arquivo de origem é mais novo que o arquivo de destino ou quando o arquivo de destino não existe.
- -x: Não copia arquivos que estão localizados em um sistema de arquivos diferente de onde a cópia iniciou.

O comando cp copia arquivos da ORIGEM para o DESTINO. Ambos origem e destino terão o mesmo conteúdo após a cópia.

Exemplos:

- cp teste.txt teste1.txt Copia o arquivo teste.txt para teste1.txt.
- cp teste.txt /tmp Copia o arquivo teste.txt para dentro do diretório /tmp.
- cp * /tmp Copia todos os arquivos do diretório atual para /tmp.
- cp /bin/* . Copia todos os arquivos do diretório /bin para o diretório em que nos encontramos no momento.
- cp -R /bin /tmp Copia o diretório /bin e todos os arquivos/sub-diretórios existentes para o diretório /tmp.
- cp -R /bin/* /tmp Copia todos os arquivos do diretório /bin (exceto o diretório /bin) e todos os arquivos/sub-diretórios existentes dentro dele para /tmp.
- cp -R /bin /tmp Copia todos os arquivos e o diretório /bin para /tmp.
 
#### mv

Move ou renomeia arquivos e diretórios. O processo é semelhante ao do comando cp mas o arquivo de origem é apagado após o término da cópia.

mv [opções] [origem] [destino]

origem: Arquivo/diretório de origem.
destino:  Local onde será movido ou novo nome do arquivo/diretório.
opções, -f, --force: Substitui o arquivo de destino sem perguntar.
-i, --interactive: Pergunta antes de substituir. É o padrão.
-v, --verbose: Mostra os arquivos que estão sendo movidos.
-u, --update: Move somente arquivos antigos, ou novos arquivos

O comando mv copia um arquivo da ORIGEM para o DESTINO (semelhante ao cp), mas após a cópia, o arquivo de ORIGEM é apagado.

Exemplos:

- mv teste.txt teste1.txt Muda o nome do arquivo teste.txt para teste1.txt.
- mv teste.txt /tmp Move o arquivo teste.txt para /tmp. Lembre-se que o arquivo de origem é apagado após ser movido.
- mv teste.txt teste.new (supondo que teste.new já exista): Copia o arquivo teste.txt por cima de teste.new e apaga teste.txt após terminar a cópia.

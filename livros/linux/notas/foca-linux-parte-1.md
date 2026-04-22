# Introdução

- Capítulo 1. Introdução
- Capítulo 2. Explicações Básicas
- Capítulo 3. Para quem esta migrando (ou pensando em migrar) do DOS/Windows para o Linux
- Capítulo 4. Discos e Partições
- Capítulo 5. Execução de programas
- Capítulo 6. Comandos para manipulação de diretório

## Capítulo 1. Introdução

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

- Servidores Web e E-mail: o Linux permite a montagem de servidores como Apache (publicação Web) e Sendmail (e-mail) com baixo custo e alta performance. Esses aplicativos são distribuídos gratuitamente com a maioria das distribuições, sendo o Apache o servidor Web mais usado no mundo e o Sendmail parte da base de 72% dos servidores de e-mail atuais.


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

- `relatório.txt` — indica que o conteúdo é um arquivo de texto;
- `script.sh` — arquivo de script (interpretado por `/bin/sh`);
- `system.log` — registro de algum programa no sistema;
- `arquivo.gz` — arquivo compactado pelo utilitário gzip;
- `index.html` — página de internet (formato Hipertexto).


### Tamanho de arquivos

A unidade de medida padrão nos computadores é o bit. A um conjunto de 8 bits nós chamamos de byte.

O tamanho de arquivos é medido em bytes, onde cada byte equivale a um caractere. Para facilitar a leitura de grandes volumes, utilizam-se múltiplos progressivos:

- **KB** (Kilobyte) = 1.024 bytes
- **MB** (Megabyte) = 1.024 KB
- **GB** (Gigabyte) = 1.024 MB
- **TB** (Terabyte) = 1.024 GB

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

- `#` (Tralha): identifica o aviso de comando do superusuário (root).
- `$` (Cifrão): identifica o aviso de comando de usuários comuns.

### Terminal Virtual (console)

O Linux permite o uso de múltiplos terminais virtuais, que são sessões de trabalho independentes.

- **Modo texto:** utiliza-se `ALT + F1` a `F6` para alternar entre os seis terminais iniciais;
- **Modo gráfico:** deve-se usar `CTRL + ALT + F1` a `F6` para ir ao modo texto e `CTRL + ALT + F7` para retornar aos gráficos.

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

Path é o caminho de procura dos arquivos/comandos executáveis. O path (caminho) é armazenado na variável de ambiente `PATH`. Você pode ver o conteúdo desta variável com o comando `echo $PATH`.

Caso o interpretador de comandos chegue até o último diretório do path e não encontre o arquivo/comando digitado, é mostrada a seguinte mensagem:

```bash
pwd # command not found (comando não encontrado).
```
Se deseja alterar o path para um único usuário, modifique o arquivo `.bash_profile` em seu diretório de usuário (home).

### Tipos de Execução de Comandos/Programas

- Primeiro Plano (Foreground): é o modo padrão, onde o usuário deve esperar o término da execução do programa para poder digitar um novo comando no prompt;
  - Para iniciar um programa em primeiro plano, basta digitar seu nome normalmente.

- Segundo Plano (Background): permite que o sistema fique livre para outras tarefas enquanto o programa roda internamente.
  - Para iniciar um programa em segundo plano, acrescente o caractere `&` após o final do comando.

### Executando Programas em Sequência

Os comandos podem ser executados em sequência (um após o término do outro) se os separarmos com `";"`. Por exemplo: `echo primeiro;echo segundo;echo terceiro`.

#### ps

O comando `ps` lista os processos em execução no sistema, mostrando o usuário que os iniciou, o horário de início e outras informações.

```bash
ps [opções]
```

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

```bash
top [opções]
```

| Opção | Descrição |
|-------|-----------|
| `-d [tempo]` | Atualiza a tela após o intervalo de `[tempo]` segundos |
| `-s` | Executa o top em modo seguro |
| `-i` | Inicia o top ignorando o tempo de processos zumbis |
| `-c` | Mostra a linha de comando completa ao invés do nome do programa |

O manual do comando pode ser obtido dentro do programa pressionando `h` ou via `man top`:

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

**Atalhos de teclado:**

- `CTRL + C` — interrompe definitivamente a execução do processo;
- `CTRL + Z` — pausa momentaneamente o processo; retome com `fg` (primeiro plano) ou `bg` (segundo plano).

**Comandos:**

- `jobs` — lista os processos parados ou rodando em segundo plano;
- `fg [número]` — faz um processo parado ou em segundo plano voltar a rodar em primeiro plano;
- `bg [número]` — faz um processo parado ou em primeiro plano passar a rodar em segundo plano.

#### kill

O comando `kill` envia um sinal de término ao processo sendo executado.

```bash
kill [opções] [sinal] [número]
```

- `[opções] -9` — envia um sinal de destruição (`SIGKILL`) ao processo, encerrando-o imediatamente sem chance de salvar dados ou limpar arquivos temporários;
- `[sinal]` — sinal a ser enviado ao processo. Se omitido, usa `-15` (`SIGTERM`) como padrão;
- `[número]` — PID do processo, obtido com `ps`, ou o número após `%` retornado pelo comando `jobs`.

Você precisa ser o dono do processo ou o usuário root para encerrá-lo. Verifique se o processo foi finalizado com o comando `ps`.

**Principais sinais aceitos:**

| Sinal | Número | Descrição |
|-------|--------|-----------|
| `SIGHUP` | `-1` | Reinicia o processo (relê as configurações) |
| `SIGTERM` | `-15` | Solicita o encerramento gracioso (padrão) |
| `SIGKILL` | `-9` | Força o encerramento imediato, sem salvar dados |

```bash
kill 500       # encerra o processo 500 com SIGTERM
kill -9 500    # força o encerramento do processo 500
kill %1        # encerra o job de número 1
```

> Nota: Killall e killall5 - Leia!

### Eliminando caracteres estranhos

- `reset`: pode ser usado para lidar com programas que não estão funcionando corretamente, muito comum em scripts mal configurados.

## Capítulo 6. Comandos para manipulação de diretório

#### ls

Lista os arquivos de um diretório.

```bash
ls [opções] [caminho/arquivo] [caminho1/arquivo1]
```

- `caminho/arquivo` — diretório/arquivo que será listado;
- `caminho1/arquivo1` — outro diretório/arquivo que será listado. Podem ser feitas várias listagens de uma só vez.

**Opções:**

| Opção | Descrição |
|-------|-----------|
| `-a`, `--all` | Lista todos os arquivos, inclusive os ocultos |
| `-A`, `--almost-all` | Lista todos os arquivos ocultos, exceto `.` e `..` |
| `-B`, `--ignore-backups` | Não lista arquivos que terminam com `~` (backup) |
| `--color=PARAM` | Coloriza a saída: `never` (nunca), `always` (sempre), `auto` (somente em terminal) |
| `-d`, `--directory` | Lista os nomes dos diretórios ao invés do conteúdo |
| `-f` | Não classifica a listagem |
| `-F` | Insere um caractere identificador após o nome: `*` executável, `/` diretório, `=` socket, `@` link simbólico, `\|` pipe |
| `-G`, `--no-group` | Oculta a coluna de grupo do arquivo |
| `-h`, `--human-readable` | Mostra o tamanho dos arquivos em KB, MB ou GB (base 1024) |
| `-H` | Igual a `-h`, mas usa base 1000 |
| `-l` | Formato longo: lista permissões, data de modificação, dono, grupo, etc. |
| `-L`, `--dereference` | Lista o arquivo original e não o link simbólico |
| `-n` | Usa identificação numérica de usuário e grupo ao invés dos nomes |
| `-o` | Listagem longa sem a coluna de dono (equivale a `-lG`) |
| `-p` | Igual a `-F`, mas não inclui `*` em executáveis |
| `-R` | Lista diretórios e subdiretórios recursivamente |
| `--full-time` | Lista data e hora completas |

**Classificação da listagem:**

| Opção | Descrição |
|-------|-----------|
| `-f` | Não classifica; usa `-au` para listar os arquivos |
| `-r` | Inverte a ordem de classificação |
| `-c` | Classifica pela data de alteração |
| `-X` | Classifica pela extensão |
| `-U` | Não classifica; lista os arquivos na ordem do diretório |
| `-Z` | Exibe o contexto SELinux de cada arquivo |

**Interpretando a saída de `ls -la`:**

```
-rwxr-xr-- 1 gleydson user 8192 nov 4 16:00 teste
```

| Campo | Valor no exemplo | Significado |
|-------|-----------------|-------------|
| Permissões | `-rwxr-xr--` | Tipo e permissões do arquivo (`-` = arquivo, `d` = diretório) |
| Links | `1` | Quantidade de links; em diretórios, indica o número de subdiretórios |
| Dono | `gleydson` | Nome do usuário dono do arquivo |
| Grupo | `user` | Grupo ao qual o arquivo pertence |
| Tamanho | `8192` | Tamanho em bytes |
| Mês | `nov` | Mês de criação/última modificação |
| Dia | `4` | Dia de criação/última modificação |
| Hora/Ano | `16:00` | Hora de criação/modificação (se tiver mais de um ano, exibe o ano) |
| Nome | `teste` | Nome do arquivo |

> As permissões de acesso são explicadas em detalhes no Capítulo 11.

#### cd

Entra em um diretório. É necessário ter permissão de execução no diretório de destino.

```bash
cd [diretório]
```

- `cd /` — retorna ao diretório raiz;
- `cd -` — retorna ao diretório anteriormente acessado;
- `cd ..` — sobe um nível na árvore de diretórios.

#### pwd

Mostra o nome e o caminho absoluto do diretório atual.

#### mkdir

Cria um diretório no sistema.

```bash
mkdir [opções] [caminho/diretório] [caminho1/diretório1]
```

- `caminho` — caminho onde o diretório será criado;
- `diretório` — nome do diretório que será criado;
- `-p` — cria os diretórios intermediários que ainda não existem;
- `--verbose` — exibe uma mensagem para cada diretório criado.

#### rmdir

Remove um diretório vazio do sistema (operação inversa ao `mkdir`).

```bash
rmdir [opções] [caminho/diretório] [caminho1/diretório1]
```

- `caminho` — caminho do diretório que será removido;
- `diretório` — nome do diretório que será removido.

> Para remover diretórios que contenham arquivos, use `rm -r`.

## Capítulo 7. Comandos para manipulação de Arquivos

#### cat

Mostra o conteúdo de um arquivo de texto ou binário.

```bash
cat [opções] [diretório/arquivo] [diretório1/arquivo1]
```

- `diretório/arquivo` — localização do arquivo a ser visualizado;
- `-n`, `--number` — exibe o número de cada linha;
- `-s`, `--squeeze-blank` — suprime linhas em branco consecutivas, exibindo no máximo uma;
- `-` — lê a entrada padrão (`stdin`).

> Use `zcat` para visualizar diretamente arquivos compactados com gzip.

```bash
cat /usr/doc/copyright/GPL
```

#### tac

Mostra o conteúdo de um arquivo assim como o `cat`, mas em ordem inversa (da última linha para a primeira).

```bash
tac [opções] [diretório/arquivo] [diretório1/arquivo1]
```

- `diretório/arquivo` — localização do arquivo a ser visualizado;
- `-s [string]` — usa `[string]` como separador de registros;
- `-` — lê a entrada padrão (`stdin`).

```bash
tac /usr/doc/copyright/GPL
```

#### rm

Apaga arquivos, diretórios e subdiretórios.

```bash
rm [opções] [caminho] [arquivo/diretório] [caminho1] [arquivo1/diretório1]
```

- `caminho` — localização do arquivo a ser apagado (se omitido, assume o diretório atual);
- `arquivo/diretório` — arquivo ou diretório que será apagado;
- `-i`, `--interactive` — solicita confirmação antes de remover (padrão);
- `-v`, `--verbose` — exibe os arquivos conforme são removidos;
- `-r`, `--recursive` — remove arquivos em subdiretórios recursivamente;
- `-f`, `--force` — remove sem solicitar confirmação;
- `-- arquivo` — remove arquivos cujo nome contém caracteres especiais (`*`, `?`, `-`, etc.).

> **Atenção:** arquivos removidos com `rm` não podem ser recuperados.

**Exemplos:**

```bash
rm teste.txt               # remove o arquivo teste.txt no diretório atual
rm *.txt                   # remove todos os arquivos .txt do diretório atual
rm *.txt teste.novo        # remove todos os .txt e também o arquivo teste.novo
rm -rf /tmp/teste/*        # remove o conteúdo de /tmp/teste, mantendo o diretório
rm -rf /tmp/teste          # remove o diretório /tmp/teste e todo o seu conteúdo
rm -f -- --arquivo--       # remove o arquivo de nome --arquivo--
```

#### cp

Copia arquivos e diretórios.

```bash
cp [opções] [origem] [destino]
```

- `origem` — arquivo(s) a ser(em) copiado(s); aceita coringas;
- `destino` — caminho ou nome de destino; se for um diretório, os arquivos são copiados para dentro dele;
- `-i`, `--interactive` — solicita confirmação antes de substituir;
- `-f`, `--force` — substitui sem solicitar confirmação;
- `-r` — copia arquivos e subdiretórios (use `-R` de preferência);
- `-R`, `--recursive` — copia arquivos, subdiretórios e arquivos especiais (FIFO e dispositivos);
- `-v`, `--verbose` — exibe os arquivos enquanto são copiados;
- `-s`, `--symbolic-link` — cria link simbólico ao invés de copiar;
- `-l`, `--link` — cria hard link no destino ao invés de copiar;
- `-p`, `--preserve` — preserva atributos do arquivo (permissões, datas);
- `-u`, `--update` — copia somente se a origem for mais recente que o destino, ou se o destino não existir;
- `-x` — não copia arquivos que estejam em um sistema de arquivos diferente do de origem.

**Exemplos:**

```bash
cp teste.txt teste1.txt        # copia teste.txt para teste1.txt
cp teste.txt /tmp              # copia teste.txt para dentro de /tmp
cp * /tmp                      # copia todos os arquivos do diretório atual para /tmp
cp /bin/* .                    # copia todos os arquivos de /bin para o diretório atual
cp -R /bin /tmp                # copia o diretório /bin e todo o seu conteúdo para /tmp
cp -R /bin/* /tmp              # copia o conteúdo de /bin (sem o diretório em si) para /tmp
```

#### mv

Move ou renomeia arquivos e diretórios. Funciona como o `cp`, mas o arquivo de origem é removido após a cópia.

```bash
mv [opções] [origem] [destino]
```

- `origem` — arquivo ou diretório de origem;
- `destino` — local de destino ou novo nome;
- `-f`, `--force` — substitui o destino sem solicitar confirmação;
- `-i`, `--interactive` — solicita confirmação antes de substituir (padrão);
- `-v`, `--verbose` — exibe os arquivos conforme são movidos;
- `-u`, `--update` — move somente se a origem for mais recente que o destino.

**Exemplos:**

```bash
mv teste.txt teste1.txt    # renomeia teste.txt para teste1.txt
mv teste.txt /tmp          # move teste.txt para /tmp (o original é removido)
mv teste.txt teste.new     # se teste.new já existir, sobrescreve e remove teste.txt
```

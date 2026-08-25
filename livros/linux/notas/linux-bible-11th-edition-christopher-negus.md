# Linux - Bible

Esse material foi escrito com o intuito de facilitar o acesso a comentários e detalhes importantes abordados ao longo do livro. Estou escrevendo meu próprio livro e estou buscando referências do material de outros autores.

[x] Chapter 1: Starting with Linux
[x] Chapter 2: Creating the Perfect Linux Desktop
[]
[]
[]
[]
[]

---

## Chapter 1: Starting with Linux

### Understanding What Linux Is

Linux é um sistema de computador, que permite ao usuário a possibilidade de instalar aplicações para executar determinadas tarefas. Possuindo um conjunto de ferramentas para poder funcionar, sendo elas:

- Detecção e preparação do hardware;
- Gerenciamento de processos;
- Gerenciamento de memória;
- Fornecendo interfaces de usuário;
- Controlador de sistema de arquivos;
- Provedor de acesso e autenticação de usuário;
- Ferramentas de administrador;
- Iniciador de serviços;
- Ferramentas de programação.

O sistema também disponibiliza uma grande quantidade de ferramentas voltadas para a empresa:

- Clustering;
- Virtualização;
- Computação em nuvem;
- Programação em tempo real;
- Armazenamento especializado.

### Exploring Linux History

1. A Origem: UNIX e a Cultura da Bell Labs

O Linux é um "clone" do UNIX, sistema criado nos laboratórios da AT&T Bell Labs por Ken Thompson e Dennis Ritchie por volta de 1969. O UNIX nasceu em um ambiente comunal e acadêmico, focado na eficiência técnica e não apenas em necessidades de mercado. Suas fundações incluíam:

- Sistema de arquivos hierárquico: organização intuitiva de pastas e arquivos;
- Redirecionamento e Pipes: capacidade de conectar utilitários simples para realizar tarefas complexas;
- Portabilidade: uso da linguagem C (criada por Ritchie e Kernighan) permitiu que o UNIX fosse facilmente adaptado para diferentes hardwares.

2. Comercialização

Eventualmente, a AT&T comercializou o UNIX, o que levou à criação de padrões como o POSIX, que serviram de roteiro para a criação de sistemas compatíveis, como o próprio Linux.

3. O Movimento Software Livre (GNU)

Em 1984, Richard Stallman iniciou o Projeto GNU (GNU is Not UNIX) com o objetivo de recriar todo o sistema UNIX de forma livre. O projeto desenvolveu quase todos os componentes necessários (editores, compiladores, shells), mas ainda faltava um "núcleo" (kernel) funcional.

4. Linus Torvalds e a "Peça Faltante"

Em 1991, Linus Torvalds, um estudante finlandês, anunciou o desenvolvimento de um kernel compatível com UNIX para rodar em seu PC doméstico. O Linux tornou-se a peça final que completou o sistema operacional livre sob a licença GPL.

O nome "Linux" acabou se tornando mais popular que "GNU", embora distribuições como o Debian se identifiquem como GNU/Linux.

5. O Surgimento das Distribuições

Para facilitar o uso do sistema por usuários não técnicos, surgiram as distribuições (distros). Algumas das pioneiras e mais influentes incluem:

- Slackware: Uma das mais antigas ainda em uso.
- Red Hat: Popularizou o gerenciamento de pacotes RPM e focou no mercado corporativo.
- Debian: Conhecida pela estabilidade e pelo formato de pacotes .deb.


## Chapter 2: Creating the Perfect Linux Desktop

Leia!


## Chapter 3: Using the Shell

### About Shells and Terminal Windows

O Linux permite o uso de mais de um terminal shell, porém o mais comum é o Bash shell, que leva o nome de Stephen Bourne.

A popularidade de um shell se dá pela facilidade no processo de passar os comandos. Mesmo sendo o Bash o mais popular, outros vêm ganhando cada vez mais espaço nas distribuições, tais como:

- Fish;
- Zsh.

#### Using the shell prompt

Por padrão, alguns símbolos são usados para definir o usuário:

- $: usuário normal
- #: usuário root

Ao abrir um terminal padrão, você irá visualizar:

```bash
# [nome do usuário @ nome da máquina pasta atual]
[nome@sistema desktop]:
```

### Running Commands

Muitos comandos precisam apenas que o nome seja passado via terminal:

```bash
# date
Sun Aug 16 03:22:34 PM -03 2026

# pwd
/home/user/git/

# hostname
mydesktop
```

#### Understanding command syntax

Da mesma forma que muitos comandos funcionam apenas com o nome, outros fazem uso de opções que podem ser passadas para um resultado mais completo:

```bash
# ls (lista)
downloads  pictures

# ls -l (lista detalhada)
total 0
drwxr-xr-x 1 user users 52 Aug 15 18:57 downloads
drwxr-xr-x 1 user users 20 Aug  6 14:23 pictures

# ls -l -a (lista detalhada com arquivos ocultos)
total 0
drwxr-xr-x 1 user users  34 Aug  6 14:34 .
drwxr-xr-x 1 user users  52 Aug 15 18:57 downloads
drwxr-xr-x 1 user users  20 Aug  6 14:23 pictures

# ls -l -a -t (lista detalhada com arquivos ocultos e ordenados por tempo)
total 0
drwxr-xr-x 1 user users  52 Aug 15 18:57 downloads
drwxr-xr-x 1 user users  34 Aug  6 14:34 .
drwxr-xr-x 1 user users  20 Aug  6 14:23 pictures

# ls -lat (o mesmo resultado que -lat)
total 0
drwxr-xr-x 1 user users  52 Aug 15 18:57 downloads
drwxr-xr-x 1 user users  34 Aug  6 14:34 .
drwxr-xr-x 1 user users  20 Aug  6 14:23 pictures
```
As opções podem ser passadas separadas ou juntas e vão gerar o mesmo resultado.

Algo muito importante é entender que os comandos podem ser passados por extenso e seu uso se dá de uma maneira muito mais comum em scripts, pois fica muito mais fácil visualizar o que o script deve fazer.

```bash
# ls -a
ls --all

# ls -lat
ls --format=long --all --sort=time
```

Para saber informações sobre o usuário:

```bash
# id
uid=1000(user) gid=100(users) groups=100(users),1(wheel),57(networkmanager)
```

- uid: número de identificação do seu usuário;
- 1000: numérico exclusivo atribuído a você (1000 indica usuários comuns do sistema);
- (user): nome de usuário (username).

- gid: ID do seu grupo primário;
- 100: número do grupo principal;
- (users): nome do grupo primário.

- groups: todos os grupos (secundários);
- 100(users): grupo padrão de usuários comuns;
- 1(wheel): grupo com permissão de administrador;
- 57(networkmanager): grupo que permite gerenciar e alterar as conexões de rede do computador sem pedir senha de administrador.


Para saber informações detalhadas do usuário logado no sistema:

```bash
# who -uH
NAME     LINE         TIME             IDLE          PID COMMENT
user     tty1         2026-08-16 12:12 03:42        1520
user     pts/2        2026-08-16 15:19 00:34       19086  
```

- who: lista os usuários logados;
- u: tempo de inatividade do usuário e o PID;
- H: adiciona um cabeçalho no topo da tabela para identificar o que significa cada coluna.


#### Locating commands

Ao usar o which seguido do nome do comando, ele vai retornar o caminho aonde está locazidado o arquivo daquele comando.

```bash
# which nome_do_comando
which ls
# /usr/sbin/ls
```

##### Alias

O livro não destaca muito esse ponto, mas isso é importante para o usuário: um alias é uma forma de criar um apelido para um comando ou script. Usamos atalhos assim para evitar ter que digitar o comando completo o tempo todo.

Ao instalar um programa, para abri-lo você pode dar dois cliques sobre o ícone ou digitar o nome do comando no terminal:

```bash
# neovim
nvim

# helix
hx
```

Para definir o nome de um alias, siga algumas regras simples:

- Evite palavras reservadas a outros comandos existentes do sistema;
- Evite palavras reservadas da sintaxe do seu shell (como if, for, function).

Para criar o alias, você só precisa editar o arquivo de configuração do seu shell:

```bash
# Abra o arquivo correspondente ao seu shell (ex: ~/.bashrc, ~/.zshrc ou ~/.config/fish/config.fish)
vim ~/.bashrc
```

Adicione o comando no arquivo seguindo a estrutura abaixo, depois basta salvar a alteração:

```bash
# Configurações do arquivo .bashrc
# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# alias nome-do-atalho='comando completo'
alias ls='ls --color=auto'

PS1='[\u@\h \W]\$ '
. "$HOME/.cargo/env"
```

Depois de salvar, recarregue as alterações feitas no seu arquivo de configuração:

```bash
source ~/.bashrc
```

### Recalling Commands Using Command History

O terminal ele por mais simples que parece tem algumas ferramentas que facilitam bastante o seu uso no dia a dia:

- history: armazena todos os últimos comandos que você utilizar via terminal, até você fechar a janela

```bash
# history
history

# 1 ls
# 2 more
# 3 vim
```

- autocompletar: o terminal também permite o uso do auto completar, usando a tecla TAB do teclado, basta digitar parte do comando e depois tab , funciona tanto para comando, quanto para o nome de arquivos e pastas.

```bash
# histo + tab
history

# neo + tab
neovim
```

#### Navigating Command Lines

Existem algumas combinações de teclas que facilitam o uso da navegação via terminal, como são muitas, vou apenas listar algumas:

| Atalho | Significado |
|---|---|
| Ctrl+F | Avança um caractere. |
| Ctrl+B | Retrocede um caractere. |
| Alt+F | Avança uma palavra. |
| Alt+B | Retrocede uma palavra. |
| Ctrl+A | Vai para o início da linha atual. |
| Ctrl+E | Vai para o fim da linha. |
| Ctrl+L | Limpa a tela e deixa a linha no topo da tela. |

#### Editing Command Lines

| Atalho | Significado |
|---|---|
| Ctrl+D | Exclui o caractere atual. |
| Backspace | Exclui o caractere anterior. |
| Ctrl+T | Troca a posição do caractere atual com o anterior. |
| Alt+T | Troca a posição da palavra atual com a anterior. |
| Alt+U | Transforma a palavra atual em maiúsculas. |
| Alt+L | Transforma a palavra atual em minúsculas. |
| Alt+C | Transforma a palavra atual com a primeira letra maiúscula. |
| Ctrl+V | Adiciona um caractere especial. Por exemplo, para adicionar um caractere de Tab, pressione Ctrl+V+Tab. |

#### Cutting and Pasting Text from within Command Lines

| Atalho | Significado |
|---|---|
| Ctrl+K | Recorta o texto até o fim da linha. |
| Ctrl+U | Recorta o texto até o início da linha. |
| Ctrl+W | Recorta a palavra localizada atrás do cursor. |
| Alt+D | Recorta a palavra seguinte ao cursor. |
| Ctrl+Y | Cola o texto recortado mais recentemente. |
| Alt+Y | Retorna ao texto recortado anteriormente e cola-o. |
| Ctrl+C | Exclui a linha inteira. |


```bash

```

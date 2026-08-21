# Sumário

## 1 - Introdução

- [x] 1.1 Mantendo o histórico do código
- [x] 1.2 Trabalhando em equipe
- [x] 1.3 Sistemas de controle de versão
- [x] 1.4 Controle de versão rápido e confiável com Git
- [x] 1.5 Hospedando código no GitHub
- [x] 1.6 O processo de escrita desse livro

## 2 - Tour prático

- [x] 2.1 Instalando e configurando o Git
- [x] 2.2 Criando um arquivo texto para versionarmos
- [x] 2.3 Versionando seu código com Git
- [x] 2.4 Compartilhando seu código através do GitHub

## 3 - Trabalhando com repositório local 19

- [x] 3.1 Criando um repositório local
- [x] 3.2 Rastreando arquivos
- [x] 3.3 Gravando arquivos no repositório
- [x] 3.4 Verificando o histórico do seu repositório
- [x] 3.5 Verificando mudanças nos arquivos rastreados
- [x] 3.6 Removendo arquivos do repositório
- [x] 3.7 Renomeando e movendo arquivos
- [x] 3.8 Desfazendo mudanças

## 4 - Trabalhando com repositório remoto 61

- [x] 4.1 Repositório remoto
- [x] 4.2 Adicionando o repositório remoto
- [x] 4.3 Enviando commits para o repositório remoto
- [x] 4.4 Clonando o repositório remoto
- [x] 4.5 Sincronizando o repositório local
- [x] 4.6 Protocolos suportados pelo Git

## 5 - Hospedando o repositório no GitHub

- [x] 5.1 Serviços de hospedagem de projetos
- [x] 5.2 GitHub: a rede social dos desenvolvedores
- [x] 5.3 Encontrando projetos e visualizando o código-fonte
- [x] 5.4 Criando um usuário no GitHub
- [x] 5.5 Criando o repositório do projeto
- [x] 5.6 Enviando os commits do projeto para o GitHub
- [x] 5.7 Clonando o repositório hospedado no GitHub
- [x] 5.8 Colaborando com projetos open source

## 6 - Organizando o trabalho com branches

- [x] 6.1 A branch master
- [x] 6.2 Criando uma branch
- [x] 6.3 Trocando de branch
- [x] 6.4 Deletando uma branch
- [x] 6.5 Comitando código em uma nova branch
- [x] 6.6 Voltando para o master e fazendo uma alteração
- [x] 6.7 Mesclando alterações

## 7 - Trabalhando em equipe com branches remotas

- [x] 7.1 Branches remotas
- [x] 7.2 Compartilhando branches
- [x] 7.3 Obtendo novas branches remotas em outros repositórios
- [x] 7.4 Enviando commits para o repositório central
- [x] 7.5 Obtendo commits de uma branch remota
- [x] 7.6 Mesclando branches remotas e locais
- [x] 7.7 Deletando branches remotas

## 8 - Controlando versões do código com tags

- [x] 8.1 Criando, listando e deletando tags
- [x] 8.2 Mais informações com tags anotadas
- [x] 8.3 Compartilhando tags com a sua equipe

## 9 - Lidando com conflitos

- [x] 9.1 Mesclando mudanças em um mesmo arquivo sem conflitos
- [x] 9.2 Conflitos após um merge com mudanças em um mesmo arquivo
- [x] 9.3 Resolvendo conflitos após um rebase
- [x] 9.4 Usando uma ferramenta para resolver conflitos

## 10 - Maneiras de trabalhar com Git

- [x] 10.1 Utilizando só a branch master com um repositório central
- [x] 10.2 Utilizando branches por funcionalidade com um repositório central
- [x] 10.3 Utilizando branches por etapa de desenvolvimento com um repositório central
- [x] 10.4 Colaborando com projetos open source com Fork e Pull Request
- [x] 10.5 Organizando projetos open source gigantescos com Ditador e Tenentes

## Introdução

1.1 Mantendo o histórico do código

Pula!

1.2 Trabalhando em equipe

Pula!

1.3 Sistemas de controle de versão

Pula!

1.4 Controle de versão rápido e confiável com Git

O Git: Criado em 2005 por Linus Torvalds, o Git é descrito como um sistema extremamente rápido e confiável.

1.5 Hospedando código no GitHub

Pula!

1.6 O processo de escrita desse livro

Pula!

## Tour prático

2.1 Instalando e configurando o Git

O primeiro passo é instalar o Git de acordo com o seu sistema operacional (Windows, Mac ou Linux). Após a instalação, é fundamental realizar a identificação básica, configurando seu nome e e-mail, que serão associados aos seus commits:

A configuração a seguir não está no livro, mas coloquei logo:

#### Configuração Inicial

#### Usuário único

```bash
# Identidade global
git config --global user.name "Seu Nome"
git config --global user.email "[email protected]"

# Gerar chave SSH
ssh-keygen -t ed25519 -C "[email protected]"
# Pressione Enter para usar os valores padrão

# Exibir a chave pública (copie e cole no GitHub)
cat ~/.ssh/id_ed25519.pub
# GitHub > Settings > SSH and GPG keys > New SSH key
```

#### Múltiplos usuários (pessoal + trabalho)

```bash
# Gerar chave para conta pessoal
ssh-keygen -t ed25519 -C "[email protected]"
# Quando solicitado o arquivo, digite: id_pessoal

# Gerar chave para conta de trabalho
ssh-keygen -t ed25519 -C "[email protected]"
# Quando solicitado o arquivo, digite: id_trabalho

# Adicionar as chaves ao agente SSH
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_pessoal
ssh-add ~/.ssh/id_trabalho

# Exibir as chaves públicas (adicione cada uma no GitHub correspondente)
cat ~/.ssh/id_pessoal.pub
cat ~/.ssh/id_trabalho.pub
```

##### Arquivo `~/.ssh/config`

```
# Conta pessoal
Host github.com-pessoal
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_pessoal

# Conta de trabalho
Host github.com-trabalho
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_trabalho
```

```bash
# Testar as conexões
ssh -T [email protected]
# Hi <usuario-pessoal>! You've successfully authenticated...

ssh -T [email protected]
# Hi <usuario-trabalho>! You've successfully authenticated...

# Clonar usando o host correto
git clone [email protected]:usuario-pessoal/repositorio.git
git clone [email protected]:usuario-trabalho/repositorio.git
```

2.2 Criando um arquivo texto para versionarmos

Pula!

2.3 Versionando seu código com Git

```bash
# Clonar um repositório existente
git clone https://github.com/usuario/projeto.git

# Criar um repositório novo do zero
git init [nome-do-projeto]

# Ver estado atual dos arquivos
git status

# Adicionar arquivos para o próximo commit
git add .                  # todos os arquivos alterados
git add [nome-do-arquivo]  # um arquivo específico

# Salvar as mudanças no histórico
git commit -m "descrição da mudança"

# Enviar commits para o repositório remoto
git push origin main

# Ver histórico
git log
git log --oneline #Ver histórico resumido de commits
```

2.4 Compartilhando seu código através do GitHub

O capítulo aborda a criação de conta e o processo de finalização com o comando.

```bash
# Enviar commits para o repositório remoto
git push origin main
```

## 3 Trabalhando com repositório local 19

3.1 Criando um repositório local

git init: É o ponto de partida. Ao executar este comando, o Git cria uma pasta oculta chamada .git, que funciona como um repositório completo contendo todo o histórico de alterações.

3.2 Rastreando arquivos

Um dos conceitos mais importantes deste capítulo é a separação entre o diretório de trabalho e o repositório, mediada pela Área de Stage:

- Arquivos não rastreados (untracked): Arquivos novos que o Git ainda não "vê".
- git add: Move os arquivos para a área de Stage. A partir daí, o Git monitora cada mudança neles.
- git commit: Grava definitivamente as mudanças que estão no Stage no repositório. Cada commit gera um identificador único de 40 caracteres (SHA-1).
- Utilidade do Stage: O autor explica que essa área permite agrupar mudanças de forma lógica. Você pode modificar vários arquivos, mas adicionar e comitar apenas aqueles que fazem parte de uma mesma funcionalidade, criando um histórico mais limpo.

#### .gitignore

O `.gitignore` instrui o Git a não rastrear determinados arquivos ou pastas.

```bash
# Ignorar uma pasta inteira
node_modules/

# Ignorar arquivos com variáveis de ambiente
.env

# Ignorar todos os arquivos de log
*.log

# Ignorar arquivos do sistema operacional
Thumbs.db
.DS_Store

# Abrir exceção para um arquivo específico
!config_fixo.log
```

3.3 Gravando arquivos no repositório

Explicação dos comandos:

```bash
# coloca todos os arquivos modificados na "fila" (Staging)
git commit add .
git commit add [nome-do-arquivo.md]

# Um "ponto de salvamento" (checkpoint) com os arquivos que você já adicionou e grava a mensagem
git commit -m "descrição"

# Adiciona automaticamente todas as modificações de arquivos que o Git já conhece e faz o commit.
git commit -a -m "descrição"
git commit -am "descrição"
```

3.4 Verificando o histórico do seu repositório

```bash
# Exibe o histórico de commits.
git log
git log --oneline # Resumo
git log -n 2 # Mostra apenas os dois últimos
git log --start
```

3.5 Verificando mudanças nos arquivos rastreados

```bash
# Verifica alterações entre o arquivo alterado e o comitado anteriormente
git diff
git diff HEAD
```

3.6 Removendo arquivos do repositório

```bash
# Adicionou o arquivo, fez o commit e precisa remover agora:
git rm [nome-do-arquivo.md]
```

3.7 Renomeando e movendo arquivos

```bash
# Renomeando um arquivo
git mv [nome-do-arquivo.md] [nome-do-arquivo.md]

# Movendo um arquivo
# Crie a pasta antes
git mv principal.js js/principal.js
```

3.8 Desfazendo mudanças

```bash
# Desfaz as alterações que ainda não foram rastreadas
git checkout -- index.html

# Remove da área de stage e sem modificar
git reset -- index.html

# Remove da área de stage e desfaz as modificações
git reset --hard
```

```bash
# Cria um novo commit que desfaz exatamente o que foi feito no seu último commit
git revert HEAD
```

## Trabalhando com repositório remoto 61

4.1 Repositório remoto

Pula!

4.2 Adicionando o repositório remoto

Veja vídeo ou siga o tutorial do Github

4.3 Enviando commits para o repositório remoto

Veja vídeo ou siga o tutorial do Github

4.4 Clonando o repositório remoto

Veja vídeo ou siga o tutorial do Github

4.5 Sincronizando o repositório local

Leia!

4.6 Protocolos suportados pelo Git

Leia e veja vídeo ou siga o tutorial no Youtube

## Hospedando o repositório no GitHub

5.1 Serviços de hospedagem de projetos

Pula!

5.2 GitHub: a rede social dos desenvolvedores

- Issue tracker: para criação e gestão de bugs e milestones do projeto;
- Pull requests: para que outros usuários possam enviar seus commits com alterações no projeto, ou commits com correções de bugs;
- Commit comments: para que os usuários possam comentar e discutir sobre as modificações no código, de um determinado commit.

  5.3 Encontrando projetos e visualizando o código-fonte

Leia e siga o tutorial no Youtube

5.4 Criando um usuário no GitHub

Leia e siga o tutorial no Youtube

5.5 Criando o repositório do projeto

Leia e siga o tutorial no Youtube

5.6 Enviando os commits do projeto para o GitHub

Leia e siga o tutorial no Youtube

5.7 Clonando o repositório hospedado no GitHub

Leia e siga o tutorial no Youtube

5.8 Colaborando com projetos open source

Leia e siga o tutorial no Youtube

## Organizando o trabalho com branches

6.1 A branch master

Uma branch é uma linha independente de desenvolvimento em que podemos comitar novas versões do código sem afetar outras
branches, funciona como um save de um jogo.

```bash
# Lista as branches do repositório
git branch

# Lista as branches do repositório e os commits associados a ela
git branch -v
```

6.2 Criando uma branch

```bash
# Cria uma branch
git branch [nome-da-branch]
```

6.3 Trocando de branch

```bash
# Troca
git checkout [nome-da-branch]

# Atualmente o comando é
git switch [nome-da-branch]

# Criando e já trocando
git switch -b [nome-da-branch]
```

6.4 Deletando uma branch

```bash
# Deletando uma branch sem commits
git branch -d [nome-da-branch]

# Deletando uma branch com commits
git branch -D [nome-da-branch]
```

6.5 Comitando código em uma nova branch

Leia!

6.6 Voltando para o master e fazendo uma alteração

Leia!

6.7 Mesclando alterações

Mesclar 2 branchs é juntar as mudanças dela em uma única

```bash
# Lista branches não mescladas
git branch --no-merged

# Mecla 2 branches
git branch merge [nome-da-branch] -m "descrição"

# Como fazer o rebase
git rebase [nome-da-branch]
```

Dica: se for fazer rebase, faça na sua máquina e não no repositório

- Merge mantém um registro fiel do que ocorreu com o nosso re-
  positório
- Rebase simplifica o histórico, mas perdemos informação sobre nosso
  repositório e alguns commits são reescritos.

## Trabalhando em equipe com branches remotas

7.1 Branches remotas

```bash
# Lista apenas as branches remotas
git branch -r

# Lista todas as branches (locais e remotas)
git branch -a -v
```

7.2 Compartilhando branches

```bash
# Envia a branch para o repositório remoto
git push origin [nome-da-branch]
```

7.3 Obtendo novas branches remotas

Cria uma branch local (tracking branch) para trabalhar em uma branch que já existe no servidor.

```bash
# Cria branch local rastreando a remota
git checkout -b [nome] origin/[nome]

# Atalho para criar tracking branch
git checkout -t origin/[nome]
```

7.4 Enviando commits para o repositório central

```bash
# Sincroniza os novos commits com o servidor
git push origin [nome-da-branch]
```

7.5 Obtendo commits de uma branch remota

```bash
# Baixa as mudanças do servidor (sem merge)
git fetch origin
```

7.6 Mesclando branches remotas e locais

```bash
# Atalho para baixar e mesclar (fetch + merge)
git pull

# Atalho para baixar e aplicar rebase (fetch + rebase)
git pull --rebase

# Mescla manualmente após um fetch
git merge origin/[nome-da-branch]
```

7.7 Deletando branches remotas

```bash
# Deleta a branch diretamente no servidor
git push origin :[nome-da-branch]
```

## Controlando versões do código com tags

8.1 Criando, listando e deletando tags

```bash
# Cria uma tag no commit atual
git tag [nome-da-tag]

# Cria uma tag para um commit específico do passado
git tag [nome-da-tag] [codigo-do-commit]

# Lista as tags existentes no repositório
git tag

# Remove uma tag localmente
git tag -d [nome-da-tag]
```

8.2 Mais informações com tags anotadas

```bash
# Cria uma tag anotada com mensagem descritiva
git tag -a [nome-da-tag] -m "[mensagem]"

# Exibe informações detalhadas da tag e do commit relacionado
git show [nome-da-tag]
```

8.3 Compartilhando tags com a sua equipe

```bash
# Envia uma tag específica para o servidor remoto (ex: GitHub)
git push origin [nome-da-tag]

# Envia todas as tags locais para o servidor de uma só vez
git push origin --tags
```

## Lidando com conflitos

9.1 Mesclando mudanças em um mesmo arquivo sem conflitos

Leia e veja um vídeo Youtube

9.2 Conflitos após um merge com mudanças em um mesmo arquivo

Leia e veja um vídeo Youtube

9.3 Resolvendo conflitos após um rebase

Leia e veja um vídeo Youtube

9.4 Usando uma ferramenta para resolver conflitos

Leia e veja um vídeo Youtube

## Maneiras de trabalhar com Git

10.1 Utilizando só a branch master com um repositório central

Leia e veja um vídeo Youtube

10.2 Utilizando branches por funcionalidade com um repositório central

Leia e veja um vídeo Youtube

10.3 Utilizando branches por etapa de desenvolvimento com um repositório central

Leia e veja um vídeo Youtube

10.4 Colaborando com projetos open source com Fork e Pull Request

Leia e veja um vídeo Youtube

10.5 Organizando projetos open source gigantescos com Ditador e Tenentes

Leia e veja um vídeo Youtube

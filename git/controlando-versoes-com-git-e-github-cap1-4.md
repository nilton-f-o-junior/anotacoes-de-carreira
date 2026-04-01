# Sumário

## Introdução

- [x] 1.1 Mantendo o histórico do código
- [x] 1.2 Trabalhando em equipe
- [x] 1.3 Sistemas de controle de versão
- [x] 1.4 Controle de versão rápido e confiável com Git
- [x] 1.5 Hospedando código no GitHub
- [x] 1.6 O processo de escrita desse livro

## 2 Tour prático

- [x] 2.1 Instalando e configurando o Git
- [x] 2.2 Criando um arquivo texto para versionarmos
- [x] 2.3 Versionando seu código com Git
- [x] 2.4 Compartilhando seu código através do GitHub 

## 3 Trabalhando com repositório local 19

- [x] 3.1 Criando um repositório local
- [x] 3.2 Rastreando arquivos
- [x] 3.3 Gravando arquivos no repositório
- [x] 3.4 Verificando o histórico do seu repositório
- [x] 3.5 Verificando mudanças nos arquivos rastreados
- [x] 3.6 Removendo arquivos do repositório
- [x] 3.7 Renomeando e movendo arquivos
- [x] 3.8 Desfazendo mudanças

# 4 Trabalhando com repositório remoto 61

- [x] 4.1 Repositório remoto
- [x] 4.2 Adicionando o repositório remoto 
- [x] 4.3 Enviando commits para o repositório remoto
- [x] 4.4 Clonando o repositório remoto
- [x] 4.5 Sincronizando o repositório local 
- [x] 4.6 Protocolos suportados pelo Git


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

## 2 Tour prático

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

# 4 Trabalhando com repositório remoto 61

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

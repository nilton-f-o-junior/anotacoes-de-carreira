# Sumário

## Hospedando o repositório no GitHub

- [x] 5.1 Serviços de hospedagem de projetos
- [x] 5.2 GitHub: a rede social dos desenvolvedores
- [x] 5.3 Encontrando projetos e visualizando o código-fonte
- [x] 5.4 Criando um usuário no GitHub
- [x] 5.5 Criando o repositório do projeto
- [x] 5.6 Enviando os commits do projeto para o GitHub
- [x] 5.7 Clonando o repositório hospedado no GitHub
- [x] 5.8 Colaborando com projetos open source

## Organizando o trabalho com branches

- [x] 6.1 A branch master
- [x] 6.2 Criando uma branch
- [x] 6.3 Trocando de branch
- [x] 6.4 Deletando uma branch
- [x] 6.5 Comitando código em uma nova branch
- [x] 6.6 Voltando para o master e fazendo uma alteração
- [x] 6.7 Mesclando alterações

## Trabalhando em equipe com branches remotas

- [x] 7.1 Branches remotas
- [x] 7.2 Compartilhando branches
- [x] 7.3 Obtendo novas branches remotas em outros repositórios
- [x] 7.4 Enviando commits para o repositório central
- [x] 7.5 Obtendo commits de uma branch remota
- [x] 7.6 Mesclando branches remotas e locais
- [x] 7.7 Deletando branches remotas

## Controlando versões do código com tags

- [x] 8.1 Criando, listando e deletando tags
- [x] 8.2 Mais informações com tags anotadas
- [x] 8.3 Compartilhando tags com a sua equipe

## Lidando com conflitos

- [x] 9.1 Mesclando mudanças em um mesmo arquivo sem conflitos
- [x] 9.2 Conflitos após um merge com mudanças em um mesmo arquivo
- [x] 9.3 Resolvendo conflitos após um rebase
- [x] 9.4 Usando uma ferramenta para resolver conflitos

## Maneiras de trabalhar com Git

- [x] 10.1 Utilizando só a branch master com um repositório central
- [x] 10.2 Utilizando branches por funcionalidade com um repositório central
- [x] 10.3 Utilizando branches por etapa de desenvolvimento com um repositório central
- [x] 10.4 Colaborando com projetos open source com Fork e Pull Request
- [x] 10.5 Organizando projetos open source gigantescos com Ditador e Tenentes


## Hospedando o repositório no GitHu

5.1 Serviços de hospedagem de projetos

Pula!

5.2 GitHub: a rede social dos desenvolvedores

- Issue rracker: para criação e gestão de bugs e milestones do projeto;
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

# Comandos Básicos

## Inicia repositório

- git clone

```bash
# clonar um repositório existente
git clone https://github.com/usuario/projeto.git
```

- git init

```bash
# criar um repositório novo do zero
git init meu-projeto
```

## Modificando Commits

- git add

```bash
# adicionar arquivos para o próximo commit
git add index.html        # um arquivo específico
git add .                 # todos os arquivos alterados
```

- git add -i

```bash
# ativa o modo iterativo
git add -i

# select > enter
```

- git reset

```bash
# volta atrás no comando add
git reset index.html      # um arquivo específico
git reset --              # todos os arquivos alterados
```

- git mv

```bash
# mover ou renomear um arquivo
git mv velho-nome.txt novo-nome.txt
```

- git restore

```bash
# Descartar alterações não commitadas
git restore arquivo.txt   # desfaz mudanças no arquivo
```

- git rm

```bash
# remover um arquivo do projeto e do índice
git rm arquivo.txt
```

- git add -p

```bash
# adicione as mudanças de forma mais controlada
git add -p

# y > yes confirma e segue para próxima alteração
# n > no pula para próxima alteração que deseja adiciona
```


## Histórico

- git bisect

```bash
# encontrar qual commit introduziu um bug
git bisect start

git bisect bad            # commit atual tem o bug
git bisect good v1.0      # essa versão estava OK

# o Git navega automaticamente; você testa e repete:
git bisect good           # ou: git bisect bad
```

- git diff

```bash
# ver o que mudou
git diff                  # mudanças ainda não adicionadas
git diff HEAD~1           # comparar com o commit anterior
```

- git grep

```bash
# buscar texto no código
git grep "função login"
```

- git log

```bash
# ver histórico de commits
git log --oneline         # resumido, uma linha por commit
```

- git show

```bash
# ver detalhes de um commit
git show a3f5c1b         # hash do commit
```

- git status

```bash
# ver estado atual dos arquivos
git status  
```


## Branches

- git branch

```bash
# criar ou listar branches
git branch                # lista todas as branches
git branch nova-feature   # cria uma branch nova
```

- git commit

```bash
# salvar as mudanças no histórico
git commit -m "Adiciona página de login"
```

- git merge

```bash
# unir uma branch na atual
git merge nova-feature
```

- git rebase

```bash
# reaplicar commits sobre outra base
git rebase main           # reaplica commits em cima da main
```

- git reset

```bash
# desfazer commits ou unstage arquivos
git reset HEAD~1          # desfaz o último commit (mantém arquivos)
```

- git switch

```bash
# trocar de branch
git switch nova-feature
git switch -c outra-branch  # cria e já troca
```

- git tag

```bash
# marcar uma versão
git tag v1.0.0
```


## Baixar, Aplicar e Enviar

- git fetch

```bash
# baixar atualizações sem aplicar
git fetch origin
```

- git pull

```bash
# baixar e já aplicar atualizações
git pull origin main
```

- git push

```bash
# enviar commits para o repositório remoto
git push origin main
```

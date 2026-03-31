# GitHub - Colaboração

## Forking vs Cloning

```md
Fork: cria uma cópia do repositório na sua conta GitHub
- usado para contribuir em projetos que você não tem acesso direto
- mantém vínculo com o repositório original

Clone: baixa o repositório para sua máquina local
- usado para trabalhar localmente em qualquer repositório

# Clonar um repositório
git clone https://github.com/usuario/projeto.git

# Clonar seu fork
git clone https://github.com/seu-usuario/projeto.git
```

## Issues

```md
- Usadas para reportar bugs, sugerir melhorias ou discutir tarefas
- Podem ser atribuídas a colaboradores
- Podem ser vinculadas a Pull Requests

1. Acesse o repositório no GitHub
2. Clique em "Issues" > "New Issue"
3. Adicione título, descrição, labels e responsável
```

## Pull Requests

### Collaborators

```md
- Colaboradores têm acesso direto ao repositório
- Podem criar branches e abrir PRs diretamente

1. Crie uma branch e faça suas alterações
git switch -c minha-feature
git add .
git commit -m "Descrição da mudança"
git push origin minha-feature

2. No GitHub, clique em "Compare & pull request"
3. Adicione título, descrição e revisores
```

### PR from a Fork

```md
- Usada para contribuir em repositórios que você não tem acesso direto

1. Faça o fork do repositório no GitHub
2. Clone o seu fork localmente
git clone https://github.com/seu-usuario/projeto.git

3. Crie uma branch e faça suas alterações
git switch -c minha-feature
git add .
git commit -m "Descrição da mudança"
git push origin minha-feature

4. No GitHub, acesse seu fork e clique em "Compare & pull request"
```

## Labelling Issues / PRs

```md
- Labels organizam e categorizam Issues e PRs
- Exemplos comuns: bug, enhancement, documentation, help wanted

1. Acesse a Issue ou PR
2. No painel lateral, clique em "Labels"
3. Selecione ou crie uma label
```

## Saved Replies

```md
- Respostas salvas para reutilizar em comentários frequentes

1. Acesse Settings > Saved Replies
2. Clique em "Add a saved reply"
3. Defina título e conteúdo
4. Para usar, clique no ícone de seta em qualquer caixa de comentário
```

## Mentions

```md
- Notifica usuários ou times diretamente em comentários

- Mencionar um usuário
@usuario

- Mencionar um time
@organizacao/time

- Referenciar uma Issue ou PR
#123
```

## Reactions

```md
- Reações rápidas em comentários, Issues e PRs
- Opções: 👍 👎 😄 🎉 😕 ❤️ 🚀 👀

1. Passe o cursor sobre o comentário
2. Clique no ícone de emoji que aparece
3. Selecione a reação
```

## GitHub Discussions

```md
- Espaço para conversas abertas que não são bugs ou tarefas
- Ideal para perguntas, ideias e anúncios

1. Acesse o repositório > aba "Discussions"
2. Clique em "New Discussion"
3. Escolha uma categoria: Q&A, Ideas, Announcements, General
4. Adicione título e descrição
```

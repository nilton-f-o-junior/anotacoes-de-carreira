# Git & GitHub — Guia Completo

---

## Sumário

1. [Fundamentos](#1-fundamentos)
2. [Instalação](#2-instalação)
3. [Configuração Inicial](#3-configuração-inicial)
4. [Repositórios](#4-repositórios)
5. [Fluxo de Trabalho Diário](#5-fluxo-de-trabalho-diário)
6. [Branches](#6-branches)
7. [Histórico e Inspeção](#7-histórico-e-inspeção)
8. [Estratégias de Merge](#8-estratégias-de-merge)
9. [GitHub — Primeiros Passos](#9-github--primeiros-passos)
10. [GitHub — Colaboração](#10-github--colaboração)
11. [Boas Práticas](#11-boas-práticas)

---

## 1. Fundamentos

### Git e GitHub

- **Git** — sistema de controle de versão distribuído; monitora alterações no código localmente.
- **GitHub** — plataforma em nuvem que hospeda repositórios e facilita a colaboração entre times.

### O que é Controle de Versão?

- Gerencia diferentes estágios de um projeto, permitindo recuperar versões anteriores e evitar perda de dados.
- Cria um histórico detalhado que identifica **quem** alterou **o quê** e **quando**.

### Por que usar?

- Uma única pasta limpa com todo o histórico preservado.
- Qualquer versão pode ser restaurada a qualquer momento.
- Sincronização instantânea via nuvem (GitHub, GitLab etc.).
- Identificação exata do commit que introduziu um problema.

### Git vs Sistemas Centralizados (SVN)

| | SVN (Centralizado) | Git (Distribuído) |
|---|---|---|
| Histórico | Somente no servidor | Cópia completa em cada máquina |
| Offline | Não funciona | 100% funcional |
| Velocidade | Depende da rede | Muito rápido |
| Segurança | Ponto único de falha | Redundância em múltiplos lugares |

---

## 2. Instalação

```bash
# Debian / Ubuntu
sudo apt-get install git

# Arch Linux
sudo pacman -Syu git
```

---

## 3. Configuração Inicial

### Usuário único

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

### Múltiplos usuários (pessoal + trabalho)

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

### Arquivo `~/.ssh/config`

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

### .gitignore

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

---

## 4. Repositórios

```bash
# Clonar um repositório existente
git clone https://github.com/usuario/projeto.git

# Criar um repositório novo do zero
git init meu-projeto
```

### Gerenciar remotos

```bash
git remote -v                                       # listar remotos
git remote add origin https://github.com/u/repo    # adicionar remoto
git remote remove origin                            # remover remoto
```

---

## 5. Fluxo de Trabalho Diário

```bash
# Ver estado atual dos arquivos
git status

# Adicionar arquivos para o próximo commit
git add .                  # todos os arquivos alterados
git add index.html         # um arquivo específico

# Adicionar de forma interativa (selecionar pedaços)
git add -p                 # y = confirma | n = pula
git add -i                 # modo interativo completo

# Desfazer o git add (antes do commit)
git reset index.html       # um arquivo específico
git reset --               # todos os arquivos

# Salvar as mudanças no histórico
git commit -m "Descrição da mudança"

# Enviar commits para o repositório remoto
git push origin main

# Baixar atualizações sem aplicar
git fetch origin

# Baixar e aplicar atualizações
git pull origin main
```

### Operações em arquivos

```bash
# Mover ou renomear um arquivo
git mv velho-nome.txt novo-nome.txt

# Descartar alterações não commitadas
git restore arquivo.txt

# Remover um arquivo do projeto e do índice
git rm arquivo.txt
```

---

## 6. Branches

```bash
# Listar todas as branches
git branch

# Criar uma branch nova
git branch nova-feature

# Trocar de branch
git switch nova-feature

# Criar e já trocar
git switch -c outra-branch

# Marcar uma versão
git tag v1.0.0
```

---

## 7. Histórico e Inspeção

```bash
# Ver histórico resumido de commits
git log --oneline

# Ver detalhes de um commit específico
git show a3f5c1b

# Ver o que mudou antes de commitar
git diff

# Comparar com o commit anterior
git diff HEAD~1

# Buscar texto no código
git grep "função login"

# Encontrar qual commit introduziu um bug
git bisect start
git bisect bad             # o commit atual tem o bug
git bisect good v1.0       # essa versão estava OK
# O Git navega automaticamente; teste e repita:
git bisect good            # ou: git bisect bad

# Desfazer o último commit (mantém os arquivos)
git reset HEAD~1
```

---

## 8. Estratégias de Merge

### Fast-Forward vs Non-FF

```bash
# Fast-forward — move o ponteiro sem criar commit de merge
git merge nova-feature

# Non-fast-forward — força a criação de um commit de merge
git merge --no-ff nova-feature
```

### Rebase

```bash
# Reaplicar commits da branch em cima da main
git switch nova-feature
git rebase main

# Após o rebase, fazer o merge
git switch main
git merge nova-feature
```

### Squash

```bash
# Unir todos os commits da branch em um só antes do merge
git merge --squash nova-feature
git commit -m "Descrição unificada das mudanças"
```

### Rebase Interativo

```bash
# Abre editor para reorganizar ou agrupar commits
git rebase -i main

# Comandos no editor:
# pick   → manter o commit
# squash → unir com o anterior
# drop   → remover o commit
```

### Resolvendo Conflitos

```bash
# 1. Ao dar merge, o Git indica os arquivos em conflito
git merge nova-feature

# 2. Abra os arquivos conflitantes — o conflito aparece assim:
# <<<<<<< HEAD
# código da branch atual
# =======
# código da branch que está sendo mergeada
# >>>>>>> nova-feature

# 3. Resolva manualmente, depois finalize
git add arquivo-resolvido.txt
git commit -m "Resolve conflito no merge"
```

### Cherry Pick

```bash
# Aplicar um commit específico de outra branch
git cherry-pick a3f5c1b

# Aplicar múltiplos commits
git cherry-pick a3f5c1b d4e6f2c
```

---

## 9. GitHub — Primeiros Passos

### Criar e configurar conta

```
1. Acesse https://github.com/ e clique em "Inscrever-se"
2. Faça login e acesse as configurações do perfil
3. Adicione nome, foto e bio
4. Configure seu README de perfil
```

### Criar um repositório

```
1. Clique em "New" na página inicial
2. Defina nome, descrição e visibilidade
3. Inicialize com README se for um projeto novo
```

### Clonar, alterar e enviar

```bash
git clone https://github.com/usuario/projeto.git
git add .
git commit -m "Descrição da mudança"
git push origin main
```

---

## 10. GitHub — Colaboração

### Fork vs Clone

| | Fork | Clone |
|---|---|---|
| O que faz | Cria cópia na sua conta GitHub | Baixa o repositório para sua máquina |
| Uso | Contribuir em projetos sem acesso direto | Trabalhar localmente em qualquer repositório |
| Vínculo | Mantém vínculo com o original | — |

```bash
# Clonar um repositório
git clone https://github.com/usuario/projeto.git

# Clonar seu fork
git clone https://github.com/seu-usuario/projeto.git
```

### Issues

```
- Usadas para reportar bugs, sugerir melhorias ou discutir tarefas
- Podem ser atribuídas a colaboradores e vinculadas a Pull Requests

1. Acesse o repositório > "Issues" > "New Issue"
2. Adicione título, descrição, labels e responsável
```

### Pull Requests — Colaboradores com acesso direto

```bash
# 1. Crie uma branch e faça suas alterações
git switch -c minha-feature
git add .
git commit -m "Descrição da mudança"
git push origin minha-feature

# 2. No GitHub, clique em "Compare & pull request"
# 3. Adicione título, descrição e revisores
```

### Pull Requests — Via Fork

```bash
# 1. Faça o fork no GitHub
# 2. Clone o seu fork
git clone https://github.com/seu-usuario/projeto.git

# 3. Crie uma branch e faça as alterações
git switch -c minha-feature
git add .
git commit -m "Descrição da mudança"
git push origin minha-feature

# 4. No GitHub, acesse seu fork e clique em "Compare & pull request"
```

### Labels, Mentions e Reações

```md
# Labels — organizam e categorizam Issues e PRs
# Exemplos: bug, enhancement, documentation, help wanted
# Acesse a Issue ou PR > painel lateral > "Labels"

# Mentions — notifica usuários ou times
@usuario              # mencionar usuário
@organizacao/time     # mencionar time
#123                  # referenciar Issue ou PR

# Reações em comentários
👍 👎 😄 🎉 😕 ❤️ 🚀 👀
```

### Saved Replies

```
1. Acesse Settings > Saved Replies
2. Clique em "Add a saved reply"
3. Defina título e conteúdo
4. Para usar, clique no ícone de seta em qualquer caixa de comentário
```

### GitHub Discussions

```
- Espaço para conversas abertas: perguntas, ideias, anúncios
1. Acesse o repositório > aba "Discussions" > "New Discussion"
2. Escolha uma categoria: Q&A, Ideas, Announcements, General
3. Adicione título e descrição
```

---

## 11. Boas Práticas

### Mensagens de Commit — Conventional Commits

```bash
# Estrutura
# <tipo>(<escopo>): <descrição curta>
# [corpo opcional — explica o *porquê*, não o *o quê*]
# [rodapé opcional — breaking changes, issues fechadas]
```

**Tipos comuns:**

| Tipo | Uso |
|---|---|
| `feat` | Nova funcionalidade |
| `fix` | Correção de bug |
| `docs` | Documentação |
| `style` | Formatação, lint |
| `refactor` | Código sem alteração de funcionalidade |
| `test` | Adição ou correção de testes |
| `chore` | Build, dependências, configurações |

```bash
# Evitar
git commit -m "ajustes"
git commit -m "fix"
git commit -m "arrumei o bug do login"

# Preferir
git commit -m "feat(auth): adiciona login com Google OAuth"
git commit -m "fix(api): corrige timeout em chamadas lentas"
git commit -m "docs(readme): atualiza instruções de setup"
```

### Nomenclatura de Branches

| Prefixo | Uso | Base |
|---|---|---|
| `feature/` | Nova funcionalidade | main ou develop |
| `bugfix/` | Correção de bug | main ou develop |
| `hotfix/` | Correção urgente em produção | main |
| `release/` | Preparação de nova versão | develop |

**Regras essenciais:**
- Use apenas letras minúsculas, números e hífens.
- Descrição curta (3 a 5 palavras).
- Delete a branch imediatamente após o merge.

### Pull Request — Checklist

```md
## O que faz este PR?
Descrição clara e objetiva da mudança.

## Por que essa mudança é necessária?
Contexto e problema que resolve.

## Como foi testado?
- [ ] Testes unitários passando
- [ ] Testado localmente

## Referências
Closes #42
```

**Boas práticas:**
- Mantenha PRs pequenos e focados.
- Vincule à issue com `Closes #N`.
- Marque como **Draft** enquanto estiver em andamento.
- Nunca force merge com revisões abertas.

### Code Review — Prefixos para Comentários

| Prefixo | Significado |
|---|---|
| `nit:` | Detalhe menor — não bloqueia aprovação |
| `suggestion:` | Sugestão opcional de melhoria |
| `question:` | Dúvida genuína sobre a implementação |
| `blocker:` | Problema crítico — deve ser resolvido antes do merge |
| `praise:` | Destaque positivo — fundamental para a moral do time |

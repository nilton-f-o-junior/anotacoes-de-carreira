# Git — Fluxo Diário

---

## 1. Começando o dia

```bash
# Clonar o projeto (primeira vez)
git clone https://github.com/usuario/projeto.git

# Atualizar a branch principal com o que tem no remoto
git pull origin main
```

---

## 2. Criando sua branch de trabalho

```bash
# Criar e já entrar na branch
git switch -c feature/minha-tarefa

# Ver em qual branch você está
git branch
```

---

## 3. Durante o desenvolvimento

```bash
# Ver o que foi modificado
git status

# Ver as mudanças linha a linha antes de adicionar
git diff

# Adicionar tudo
git add .

# Adicionar um arquivo específico
git add src/componente.js

# Salvar com mensagem
git commit -m "feat: adiciona botão de login"
```

---

## 4. Mantendo sua branch atualizada com a main

```bash
# Trazer as atualizações da main para dentro da sua branch
git switch main
git pull origin main
git switch feature/minha-tarefa
git rebase main
```

---

## 5. Enviando o trabalho

```bash
# Subir sua branch para o remoto (primeira vez)
git push -u origin feature/minha-tarefa

# Subir nas próximas vezes
git push
```

---

## 6. Inspecionando o histórico

```bash
# Ver commits recentes de forma resumida
git log --oneline

# Ver quem alterou cada linha de um arquivo
git blame src/componente.js

# Ver detalhes de um commit específico
git show a3f5c1b
```

---

## 7. Corrigindo antes do commit

```bash
# Desfazer o git add de um arquivo
git restore --staged arquivo.js

# Descartar as mudanças locais de um arquivo
git restore arquivo.js

# Corrigir a mensagem do último commit
git commit --amend -m "feat: mensagem corrigida"
```

---

## 8. Corrigindo após o commit

```bash
# Desfazer o último commit (mantém as alterações nos arquivos)
git reset HEAD~1

# Stash — guardar mudanças temporariamente sem commitar
git stash            # guarda
git stash pop        # recupera
```

---

## 9. Finalizando a tarefa

```bash
# Após o PR ser aprovado e mergeado, limpar localmente
git switch main
git pull origin main
git branch -d feature/minha-tarefa
```

---

## Referência rápida

| Comando | O que faz |
|---|---|
| `git status` | Ver arquivos modificados |
| `git diff` | Ver mudanças linha a linha |
| `git add .` | Adicionar tudo para o commit |
| `git commit -m ""` | Salvar com mensagem |
| `git push` | Enviar para o remoto |
| `git pull` | Baixar e aplicar atualizações |
| `git switch -c` | Criar e entrar em uma branch |
| `git log --oneline` | Ver histórico resumido |
| `git stash` | Guardar mudanças sem commitar |
| `git rebase main` | Atualizar branch com a main |
| `git reset HEAD~1` | Desfazer último commit |
| `git restore arquivo` | Descartar mudanças locais |

--- 

## Prefixos de Commit — Conventional Commits

| Prefixo | Uso | Exemplo |
|---|---|---|
| `feat` | Nova funcionalidade | `feat(auth): adiciona login com Google` |
| `fix` | Correção de bug | `fix(api): corrige timeout em chamadas lentas` |
| `docs` | Documentação | `docs(readme): atualiza instruções de setup` |
| `style` | Formatação, lint (sem mudança de lógica) | `style: aplica prettier nos arquivos` |
| `refactor` | Melhoria de código sem alterar comportamento | `refactor(user): simplifica lógica de validação` |
| `test` | Adição ou correção de testes | `test(auth): adiciona testes de login` |
| `chore` | Build, dependências, configurações | `chore: atualiza versão do eslint` |
| `perf` | Melhoria de performance | `perf(query): otimiza busca no banco` |
| `ci` | Configuração de CI/CD | `ci: adiciona workflow de deploy` |
| `revert` | Reverter um commit anterior | `revert: desfaz feat(auth): login com Google` |

---

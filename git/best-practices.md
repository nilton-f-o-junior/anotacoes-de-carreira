# Git & GitHub

Boas PráticasGuia de boas práticas para times que levam o histórico a sério, baseado nos padrões de Conventional Commits e fluxos de colaboração profissionais.

## Commit Messages

Adote o padrão Conventional Commits para mensagens legíveis e processáveis por ferramentas.Estrutura do Commit

```bash
# <tipo>(<escopo>): <descrição curta>
# [corpo opcional — explica o *porquê*, não o *o quê*]
# [rodapé opcional — breaking changes, issues fechadas]
```

## Tipos Comuns:

```bash
# feat: nova funcionalidade
# fix: correção de bug
# docs: documentaçãos
# tyle: formatação, lint
# refactor: código que não altera funcionalidade
# test: adição ou correção de testes
# chore: build, dependências, configurações
```

## Exemplos

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

## Branch Naming

Branches com nomes claros facilitam a navegação e automações de CI/CD.

Tipos de Branch

- feature/: nova funcionalidade (Base: main ou develop);
- bugfix/: correção de bug (Base: main ou develop);
- hotfix/: correção urgente em produção (Base: main);
- release/: preparação de nova versão (Base: develop).

Regras Essenciais

- Use apenas letras minúsculas, números e hífens.
- Descrição curta (3 a 5 palavras).
- Delete a branch imediatamente após o merge.

## Pull Request (PR) Guidelines

Um PR bem escrito acelera a revisão e documenta decisões para o futuro.

- O que faz este PR? Descrição clara e objetiva da mudança.
- Por que essa mudança é necessária? Contexto e problema que resolve.
- Como foi testado?
  - [ ] Testes unitários passando
  - [ ] Testado localment
- Referências: Closes #42

Boas Práticas

- Mantenha PRs pequenos e focados.
- Vincule à issue correspondente usando Closes #N.
- Marque como Draft enquanto estiver em andamento.
- Nunca force merge com comentários de revisão abertos.

## Code Review

Code review é sobre aprendizado e qualidade coletiva, não sobre ataques pessoais.

Prefixos para Comentários

- nit:: Detalhe menor (não bloqueia aprovação).
- suggestion:: Sugestão opcional de melhoria.
- question:: Dúvida genuína sobre a implementação.
- blocker:: Problema crítico que deve ser resolvido antes do merge.
- praise:: Destaque positivo (fundamental para a moral do time).

## Clean Git History

O histórico é documentação. O comando git log deve contar a história do projeto, não listar tentativas e erros.

Rebase Interativo

```bash
# Abre editor para reorganizar/agrupar commits
git rebase -i main

# Comandos no editor:
# pick   -> manter commit
# squash -> unir com o anterior (limpa o histórico)
# drop   -> remover commit indesejado
```

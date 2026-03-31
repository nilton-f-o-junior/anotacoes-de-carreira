# Estrátegias de Merge

## Fast-Forward vs Non-FF

```bash
# fast-forward: move o ponteiro da branch sem criar commit de merge
git merge nova-feature

# non-fast-forward: força a criação de um commit de merge
git merge --no-ff nova-feature
```

## Rebase

```bash
# reaplicar commits da branch em cima da main
git switch nova-feature
git rebase main

# após o rebase, fazer o merge
git switch main
git merge nova-feature
```

## Squash

```bash
# unir todos os commits da branch em um só antes de fazer o merge
git merge --squash nova-feature
git commit -m "Descrição unificada das mudanças"
```

## Handling Conflicts

```bash
# 1. ao dar merge, o Git indica os arquivos em conflito
git merge nova-feature

# 2. abra os arquivos conflitantes e resolva manualmente
# os conflitos aparecem assim:
# <<<<<<< HEAD
# código da branch atual
# =======
# código da branch que está sendo mergeada
# >>>>>>> nova-feature

# 3. após resolver, adicione os arquivos e finalize
git add arquivo-resolvido.txt
git commit -m "Resolve conflito no merge"
```

## Cherry Picking

```bash
# aplicar um commit específico de outra branch na branch atual
git cherry-pick a3f5c1b

# aplicar múltiplos commits
git cherry-pick a3f5c1b d4e6f2c
```

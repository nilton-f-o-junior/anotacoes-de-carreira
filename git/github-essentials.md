# GitHub - Primeiros Passos

## 1. Criar uma conta

```
1. Acesse https://github.com/
2. Clique em Inscrever-se
3. Como alternativa, clique em Continuar com o Google
4. Siga as instruções para criar sua conta
```

## 2. Configurar o perfil

```
1. Faça login em https://github.com/
2. Acesse as configurações do perfil
3. Adicione nome, foto e bio
4. Configure seu README de perfil
# Guia completo: https://docs.github.com/pt/get-started/start-your-journey/setting-up-your-profile
```

## 3. Criar um repositório

```
1. Clique em "New" na página inicial
2. Defina nome, descrição e visibilidade
3. Inicialize com README se for um projeto novo
# Guia completo: https://docs.github.com/pt/repositories/creating-and-managing-repositories/quickstart-for-repositories
```

## 4. Clonar o repositório localmente

```bash
git clone https://github.com/usuario/projeto.git
# Guia completo: https://docs.github.com/pt/repositories/creating-and-managing-repositories/cloning-a-repository
```

## 5. Fazer alterações e enviar

```bash
git add .
git commit -m "Descrição da mudança"
git push origin main
# Guia completo: https://docs.github.com/pt/get-started/using-git/pushing-commits-to-a-remote-repository
```

## 6. Buscar atualizações

```bash
git fetch origin        # baixa sem mesclar
git pull origin main    # baixa e mescla
# Guia completo: https://git-scm.com/docs/git-fetch
```

## 7. Gerenciar repositórios remotos

```bash
git remote -v                                      # listar remotos
git remote add origin https://github.com/u/repo   # adicionar remoto
git remote remove origin                           # remover remoto
# Guia completo: https://docs.github.com/pt/get-started/git-basics/managing-remote-repositories
```

# config

- usuário único

```bash
# Usuário
git config --global user.name "Seu Nome"

# E-mail
git config --global user.email "[email protected]"

# Gerar chave SSH
ssh-keygen -t ed25519 -C "[email protected]"
# Pressione Enter nas perguntas para usar os valores padrão

# Exibir a chave pública para copiar e colar no GitHub
cat ~/.ssh/id_ed25519.pub

# GitHub > Settings > SSH and GPG keys > New SSH key > Cole e salve
```

- múltiplos usuários (pessoal + trabalho)

```bash
# conta pessoal
ssh-keygen -t ed25519 -C "[email protected]"
# Quando solicitado o arquivo, digite: id_pessoal

# Generate public/private ed25519 key pair.
# Enter file in which to save the key (/c/Users/pessoal.ssh/id_ed25519): id_pessoal
# Enter passphrase (empty for no passphrase):

# Pressione Enter para deixar a passphrase em branco (ou defina uma)
```

```bash
# conta trabalho
ssh-keygen -t ed25519 -C "[email protected]"
# Quando solicitado o arquivo, digite: id_trabalho

# Generate public/private ed25519 key pair.
# Enter file in which to save the key (/c/Users/pessoal.ssh/id_ed25519): id_trabalho
# Enter passphrase (empty for no passphrase):

# Pressione Enter para deixar a passphrase em branco (ou defina uma)
```

```bash
# chaves ssh
eval "$(ssh-agent -s)"
# Agent pid XXXX

ssh-add ~/.ssh/id_pessoal
# Identity added: id_pessoal

ssh-add ~/.ssh/id_trabalho
# Identity added: id_trabalho
```
  
```bash
# chaves publicas
# Copie cada chave e adicione em:
# GitHub > Settings > SSH and GPG keys > New SSH key > Cole e salve

cat ~/.ssh/id_pessoal.pub
cat ~/.ssh/id_trabalho.pub
```

# config file

```bash
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
# teste de conexão
ssh -T [email protected]
# Hi <usuario-pessoal>! You've successfully authenticated...

ssh -T [email protected]
# Hi <usuario-trabalho>! You've successfully authenticated...
```

```bash
# clone
git clone [email protected]:usuario-pessoal/repositorio.git
git clone [email protected]:usuario-trabalho/repositorio.git
```

## .gitignore

 O arquivo .gitignore instrui o Git a não rastrear determinados arquivos ou pastas, como dependências, variáveis de ambiente e arquivos gerados automaticamente. 

```bash
# Ignorar uma pasta inteira (ex: dependências do Node)
node_modules/

# Ignorar arquivos de configuração sensíveis (senhas)
.env

# Ignorar todos os arquivos que terminam com .log
*.log

# Ignorar arquivos específicos do sistema operacional
Thumbs.db
.DS_Store

# "!" Abre a exceção para o arquivo específico
!config_fixo.log
```

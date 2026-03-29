# Aprenda o Básico

- Git e GitHub
- O que é o Controle de Versão?
- Por que usar o Controle de Versão?
- Git vs Outros VCS
- Instalar o Git Localmente

## Git e GitHub

- Git: sistema de controle de versão para monitorar alterações no código localmente;
- GitHub: plataforma na nuvem que hospeda repositórios e facilita a colaboração entre múltiplos desenvolvedores.

## O que é o Controle de Versão?

- Versionamento: prática de gerenciar diferentes estágios de um projeto, permitindo recuperar versões anteriores e evitar perda de dados;
- Controle de versão: cria um histórico detalhado que identifica quem alterou o quê e quando, garantindo a integridade do código-fonte.

## Por que usar o Controle de Versão?

- Uma única pasta limpa com todo o histórico oculto;
- Tudo pode ser recuperado no histórico;
- Sincronização instantânea via nuvem (GitHub/GitLab);
- Identificação exata do commit que quebrou o código.

## Git vs Outros VCS

- Sistemas Centralizados (SVN): o histórico completo fica apenas em um servidor. Se você estiver sem internet, não consegue ver versões antigas nem salvar seu progresso. Se o servidor pifar e não houver backup, o histórico do projeto morre;

- Git (Distribuído): todo desenvolvedor tem uma cópia idêntica e completa de todo o histórico no seu próprio computador. Você trabalha 100% offline, é muito mais rápido e o projeto está seguro em múltiplos lugares ao mesmo tempo.

## Instalar o Git Localmente

`debian/ubuntu`

```bash
sudo apt-get install git
```

`arch`

```bash
sudo pacman -Syu git
```



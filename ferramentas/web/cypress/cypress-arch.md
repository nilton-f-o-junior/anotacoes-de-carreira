# Cypress

Comentários de um usuário arch no seu dia mais comum!

1. Pré-requisitos: Node.js e npm

```bash
sudo pacman -S nodejs npm  
```

2. Versões (Cypress requer Node >= 18)

```bash
node -v
npm -v
```

3. Instalar dependências do sistema

```bash
sudo pacman -S gtk3 libnotify nss libxtst xorg-server-xvfb at-spi2-core libxss alsa-lib unzip xorg-xwayland
```

4. Instalar o Cypress no projeto

```bash
npm install cypress --save-dev 
```

5. Instalar

```bash
npx cypress install  
```

6. Abrir o cypress

```bash
ELECTRON_OZONE_PLATFORM_HINT=auto npx cypress open
```

> Infelizmente o Electron não conversa muito bem com wayland, então acabamos burlando isso da forma como conseguimos, podemos criar também uma janela com Xwayland e depois abrir o Cypress nela. A gente faz o que pode em TWMs


7. Inicia o Xwayland

```bash
Xwayland :0 &
```

8. Abre o cypress

```bash
DISPLAY=:0 npx cypress open
```

# Cypress

## Configurando

Inicia o Cypress:

```bash
# Normal
npx cypress open

# Arch + TWM (necessário para ambientes com Wayland)
ELECTRON_OZONE_PLATFORM_HINT=auto npx cypress open
```

Estrutura de pastas e arquivos gerada após a instalação:

```
cypress/  cypress.config.js  node_modules/  package.json  package-lock.json
```

Crie a pasta de testes dentro de `cypress/`:

```bash
cd cypress
mkdir e2e
```

Por padrão, o Cypress procura os testes dentro da pasta `e2e`. Caso queira usar outro nome, altere o `specPattern` no arquivo de configuração:

```javascript
// cypress.config.js
const { defineConfig } = require("cypress");

module.exports = defineConfig({
  e2e: {
    // Agora o Cypress vai procurar os testes dentro da pasta 'testes'
    specPattern: 'cypress/testes/**/*.cy.{js,jsx,ts,tsx}',

    setupNodeEvents(on, config) {
      // seus plugins aqui
    },
  },
});
```

> Atenção: se alterar o `specPattern`, lembre-se de criar a pasta correspondente (`cypress/testes/`) e usá-la em todos os arquivos de teste.

Ao criar o arquivo de testes, use o nome daquilo que deseja testar:

```
login.cy.js
```

Para ativar o autocomplete e o IntelliSense dos comandos `cy.` na sua IDE, crie o arquivo `jsconfig.json` na raiz do projeto:

```json
{
  "include": ["./node_modules/cypress", "cypress/**/*.js"]
}
```

O Cypress usa o Electron por padrão, mas você pode configurar outro navegador. Caso precise desativar restrições de segurança entre origens diferentes, consulte a documentação oficial:

> [Cross-origin testing — Set chromeWebSecurity to false](https://docs.cypress.io/app/guides/cross-origin-testing#Set-chromeWebSecurity-to-false)

---

## Estrutura básica de um teste

O padrão **AAA** divide cada teste em três etapas:

1. **Arrange (Organizar):** define o estado inicial e os dados de entrada.
   - Exemplo: definir que os números são 5 e 3.

2. **Act (Agir):** executa a ação que está sendo testada.
   - Exemplo: chamar a função de soma e guardar o resultado.

3. **Assert (Verificar):** compara o resultado obtido com o valor esperado.
   - Exemplo: verificar se o resultado é igual a 8.

```javascript
test('Soma de 2 + 3', () => {
  // 1. Arrange
  const num1 = 2;
  const num2 = 3;
  const esperado = 5;

  // 2. Act
  const resultado = Soma(num1, num2);

  // 3. Assert
  expect(resultado).toBe(esperado);
});
```

---

## Criando testes

O Cypress é usado para testes automatizados de ponta a ponta (E2E). Assim como nos testes manuais, é importante ter os cenários escritos previamente.

### 1. Crie o arquivo de teste

```
login.cy.js
```

### 2. Escolha os seletores corretos

Para localizar os elementos no HTML, priorize os seguintes atributos nesta ordem:

- `data-testid`
- `id`
- `class`

O ideal é adicionar `data-testid` nos elementos que serão testados, pois esse atributo é exclusivo para testes e não interfere no restante do código.

### 3. Estrutura do arquivo de teste

```javascript
describe('Login', () => {

  it('Realizar login com sucesso', () => {
    // Arrange
    cy.visit('http://localhost:3000/')

    // Act


    // Assert

  })
})
```

Entendendo a estrutura:

- **`describe('rótulo', () => {})`**: agrupa os testes relacionados a uma mesma funcionalidade.
- **`it('descrição', () => {})`**: descreve um cenário específico dentro do grupo.
- **`cy.`**: prefixo para acessar os comandos do Cypress: `cy.visit`, `cy.get`, `cy.type`, `cy.click`, entre outros.

---

## Testes na prática

### Teste de botão e redirecionamento de URL

O teste a seguir verifica se um botão na página redireciona corretamente para uma URL específica.

**HTML**

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <title>Google</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <button class="btn" data-testid="btn-google" onclick="location.href='https://www.google.com'">Google</button>
</body>
</html>
```

**CSS**

```css
body {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  margin: 0;
}

.btn {
  padding: 12px 32px;
  background: #4285F4;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-family: sans-serif;
  font-size: 16px;
  cursor: pointer;
}
```

Para executar os testes localmente, suba um servidor estático na raiz do projeto:

```bash
python3 -m http.server 8000
```

#### Cenário 1: Deve redirecionar para o Google (teste que deve passar)

```javascript
describe('Home', () => {

  it('Deve redirecionar para o Google ao clicar no botão', () => {

    // Arrange
    cy.visit('http://0.0.0.0:8000/')

    // Act
    cy.get('[data-testid="btn-google"]').click()

    // Assert
    cy.url().should('eq', 'https://www.google.com/')

  })
})
```

#### Cenário 2: Não deve permanecer na página inicial (teste que deve falhar)

Este teste é intencionalmente escrito para **falhar**. O objetivo é demonstrar como o Cypress reporta erros: após o clique no botão, a URL muda para o Google, mas o teste verifica se ela permanece na página inicial.

```javascript
describe('Home', () => {

  it('Não deve permanecer na página inicial ao clicar no botão', () => {

    // Arrange
    cy.visit('http://0.0.0.0:8000/')

    // Act
    cy.get('[data-testid="btn-google"]').click()

    // Assert 
    cy.url().should('eq', 'http://0.0.0.0:8000/')

  })
})
```

Nota: o cypress tem algumas formas de complementar essa verificação feita no Cenário 2:

Verificar que não ficou na página original:

```javascript
cy.url().should('not.include', '0.0.0.0:8000')
```

- **should('not.eq', url_atual)**: fluxo de login. Você não sabe se o sistema vai redirecionar para /dashboard, /home ou /feed dependendo do perfil do usuário, mas precisa garantir que ele saiu da página de login.

Verificar que foi para algum lugar externo (sem saber exatamente qual):

```javascript
cy.url().should('not.eq', 'http://0.0.0.0:8000/')
```

- **should('not.include', 'checkout')**: fluxo de cancelamento de compra. Você quer garantir que ao cancelar, o usuário não avançou para a página de pagamento, sem se importar exatamente para onde voltou.

Verificar apenas parte da URL, útil quando a URL pode ter parâmetros:

```javascript
cy.url().should('include', 'google.com')
```

- **should('include', 'google.com')**: ambientes diferentes (dev, staging, produção) onde a URL base muda, mas o domínio de destino é sempre o mesmo. Testar https://staging.google.com/... e https://google.com/... com eq exigiria dois testes; com include um só resolve.


### Teste de Login

Vamos agora visualizar agora como funciona o teste de login.

`index.html`

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Login</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>

<div class="card" data-testid="login-card">
  <h1>Login</h1>

  <label for="username">Usuário</label>
  <input id="username" type="text" data-testid="input-username" autocomplete="username">

  <label for="password">Senha</label>
  <input id="password" type="password" data-testid="input-password" autocomplete="current-password">

  <p id="error" data-testid="error-message">Usuário ou senha inválidos.</p>

  <button data-testid="btn-login" onclick="login()">Entrar</button>
</div>

<script src="login.js"></script>
</body>
</html>
```

`dashboard.html`

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Dashboard</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>

  <div class="success-page" data-testid="success-page">
    <div class="success-box">
      <div class="success-icon">✓</div>
      <p class="success-message" data-testid="success-message">Login efetuado com sucesso!</p>
    </div>
  </div>

</body>
</html>
```

`css`

```css
* { margin: 0; padding: 0; box-sizing: border-box; }

body {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f2f5;
  font-family: 'Courier New', monospace;
}

.card {
  background: #fff;
  border: 1px solid #dde1e7;
  padding: 2.5rem;
  width: 320px;
  box-shadow: 0 2px 12px rgba(0,0,0,.06);
}

h1 {
  color: #1a1a2e;
  font-size: 1.1rem;
  letter-spacing: .2em;
  text-transform: uppercase;
  margin-bottom: 2rem;
  border-bottom: 1px solid #dde1e7;
  padding-bottom: 1rem;
}

label {
  display: block;
  color: #888;
  font-size: .7rem;
  letter-spacing: .15em;
  text-transform: uppercase;
  margin-bottom: .4rem;
}

input {
  width: 100%;
  background: #f8f9fb;
  border: 1px solid #dde1e7;
  color: #1a1a2e;
  padding: .65rem .8rem;
  font-family: inherit;
  font-size: .9rem;
  margin-bottom: 1.2rem;
  outline: none;
  transition: border-color .2s;
}

input:focus { border-color: #3b82f6; }

#error {
  color: #dc2626;
  font-size: .75rem;
  margin-bottom: 1rem;
  display: none;
}

button {
  width: 100%;
  background: #3b82f6;
  color: #fff;
  border: none;
  padding: .75rem;
  font-family: inherit;
  font-size: .8rem;
  letter-spacing: .15em;
  text-transform: uppercase;
  cursor: pointer;
  transition: background .2s;
}

button:hover { background: #2563eb; }

.success-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  background: #f0f2f5;
}

.success-box {
  background: #fff;
  border: 1px solid #dde1e7;
  padding: 2.5rem;
  width: 320px;
  box-shadow: 0 2px 12px rgba(0,0,0,.06);
  text-align: center;
}

.success-icon {
  font-size: 2.5rem;
  color: #16a34a;
  margin-bottom: 1rem;
}

.success-message {
  color: #16a34a;
  font-size: 1rem;
  letter-spacing: .1em;
  text-transform: uppercase;
}
```

`js`

```javascript
function login() {
  const ok = document.getElementById('username').value === 'admin'
          && document.getElementById('password').value === 'admin';

  if (ok) {
    // Redireciona exatamente para a rota que o teste espera
    window.location.href = '/dashborad';
  } else {
    document.getElementById('error').style.display = 'block';
  }
}
```

Agora vamos visualizar os testes

#### Cenário 1: Login feito os dados válidos

```javascript
describe('Login', () => {

  it('Realizar login com sucesso', () => {

    // Arrange
    cy.visit('http://0.0.0.0:8000/')

    // Act
    cy.get('[data-testid="input-username"]').type('admin')
    cy.get('[data-testid="input-password"]').type('admin')
    cy.get('[data-testid="btn-login"]').click()

    // Assert
    cy.url().should('eq', 'http://0.0.0.0:8000/dashboard')
  })
})
```

#### Cenário 2: Erro nas credenciais de login

```javascript
describe('Login', () => {

  it('Login com credenciais inválidas', () => {

    // Arrange
    cy.visit('http://0.0.0.0:8000/')

    // Act
    cy.get('[data-testid="input-username"]').type('user')
    cy.get('[data-testid="input-password"]').type('pass')
    cy.get('[data-testid="btn-login"]').click()

    // Assert
    cy.get('[data-testid="error-message"]')
    .should(
      'contain.text',
      'Usuário ou senha inválidos.'
      
    )
  })
})
```

Note:

- **it.only**: ao usar esse comando, apenas esse teste será visualizado na página, os demais serão ignorados.

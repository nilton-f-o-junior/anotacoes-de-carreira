# Cypress

## Configurando

Inicia o cypress

```bash
# normal
npx cypress open

# arch + twm
ELECTRON_OZONE_PLATFORM_HINT=auto npx cypress open
```

Estruturas de pastas e arquivos

```bash
cypress  cypress.config.js  node_modules  package.json  package-lock.json
```

Crie uma pasta dentro da cypress

```bash
cd cypress
mkdir e2e
```

O cypress por padrão defini a pasta e2e como a pasta aonde vão os testes, porém podemos mudar o nome para testes ou outro:

```bash
const { defineConfig } = require("cypress");

module.exports = defineConfig({
  e2e: {
    // Agora o Cypress vai procurar os testes dentro da pasta 'testes'
    specPattern: 'cypress/teste/**/*.cy.{js,jsx,ts,tsx}',
    
    setupNodeEvents(on, config) {
      // seus plugins aqui
    },
  },
});
```

Ao criar o arquivo de testes, use o nome daquilo que deseja testar

```bash
# nome.cy.js
login.cy.js
```

Sua IDE pode precisar de um pequeno ajuste

```bash
# crie o arquivo
jsconfig.json

# adiciona a config
{
  "include": ["./node_modules/cypress", "cypress/**/*.js"]
}
```

O cypress usa o Electron por padrão, porém você pode escolher outro navegador, mas caso precise fazer algumas configurações, segue o link:

```bash
https://docs.cypress.io/app/guides/cross-origin-testing#Set-chromeWebSecurity-to-false
```

## O básico de testes é a estrutura:

AAA

1. Arrange (Organizar):
  - O que fazer: Instanciar a classe que será testada e definir os valores de entrada.
    - Exemplo: Se vou testar uma calculadora, aqui eu defino que os números são: 5 e 3.

2. Act (Agir):
  - O que fazer: Chamar a função e armazenar o resultado em uma variável.
    - Exemplo: Somar os números 5 e 3 e guardar o resultado.

3. Assert (Afirmar/Verificar):
  - O que fazer: Comparar o resultado obtido com o valor esperado.
    - Exemplo: Verificar se o resultado da soma é igual a 8.


Exemplo:

```javascript
// Teste: deve somar dois números corretamente
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

## Criando testes

O cypress é usado para fazer testes automatizados, assim como nós testes manuais, você precisa ter os testes escritos previamente e saber o que vai precisar testar.

1. Primeiros criamos o arquivo

```javascript
login.cy.js
```

2. Tags  usadas

Para fazer os testes usamos os tags presentes do html, porém se uma tag se repete é mais dificil escrever o teste, logo priorizamos algumas tags, sendo elas:

- data-testid
- id
- class

O ideal, é ao escrever o código adicionar a tag `data-testid` nós itens que serão testados, 

3. Estrutura do arquivo

```javascript
describe('Login', () => {

  it('Realizar login com sucesso', () => {
    // arrange
    cy.visit('')

    // art


    // assert
    
  })
})
```

# Introdução a Testes de Software

## Pirâmide de Testes

```
          / \
         /   \
        /     \
       /  E2E  \        ← Topo: mais lentos, mais caros, menos quantidade
      /---------\
     / Integração\      ← Meio: validam a comunicação entre módulos
    /-------------\
   /   Unitários   \    ← Base: mais rápidos, mais baratos, mais quantidade
  /_________________\
```

A pirâmide orienta a **proporção ideal** de testes: a base deve ter a maior quantidade, pois são rápidos e baratos; o topo deve ter poucos, pois são lentos e custosos.

---

## Tipos de Testes

### Caixa Preta (*Black Box*)

Testa o **comportamento externo** do software, sem conhecer a implementação interna.  

O testador fornece entradas e verifica as saídas, sem acesso ao código-fonte.  

- **Foco:** o que o sistema faz.  
- **Exemplo:** preencher um formulário e verificar se a mensagem de erro aparece corretamente.

### Caixa Branca (*White Box*)

Testa a **estrutura interna** do código, com pleno conhecimento da implementação.  

O testador analisa caminhos lógicos, condições e fluxos do código.  

- **Foco:** como o sistema faz.  
- **Exemplo:** verificar se todos os ramos de um `if/else` são cobertos.

### Caixa Cinza (*Grey Box*)

Combinação das duas abordagens: o testador tem conhecimento **parcial** da implementação. Comum em testes de integração e segurança.

---

## Os 4 Tipos Principais de Testes

| Tipo | Objetivo | Nível na Pirâmide |
|---|---|---|
| **Unitário** | Verificar a lógica de uma unidade isolada (função, método, classe) | Base |
| **Integração** | Validar a comunicação entre módulos ou serviços | Meio |
| **Funcional** | Verificar se o sistema atende aos requisitos funcionais | Topo |
| **Regressivo** | Re-testar o software após mudanças para garantir que nada foi quebrado | Todos os níveis |

### Detalhamento

- **Unitário:** rápido, isolado, usa mocks/stubs para substituir dependências externas.
- **Integração:** valida contratos entre APIs, banco de dados, filas de mensagens, etc.
- **Funcional (E2E):** simula o fluxo completo do usuário no sistema.
- **Regressivo:** geralmente automatizado; executado em pipelines de CI/CD a cada novo commit.

---

## Outros Tipos de Testes

### Performance
- **Teste de Carga (*Load Test*):** avalia o comportamento do sistema sob carga esperada.
- **Teste de Estresse (*Stress Test*):** avalia o comportamento além dos limites normais, até o ponto de falha.
- **Teste de Pico (*Spike Test*):** avalia a resposta a aumentos súbitos e extremos de carga.
- **Teste de Resistência (*Soak/Endurance Test*):** avalia o comportamento sob carga sustentada por longo período.

### Qualidade
- **Teste de Usabilidade:** avalia a experiência do usuário (UX) com pessoas reais.
- **Teste de Acessibilidade:** verifica conformidade com padrões como WCAG.
- **Teste de Segurança (*Penetration Test*):** identifica vulnerabilidades e falhas de segurança.
- **Teste de Compatibilidade:** verifica o funcionamento em diferentes navegadores, SOs e dispositivos.
- **Teste de Contrato (*Contract Testing*):** valida que a comunicação entre serviços respeita um contrato definido (ex: Pact).

---

## Ferramentas

### Testes Unitários e de Integração (Caixa Branca)
| Linguagem | Ferramenta |
|---|---|
| JavaScript/TypeScript | Jest, Vitest, Mocha |
| Python | pytest, unittest |
| Java | JUnit, Mockito |
| C# | NUnit, xUnit |
| Go | testing (nativo) |
| Rust | cargo test (nativo), rstest |

### Testes E2E / Funcionais (Caixa Preta)
| Ferramenta | Observação |
|---|---|
| **Cypress** | popular para apps web, executa no navegador |
| **Playwright** | multi-browser, suporta mobile, mantido pela Microsoft |
| **Selenium** | pioneiro, multi-linguagem, multi-browser |
| **Puppeteer** | automação de Chrome/Chromium via Node.js |

### Testes de Carga e Performance
| Ferramenta | Observação |
|---|---|
| **Gatling** | baseado em Scala, relatórios detalhados |
| **k6** | scripts em JavaScript, fácil integração com CI/CD |
| **Apache JMeter** | interface gráfica, muito utilizado em empresas |
| **Locust** | scripts em Python, fácil de escalar |

### Testes de API
| Ferramenta | Observação |
|---|---|
| **Postman / Newman** | criação manual e automação de coleções |
| **REST Assured** | biblioteca Java para testes de API REST |
| **Insomnia** | alternativa ao Postman |

---

## Conceitos Complementares

- **TDD (*Test-Driven Development*):** escreve-se o teste antes do código de produção. Ciclo: Red → Green → Refactor.
- **BDD (*Behavior-Driven Development*):** testes escritos em linguagem natural (Gherkin: *Given / When / Then*). Ferramentas: Cucumber, Behave.
- **Cobertura de Código (*Code Coverage*):** métrica que indica o percentual do código exercitado pelos testes. Não é garantia de qualidade sozinha.
- **Mock / Stub / Spy:** técnicas para isolar unidades de teste substituindo dependências por objetos controlados.
- **CI/CD:** testes automatizados são parte fundamental de pipelines de integração e entrega contínua (GitHub Actions, GitLab CI, Jenkins).

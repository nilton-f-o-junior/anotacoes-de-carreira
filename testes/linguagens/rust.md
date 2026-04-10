# Organizando

- Testes unitários em .rs normalmente a gente coloca direto no arquivo principal depois da nossa funcão, o rust quando compilar ele não gera isso no resultado final.

- Testes de integração para as funcionalidades maiores, podemos seguir com o padrão normal de mercado: /src /test podemos manter as duas pastas e criar fora. 

```rs
// codigo
pub fn imprimir () {
  println!("Olá, mundo!");
}

// test
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_imprimir(){
    imprimir();
  }
  
}

// resultado
cargo test -- --nocapture

// saida
// running 1 test
// Hello, world!
// test tests::test_imprimir ... ok
```

- Em relação ao resultado, nos temos opções, mas os mais simples são:

```bash
cargo test

# Saída:
running 1 test
test tests::test_imprimir ... ok
```

```bash
cargo test -- --nocapture

# Saída:
running 1 test
Hello, world!
test tests::test_imprimir ... ok
```

```bash
cargo test                        # todos
cargo test --test integracao      # só o arquivo tests/integracao.rs
cargo test --doc                  # só os doc tests
```

- Estamos fazendos os testes unitários e vai acabar se acumulando várias funcoes, metodos e testes, então podemos organizar o código da melhor forma possível para cada equipe, logo:


- Funções tudo junto e depois teste

```rust
pub fn imprimir() {
    println!("Hello, world!");
}

pub fn var_imprimir() {
    let valor: i8 = 2;
    println!("{valor}");
}

#[cfg(test)]
mod tests_a {
    use super::*;

    //
    #[test]
    fn test_imprimir() {
        imprimir()
    }

    //
    #[test]
    fn test_var_imprimir() {
        var_imprimir();
    }
}
```

- Função depois teste > Função depois teste

```rust
pub fn imprimir() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    fn test_imprimir() {
        imprimir()
    }
}

pub fn var_imprimir() {
    let valor: i8 = 2;
    println!("{valor}");
}

#[cfg(test)]
mod tests_b {
    use super::*;

    #[test]
    fn test_var_imprimir() {
        var_imprimir();
    }
}
```

> Rust diferente de C, você pode organizar da forma como acreditar ser melhor!

Análisando todos os dados da saída

| Parte | Significado |
|---|---|
| `test result: ok` | Todos os testes passaram |
| `2 passed` | 2 testes rodaram e passaram |
| `0 failed` | Nenhum teste falhou |
| `0 ignored` | Nenhum teste foi marcado com `#[ignore]` |
| `0 measured` | Nenhum benchmark rodou (seria com `#[bench]`) |
| `0 filtered out` | Nenhum teste foi excluído por filtro de nome |
| `finished in 0.00s` | Tempo total de execução |


Mesmo que você escreva vários testes, você pode rodar apenas 1

```bash
cargo test test_imprimir  # roda só esse teste
```

Vamos aprender a ignorar um teste?

```rust
pub fn imprimir() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_a {
    use super::*;

    // Vamos ignorar esse teste! O teste roda normalmente, mas sai ignore nessa parte!
    #[test]
    #[ignore]
    fn test_imprimir() {
        imprimir();
    }
}
```

Em rust mesmo que tenhamos uma ideia que o código é escrito para não ter erros como estouro de memoria, podemos testar situações assim e segue o cógido:


```rust
pub fn sub(a: i8, b: i8) -> i8 {
    a - b
}

#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    #[should_panic]
    fn test_sub() {
        sub(127, -1); // 127 - (-1) = 128, estoura i8!
    }
}

```

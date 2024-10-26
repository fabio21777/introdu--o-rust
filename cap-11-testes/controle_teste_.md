# Controlando como os testes são executados

Assim como cargo runcompila seu código e então executa o binário resultante, cargo testcompila seu código no modo de teste e executa o binário de teste resultante. O comportamento padrão do binário produzido por cargo testé executar todos os testes em paralelo e capturar a saída gerada durante as execuções de teste, impedindo que a saída seja exibida e facilitando a leitura da saída relacionada aos resultados do teste. Você pode, no entanto, especificar opções de linha de comando para alterar esse comportamento padrão.


## Executando testes em paralelo ou em série

Se você não quiser executar os testes em paralelo ou se quiser um controle mais refinado sobre o número de threads usados, você pode enviar o --test-threadssinalizador e o número de threads que você quer usar para o binário de teste. Dê uma olhada no exemplo a seguir:

```sh
$ cargo test -- --test-threads=1
```

## Exibindo a saída da função de teste

Por padrão, se um teste for aprovado, a biblioteca de testes do Rust captura qualquer coisa impressa na saída padrão. Por exemplo, se chamarmos println!um teste e o teste for aprovado, não veremos a println!saída no terminal; veremos apenas a linha que indica que o teste foi aprovado. Se um teste falhar, veremos o que foi impresso na saída padrão com o restante da mensagem de falha.


```rs
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
```

```sh
$ cargo test
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 10
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

```

se você quiser ver a saída da função de teste, você pode adicionar o --show-outputflag para o comando de teste:

```sh
$ cargo test -- --show-output
```
## Rodando um subconjunto de testes


Às vezes, executar um conjunto de testes completo pode levar muito tempo. Se você estiver trabalhando em um código em uma área específica, talvez queira executar apenas os testes pertencentes a esse código. Você pode escolher quais testes executar passando cargo testo nome ou nomes do(s) teste(s) que deseja executar como um argumento.


```rs 
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
```

```sh
$ cargo test one_hundred
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s


```

Aqui, estamos executando apenas o one_hundredteste. Se você quiser executar vários testes, você pode passar vários argumentos para cargo test. Por exemplo, se você quiser executar o add_two_and_twoe o one_hundredtestes, você pode executar cargo test add_two_and_two one_hundred.

### Filtrando para executar testes que contêm uma palavra-chave

Se você quiser executar apenas os testes que contêm uma determinada palavra-chave, você pode passar a --testflag e a palavra-chave que você deseja filtrar. Por exemplo, se você quiser executar todos os testes que contêm addem seus nomes, você pode executar cargo test --test add.

```sh
$ cargo test add
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

```

## Ignorando testes

Às vezes, você pode querer escrever um teste que ainda não está pronto para ser executado, mas não quer apagá-lo. Você pode usar a #[ignore]anotação para sinalizar que um teste deve ser ignorado. Aqui está um exemplo:

```rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

```

```sh
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```


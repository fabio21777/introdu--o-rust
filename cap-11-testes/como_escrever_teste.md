# Como escrever testes

Vamos dar uma olhada nos recursos que o Rust fornece especificamente para escrever testes que realizam essas ações, que incluem o testatributo, algumas macros e o should_panicatributo.

## A anatomia de uma função de teste


Em sua forma mais simples, um teste em Rust é uma função que é anotada com o test atributo. Atributos são metadados sobre partes do código Rust; um exemplo é o deriveatributo que usamos com structs no Capítulo 5. Para transformar uma função em uma função de teste, adicione #[test]a linha antes de fn. Quando você executa seus testes com o cargo testcomando, Rust cria um binário de execução de teste que executa as funções anotadas e relata se cada função de teste passa ou falha.

Sempre que fazemos um novo projeto de biblioteca com Cargo, um módulo de teste com uma função de teste é gerado automaticamente para nós. Este módulo fornece um modelo para escrever seus testes, para que você não precise procurar a estrutura e a sintaxe exatas toda vez que iniciar um novo projeto. Você pode adicionar quantas funções de teste adicionais e quantos módulos de teste quiser!

Exploraremos alguns aspectos de como os testes funcionam experimentando o teste de modelo antes de realmente testar qualquer código. Então, escreveremos alguns testes do mundo real que chamam algum código que escrevemos e afirmam que seu comportamento está correto.


```rs 
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
```

```sh
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

## Verificando pânico com should_panic

Além de verificar valores de retorno, é importante verificar se nosso código lida com condições de erro como esperamos. Por exemplo, considere o Guesstipo que criamos no Capítulo 9, Listagem 9-13. Outro código que usa Guess depende da garantia de que Guessas instâncias conterão apenas valores entre 1 e 100. Podemos escrever um teste que garanta que tentar criar uma Guessinstância com um valor fora desse intervalo cause pânico.

```rs
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
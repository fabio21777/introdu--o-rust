# Organizando testes em Rust

comunidade Rust pensa em testes em termos de duas categorias principais: testes unitários e testes de integração. Os testes unitários são pequenos e mais focados, testando um módulo isoladamente por vez, e podem testar interfaces privadas.

## Testes unitários
A #[cfg(test)]anotação no testsmódulo diz ao Rust para compilar e executar o código de teste somente quando você executar cargo test, não quando você executar cargo build. Isso economiza tempo de compilação quando você deseja apenas construir a biblioteca e economiza espaço no artefato compilado resultante porque os testes não são incluídos. Você verá que, como os testes de integração vão para um diretório diferente, eles não precisam da #[cfg(test)]anotação. No entanto, como os testes de unidade vão nos mesmos arquivos que o código, você usará #[cfg(test)]para especificar que eles não devem ser incluídos no resultado compilado.

Lembre-se de que quando geramos o novo adder projeto na primeira seção deste capítulo, o Cargo gerou este código para nós:

### Testando funções privadas

Há um debate dentro da comunidade de testes sobre se funções privadas devem ou não ser testadas diretamente, e outras linguagens dificultam ou impossibilitam o teste de funções privadas. Independentemente de qual ideologia de teste você aderir, as regras de privacidade do Rust permitem que você teste funções privadas. Considere o código na Listagem 11-12 com a função privada internal_adder.

```rs
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```

Aqui, a internal_adderfunção é privada, mas o teste interno é capaz de chamar a função diretamente. O teste interno é um teste de unidade para a internal_adderfunção. Se você executar cargo test, verá que o teste passa.

## Testes de integração

Os testes de integração são testes de código que exercitam seu código em um caminho mais completo, testando muitas partes do seu código em conjunto. Se seus testes de unidade estão bem escritos, você terá menos testes de integração para escrever. Isso ocorre porque você terá testado a maioria do comportamento do seu código com testes de unidade, então seus testes de integração podem ser mais focados em testar a interação entre os módulos.

### Diretórios de teste

Criamos um diretório de testes no nível superior do diretório do nosso projeto, ao lado de src . O Cargo sabe procurar por arquivos de teste de integração neste diretório. Podemos então fazer quantos arquivos de teste quisermos, e o Cargo compilará cada um dos arquivos como uma caixa individual.

Vamos criar um teste de integração. Com o código na Listagem 11-12 ainda no arquivo src/lib.rs , crie um diretório tests e crie um novo arquivo chamado tests/integration_test.rs . Sua estrutura de diretório deve ficar assim:

```sh
$ tree
.
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Cada arquivo no diretório de testes é uma caixa separada, então precisamos trazer nossa biblioteca para o escopo de cada caixa de teste. Por esse motivo, adicionamos use adder::add_two;no topo do código, o que não precisávamos nos testes unitários.

Não precisamos anotar nenhum código em tests/integration_test.rs com #[cfg(test)]. O Cargo trata o diretório tests de forma especial e compila arquivos neste diretório somente quando executamos cargo test. Execute cargo testagora:

```sh
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```


## Resumo

Os recursos de teste do Rust fornecem uma maneira de especificar como o código deve funcionar para garantir que ele continue a funcionar como você espera, mesmo quando você faz alterações. Os testes de unidade exercitam diferentes partes de uma biblioteca separadamente e podem testar detalhes de implementação privados. Os testes de integração verificam se muitas partes da biblioteca funcionam juntas corretamente e usam a API pública da biblioteca para testar o código da mesma forma que o código externo o usará. Embora o sistema de tipos e as regras de propriedade do Rust ajudem a evitar alguns tipos de bugs, os testes ainda são importantes para reduzir bugs de lógica relacionados a como seu código deve se comportar.
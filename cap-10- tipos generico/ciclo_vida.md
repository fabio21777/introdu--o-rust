Validando referências com tempos de vida

Lifetimes são outro tipo de genérico que já usamos. Em vez de garantir que um tipo tenha o comportamento que queremos, lifetimes garantem que as referências sejam válidas pelo tempo que precisamos que sejam.

Um detalhe que não discutimos na seção “Referências e Empréstimos” no Capítulo 4 é que cada referência em Rust tem um tempo de vida , que é o escopo para o qual essa referência é válida. Na maioria das vezes, os tempos de vida são implícitos e inferidos, assim como na maioria das vezes, os tipos são inferidos. Devemos anotar tipos somente quando vários tipos são possíveis. De forma semelhante, devemos anotar tempos de vida quando os tempos de vida das referências podem ser relacionados de algumas maneiras diferentes. Rust exige que anotemos os relacionamentos usando parâmetros de tempo de vida genéricos para garantir que as referências reais usadas no tempo de execução definitivamente serão válidas.

Anotar tempos de vida não é um conceito que a maioria das outras linguagens de programação tem, então isso vai parecer estranho. Embora não abordemos tempos de vida em sua totalidade neste capítulo, discutiremos maneiras comuns pelas quais você pode encontrar a sintaxe de tempo de vida para que você possa se sentir confortável com o conceito.



## Evitando referências pendentes com tempos de vida

O principal objetivo dos lifetimes é evitar referências pendentes , que fazem com que um programa faça referência a dados diferentes dos dados que ele pretende referenciar. Considere o programa na Listagem 10-16, que tem um escopo externo e um escopo interno.O principal objetivo dos lifetimes é evitar referências pendentes , que fazem com que um programa faça referência a dados diferentes dos dados que ele pretende referenciar. Considere o programa na Listagem 10-16, que tem um escopo externo e um escopo interno.

```rust

fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

```

O escopo externo declara uma variável chamada rsem valor inicial, e o escopo interno declara uma variável chamada xcom o valor inicial de 5. Dentro do escopo interno, tentamos definir o valor de rcomo uma referência a x. Então o escopo interno termina, e tentamos imprimir o valor em r. Este código não será compilado porque o valor que restá se referindo a saiu do escopo antes de tentarmos usá-lo. Aqui está a mensagem de erro:

## O Verificador de Empréstimos

O compilador Rust tem um verificador de empréstimos que compara escopos para determinar se todos os empréstimos são válidos. A Listagem 10-17 mostra o mesmo código da Listagem 10-16, mas com anotações mostrando os tempos de vida das variáveis.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

## Anotando tempos de vida em funções

Escreveremos uma função que retorna a maior das duas fatias de string. Essa função pegará duas fatias de string e retornará uma única fatia de string. Após implementarmos a longestfunção, o código na Listagem 10-19 deve imprimir The longest string is abcd.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

O texto de ajuda revela que o tipo de retorno precisa de um parâmetro de tempo de vida genérico nele porque Rust não consegue dizer se a referência retornada se refere a xou y. Na verdade, também não sabemos, porque o ifbloco no corpo desta função retorna uma referência a xe o elsebloco retorna uma referência a y!

Ao definir esta função, não sabemos os valores concretos que serão passados ​​para esta função, então não sabemos se o ifcase ou o elsecase serão executados. Também não sabemos os tempos de vida concretos das referências que serão passadas, então não podemos olhar os escopos como fizemos nas Listagens 10-17 e 10-18 para determinar se a referência que retornamos será sempre válida. O verificador de empréstimo também não pode determinar isso, porque ele não sabe como os tempos de vida de xe yse relacionam com o tempo de vida do valor de retorno. Para corrigir esse erro, adicionaremos parâmetros de tempo de vida genéricos que definem o relacionamento entre as referências para que o verificador de empréstimo possa executar sua análise.

### Sintaxe de tempo de vida em funções

Anotações de tempo de vida não mudam o tempo de vida de qualquer uma das referências. Em vez disso, elas descrevem os relacionamentos dos tempos de vida de várias referências entre si sem afetar os tempos de vida. Assim como funções podem aceitar qualquer tipo quando a assinatura especifica um parâmetro de tipo genérico, funções podem aceitar referências com qualquer tempo de vida especificando um parâmetro de tempo de vida genérico.

Anotações de tempo de vida têm uma sintaxe um pouco incomum: os nomes dos parâmetros de tempo de vida devem começar com um apóstrofo ( ') e geralmente são todos minúsculos e muito curtos, como tipos genéricos. A maioria das pessoas usa o nome 'apara a primeira anotação de tempo de vida. Colocamos anotações de parâmetros de tempo de vida após o &de uma referência, usando um espaço para separar a anotação do tipo da referência.

Aqui estão alguns exemplos: uma referência a um i32sem um parâmetro de tempo de vida, uma referência a um i32que tem um parâmetro de tempo de vida chamado 'ae uma referência mutável a um i32que também tem o tempo de vida 'a.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

```

### Especificando um tempo de vida genérico em uma assinatura de função

Para usar anotações de tempo de vida em assinaturas de função, precisamos declarar os parâmetros genéricos de tempo de vida dentro de colchetes angulares entre o nome da função e a lista de parâmetros, assim como fizemos com parâmetros de tipo genérico.

Queremos que a assinatura expresse a seguinte restrição: a referência retornada será válida enquanto ambos os parâmetros forem válidos. Essa é a relação entre os tempos de vida dos parâmetros e o valor de retorno. Nomearemos o tempo de vida 'ae então o adicionaremos a cada referência, como mostrado na Listagem 10-21.


```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
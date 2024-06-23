# Definindo uma instância de uma struct

As estruturas são semelhantes às tuplas, discutidas na seção “O tipo de tupla” , pois ambas contêm vários valores relacionados. Assim como as tuplas, as partes de uma estrutura podem ser de tipos diferentes. Ao contrário das tuplas, em uma estrutura você nomeará cada dado para que fique claro o que os valores significam. Adicionar esses nomes significa que as estruturas são mais flexíveis que as tuplas: você não precisa confiar na ordem dos dados para especificar ou acessar os valores de uma instância.

Para definir uma estrutura, inserimos a palavra-chave structe nomeamos toda a estrutura. O nome de uma estrutura deve descrever o significado dos dados que estão sendo agrupados. Em seguida, entre colchetes, definimos os nomes e tipos dos dados, que chamamos de campos . Por exemplo, a Listagem 5-1 mostra uma estrutura que armazena informações sobre uma conta de usuário.

```rs
struct User {
    ativado: bool,
    nome: String,
    email: String,
    sign_in_count: u64,
}
```

Para usar uma estrutura depois de defini-la, criamos uma instância dessa estrutura especificando valores concretos para cada um dos campos. Criamos uma instância informando o nome da estrutura e, em seguida, adicionamos chaves contendo pares chave: valor , onde as chaves são os nomes dos campos e os valores são os dados que queremos armazenar nesses campos. Não precisamos especificar os campos na mesma ordem em que os declaramos na estrutura. Em outras palavras, a definição da estrutura é como um modelo geral para o tipo, e as instâncias preenchem esse modelo com dados específicos para criar valores do tipo.

```rs


fn main() {
    let user1 = User {
        email: String::from("Fabiosouza21777@gmail.com"),
        nome: String::from("Fabio Souza"),
        sign_in_count: 1,
        ativado: true,
    }; // o user 1 é imutável

    let mut user2 = User {
        email: String::from("use@teste.com"),
        nome: String::from("User Teste"),
        sign_in_count: 1,
        ativado: true,
    }; // o user 2 é mutável

    println!("Nome: {}", user1.nome); // user1.nome é uma referência a um valor de string que é

    user2.email = String::from("User Teste updated");
    println!("Email: {}", user2.email);


    let user3 = build_user(String::from("teste email"), String::from("teste nome"));
    println!("Email: {}", user3.email);
}


/*fn build_user(email: String, nome: String) -> User {
    User {
        email,
        nome,
        sign_in_count: 1,
        ativado: true,
    }
}*/

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}



```

Observe que toda a instância deve ser mutável; Rust não nos permite marcar apenas determinados campos como mutáveis. Como acontece com qualquer expressão, podemos construir uma nova instância da struct como a última expressão no corpo da função para retornar implicitamente essa nova instância.

Aqui, estamos criando uma nova instância da Userstruct, que possui um campo chamado email. Queremos definir o emailvalor do campo para o valor do emailparâmetro da build_userfunção. Como o emailcampo e o emailparâmetro têm o mesmo nome, só precisamos escrever emailem vez de email: email.

## Criando instâncias de outras instâncias com sintaxe de atualização de estrutura

Muitas vezes é útil criar uma nova instância de uma estrutura que inclua a maioria dos valores de outra instância, mas altere alguns. Você pode fazer isso usando a sintaxe de atualização de struct .

```rs
fn main() {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Usando a sintaxe de atualização de struct, podemos obter o mesmo efeito com menos código.

```rs
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

O codigo acima ele usar o conceito de mover do rust ou seja o valor user1 é movido para user2, e não é mais válido usar user1, se tentar usar user1 após a atribuição de user2, o código não compilará.

## Usando estruturas de tupla sem campos nomeados para criar tipos diferentes

Rust também oferece suporte a estruturas semelhantes a tuplas, chamadas `tuple structs` . As estruturas de tupla têm o significado adicional que o nome da estrutura fornece, mas não possuem nomes associados aos seus campos; em vez disso, eles apenas têm os tipos de campos. Estruturas de tupla são úteis quando você deseja dar um nome à tupla inteira e torná-la um tipo diferente de outras tuplas, e quando nomear cada campo como em uma estrutura regular seria detalhado ou redundante.

Para definir uma estrutura de tupla, comece com a structpalavra-chave e o nome da estrutura seguido pelos tipos na tupla. Por exemplo, aqui definimos e usamos duas estruturas de tupla denominadas Colorand Point:

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit-Like a unidades sem nenhum campo

Você também pode definir estruturas que não possuem campos! Elas são chamadas de `Unit-Like` a unidades porque se comportam de maneira semelhante a (), o tipo de unidade que mencionamos na seção “O tipo de tupla” . `Unit-Like` a unidades podem ser úteis quando você precisa implementar uma característica em algum tipo, mas não possui nenhum dado que deseja armazenar no próprio tipo. Discutiremos características no Capítulo 10. Aqui está um exemplo de declaração e instanciação de uma estrutura de unidade chamada AlwaysEqual:

```rs

struct AlwaysEqual;

fn main() {
    let _a = AlwaysEqual;
    let _b = AlwaysEqual;
}
```

Para definir AlwaysEqual, usamos a structpalavra-chave, o nome que queremos e depois um ponto e vírgula. Não há necessidade de chaves ou parênteses! Então podemos obter uma instância de AlwaysEqualna subjectvariável de maneira semelhante: usando o nome que definimos, sem chaves ou parênteses. Imagine que mais tarde implementaremos um comportamento para este tipo de forma que cada instância de AlwaysEqualseja sempre igual a cada instância de qualquer outro tipo, talvez para ter um resultado conhecido para fins de teste. Não precisaríamos de nenhum dado para implementar esse comportamento! Você verá no Capítulo 10 como definir características e implementá-las em qualquer tipo, incluindo estruturas semelhantes a unidades.

## Propriedade de dados estruturais

Na Userdefinição de struct na Listagem 5-1, usamos o String tipo de propriedade em vez do &strtipo de fatia de string. Esta é uma escolha deliberada porque queremos que cada instância desta estrutura possua todos os seus dados e que esses dados sejam válidos enquanto toda a estrutura for válida.

Também é possível que structs armazenem referências a dados pertencentes a outra coisa, mas para isso é necessário o uso de `lifetimes` , um recurso do Rust que discutiremos no Capítulo 10. Lifetimes garantem que os dados referenciados por uma struct sejam válidos por quanto tempo. contanto que a estrutura seja. Digamos que você tente armazenar uma referência em uma estrutura sem especificar tempos de vida, como a seguir; isso não vai funcionar:

```rs
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
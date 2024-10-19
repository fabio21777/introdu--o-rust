# Tipos de dados genéricos

Usamos genéricos para criar definições para itens como assinaturas de função ou structs, que podemos usar com muitos tipos de dados concretos diferentes. Vamos primeiro ver como definir funções, structs, enums e métodos usando genéricos. Então, discutiremos como os genéricos afetam o desempenho do código.


## Em Definições de Método
Podemos implementar métodos em structs e enums (como fizemos no Capítulo 5) e usar tipos genéricos em suas definições também. A Listagem 10-9 mostra a Point<T> struct que definimos na Listagem 10-6 com um método chamado ximplementado nela.


```rust 

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

```

Aqui, definimos um método chamado xon Point<T>que retorna uma referência aos dados no campo x.

Note que temos que declarar Tlogo depois implpara que possamos usar Tpara especificar que estamos implementando métodos no tipo Point<T>. Ao declarar Tcomo um tipo genérico depois de impl, Rust pode identificar que o tipo nos colchetes angulares em Pointé um tipo genérico em vez de um tipo concreto. Poderíamos ter escolhido um nome diferente para este parâmetro genérico do que o parâmetro genérico declarado na definição de struct, mas usar o mesmo nome é convencional. Métodos escritos dentro de an implque declara o tipo genérico serão definidos em qualquer instância do tipo, não importa qual tipo concreto acabe substituindo o tipo genérico.

Também podemos especificar restrições em tipos genéricos ao definir métodos no tipo. Poderíamos, por exemplo, implementar métodos somente em Point<f32>instâncias em vez de Point<T>instâncias com qualquer tipo genérico. Na Listagem 10-10, usamos o tipo concreto f32, o que significa que não declaramos nenhum tipo depois de impl.

## Desempenho de código usando genéricos

Você pode estar se perguntando se há um custo de tempo de execução ao usar parâmetros de tipo genérico. A boa notícia é que usar tipos genéricos não fará seu programa rodar mais devagar do que faria com tipos concretos.

Rust realiza isso realizando a monomorfização do código usando genéricos em tempo de compilação. Monomorfização é o processo de transformar código genérico em código específico preenchendo os tipos concretos que são usados ​​quando compilados. Neste processo, o compilador faz o oposto das etapas que usamos para criar a função genérica na Listagem 10-5: o compilador olha para todos os lugares onde o código genérico é chamado e gera código para os tipos concretos com os quais o código genérico é chamado.

Quando Rust compila esse código, ele realiza monomorfização. Durante esse processo, o compilador lê os valores que foram usados ​​em Option<T> instâncias e identifica dois tipos de Option<T>: um é i32e o outro é f64. Como tal, ele expande a definição genérica de Option<T>em duas definições especializadas para i32e f64, substituindo assim a definição genérica pelas específicas.

A versão monomorfizada do código se parece com o seguinte (o compilador usa nomes diferentes dos que estamos usando aqui para ilustração):

O genérico Option<T>é substituído pelas definições específicas criadas pelo compilador. Como o Rust compila código genérico em código que especifica o tipo em cada instância, não pagamos custo de tempo de execução para usar genéricos. Quando o código é executado, ele executa exatamente como se tivéssemos duplicado cada definição manualmente. O processo de monomorfização torna os genéricos do Rust extremamente eficientes em tempo de execução.
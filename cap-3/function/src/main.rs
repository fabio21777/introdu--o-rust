fn main() {
    println!("Hello, world!");

    let valor = 10;
    let unidade = "teste".to_string();

    outra_funcao(valor, unidade);

    let resultado = soma(10, 20);

    println!("O resultado da soma é: {}", resultado);

    let cinco =  plus_one(4);

    println!("O valor de cinco é: {}", cinco);

}

fn outra_funcao(value: i32, label: String) {
    println!("O valor é: {}", value);
    println!("A unidade é: {}", label);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

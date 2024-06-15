fn main() {
    println!("Hello, world!");

    let valor = 10;
    let unidade = "teste".to_string();

    outra_funcao(valor, unidade);

    let resultado = soma(10, 20);

    println!("O resultado da soma é: {}", resultado);

    let cinco =  plus_one(4);

    println!("O valor de cinco é: {}", cinco);

    'externo: for i in 1..=10 {
        //loop de 1 a 10
        println!("O valor de i é: {}", i);
        'interno: for j in 1..=10 {
            if i == 5 && j == 5 {
                println!("O valor de i é: {} e o valor de j é: {}", i, j);
                break 'externo; // sai do loop externo
            }
        }
    }

    let array = [10, 20, 30, 40, 50];

    for elemento in array{
        println!("O elemento é: {}", elemento);
    }

    // for inverso
    for numero in (1..4).rev() {
        println!("O número é: {}", numero);
    }

}

fn outra_funcao(value: i32, label: String) {
    println!("O valor é: {}", value);
    println!("A unidade é: {}", label);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn plus_one(x: i32) -> i32 {
   return  x + 1
}

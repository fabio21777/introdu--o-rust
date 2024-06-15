use std::{cmp::Ordering, io};

use rand::Rng;


/**
 * Notas sobre o código:
 * 
 * 1 - Aprederemos sobre variáveis, mutabilidade, somas, interpolação de strings, entrada de dados, conversão de tipos, 
 * comparação de valores, loops e condicionais
 * 
 * 2 - Adicionamos a dependência rand ao nosso arquivo Cargo.toml
 * 
 * 3 - Utilizamos a função rand::thread_rng() para gerar um número aleatório
 * 
 * 4 - Utilizamos a função gen_range() para gerar um número aleatório entre 1 e 100
 * 
 * 5 - Utilizamos a função expect() para tratar erros
 * 
 * 6 - Utilizamos a função read_line() para ler a linha digitada pelo usuário
 * 
 * 7 - usamos o match para tratar o erro de conversão de string para número e para comparar o palpite do usuário com o número secreto
 * 
 * 8 - Utilizamos o loop para repetir a leitura do palpite do usuário até que ele acerte o número secreto
 * 
 * 
 * 
 */

fn main() {

    let numero_secreto = rand::thread_rng().gen_range(1..=100); // gera um número aleatório entre 1 e 100

    println!("O número secreto é : {numero_secreto}"); // imprime o número secreto (para testes)

    loop {
        println!("Digite um número: ");
    

        let mut guess = String::new(); // declaração de variável mutável
    
    
        io::stdin().read_line(&mut guess).expect("Erro ao ler linha"); // lê a linha digitada pelo usuário
    
        // converte a string digitada pelo usuário para um número inteiro de 32 bits
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        
        println!("Seu palpite {guess}");
    
        //comparando o palpite do usuário com o número secreto
    
        match guess.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
        
    }



    

    /*
    let x = 5; // declaração de variável imutável;
    let y = 6; // declaração de variável imutável;
    println!("o valor de x é {x} é o valor de y é {y} a soma é {}", x + y); // imprime o valor de x e y e a soma deles, aqui existem dusas forma de interpolar as variáveis, a primeira é utilizando {} e a segunda é utilizando {:?}
    */






}

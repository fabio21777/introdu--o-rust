#[derive(Debug)] // para poder imprimir o conteúdo da struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //let width = 30;
    //let height = 50;

    //let rect1 = (30, 50); // usando tupla para representar um retângulo

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    //println!(" A area do retângulo é: {}", area(rectangle));
    println!(" A area do retângulo é: {}", area_ref(&rect2));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        teste => move_player(teste),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("num_spaces é {}", num_spaces)
    }

    //println!("rect1 altura é {}", rect1.height); // isso da erro, pos o valor de rect1 foi movido para a função area
    println!("rect2 altura é {}", rect2.height); // isso da certo, pois passamos a referencia de rect2 para a função areaRef

    println!("rect1 é {:?}", rect2); // para imprimir o conteúdo da struct, é necessário adicionar #[derive(Debug)] na struct

    dbg!(&rect2);
}

/*fn area(width: u32, height: u32) -> u32 {
    width * height
}*/

/*fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}*/

fn area(rectangle: Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn area_ref(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

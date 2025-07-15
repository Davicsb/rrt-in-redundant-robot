use std::io;

struct Triangulo{
    base: i32,
    altura: i32,
}

fn read_number() -> i32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail!");
    let num: i32 = num.trim().parse().expect("Invalid Number");
    num
}

fn cal_area(triangulo: &Triangulo) -> i32{
    (triangulo.base * triangulo.altura)/2
}

fn main(){
    println!("Calcule a área do seu triângulo.\nDigite a base:");
    let base : i32 = read_number();
    println!("Altura:");
    let altura : i32 = read_number();
    let figura = Triangulo{
        base,
        altura,
    };
    println!("A área do seu triângulo é: {}", cal_area(&figura));
}
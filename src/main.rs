use std::io;

fn main (){
    println!("Guess the number!");
    println!("Guess a number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error readind a line");

    println!("Your guess: {}", guess);
}
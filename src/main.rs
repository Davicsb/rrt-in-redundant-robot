use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main (){
    println!("Guess the number!");

    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error readind a line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&num){
            Ordering::Less => println!("But it's too small"),
            Ordering::Greater => println!("But it's too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
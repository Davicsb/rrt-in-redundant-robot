use std::io;

fn read_number() -> i32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail!");
    let num: i32 = num.trim().parse(). expect("Invalid Number");
    num
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}

fn sub(x: i32, y: i32) -> i32{
    x - y
}

fn main (){
    println!("==Calculator==");
    loop{
        println!("What you wanna do?");
        println!("0 - Exit\n1 - Sum\n2 - Sub");
        let option : i32 = read_number();
        if option == 0{
            break;
        }
        else if option == 1{
            println!("Enter the two numbers.");
            let x = read_number();
            let y = read_number();
            println!("Your Sum = {}.", sum(x, y));
        }
        else if option == 2{
            println!("Enter the two numbers.");
            let x = read_number();
            let y = read_number();
            println!("Your Sub = {}.", sub(x, y));
        }
        else{
            println!("Press a valid option!")
        }
    }
    
}
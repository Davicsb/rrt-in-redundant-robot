use std::io;

fn read_number() -> i32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail!");
    let num: i32 = num.trim().parse().expect("Invalid Number");
    num
}

fn read_string() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Fail!");
    s
}

fn len_calculator(s : &String) -> usize{
    s.trim().len()
}

fn main (){
    println!("==String len calculator==");
    loop{
        println!("What you wanna do?");
        println!("0 - Exit\n1 - Read string");
        let option : i32 = read_number();
        if option == 0{
            break;
        }
        else if option == 1{
            println!("Enter the string.");
            let s = read_string();
            let len = len_calculator(&s);
            println!("Len = {}.", len);
        }
        else{
            println!("Press a valid option!")
        }
    }
    
}
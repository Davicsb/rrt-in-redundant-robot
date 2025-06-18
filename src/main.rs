use std::io;
use std::fmt;

struct Color(i32, i32, i32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb:({}, {}, {})", self.0, self.1, self.2)
    }
}

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

fn first_slice(s : &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main (){
    println!("==String len calculator==");
    loop{
        println!("What you wanna do?");
        println!("0 - Exit\n1 - Len string calculator\n2 - First slice\n3 - View collors");
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
        else if option == 2{
            println!("Enter the string.");
            let s = read_string();
            let word = first_slice(&s);
            println!("First slice: {}.", word);
        }
        else if option == 3{
            let black = Color(0, 0, 0);
            let red = Color(255, 0, 0);
            let white = Color(255, 255, 255);
            loop{
                println!("0 - Exit\n1 - Black\n2 - Red\n3 - White");
                let choose : i32 = read_number();
                if choose == 0{
                    break;
                }
                if choose == 1{
                    println!("{black}");
                }
                else if choose == 2{
                    println!("{red}");
                }
                else if choose == 3{
                    println!("{white}");
                }
                else{
                    println!("Invalid option, try again!")
                } 
            }
        }
        else{
            println!("Press a valid option!")
        }
    }
    
}
use std::io;

fn read_string() -> String {
    let mut out_string = String::new();
    match io::stdin().read_line(&mut out_string) {
        Ok(_) => {
            out_string = out_string.trim().to_string();
            out_string
        }
        Err(_) => {
            panic!("fail to read a str.");
        }
    }
}

fn read_int() -> i32 {
    let mut buff = String::new();
    match io::stdin().read_line(&mut buff) {
        Ok(_) => buff.trim().parse::<i32>().unwrap(),
        Err(_) => {
            panic!("fail to read a i32");
        }
    }
}

fn read_char() -> char {
    let mut buff = String::new();
    match io::stdin().read_line(&mut buff) {
        Ok(_) => match buff.trim().parse::<char>() {
            Ok(c) => c,
            Err(_) => {
                panic!("fail to parse a char");
            }
        },
        Err(_) => {
            panic!("fail to read a char");
        }
    }
}

//不支持可变参数和void*指针

fn main() {
    println!("What's your first name? ");
    let first_name = read_string();

    println!("What's your initial? ");
    let initial = read_char();

    println!("What's your last name? ");
    let last_name = read_string();

    println!("How old are you? ");
    let age = read_int();

    println!("---- RESULTS ----");
    println!("First Name: {}", first_name);
    println!("Initial: {}", initial);
    println!("Last Name: {}", last_name);
    println!("Age: {}", age);
}

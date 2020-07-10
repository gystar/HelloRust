use std::io;

const EYE_COLOR_NAMES: [&str; 5] = ["Blue", "Green", "Brown", "Black", "Other"];

struct Person {
    age: usize,
    first_name: String,
    last_name: String,
    eyes: usize, //0..5
    income: f32,
}

fn main() {
    let mut you = Person {
        age: 0,
        first_name: String::new(),
        last_name: String::new(),
        eyes: 5,
        income: 0.0f32,
    };

    //input fisrt name
    println!("What's your First Name? ");
    io::stdin()
        .read_line(&mut you.first_name)
        .expect("Failed to read first name.");
    you.first_name = you.first_name.trim().to_string();
    //input last name
    println!("What's your Last Name? ");
    io::stdin()
        .read_line(&mut you.last_name)
        .expect("Failed to read last name.");
    you.last_name = you.last_name.trim().to_string();

    //input age
    println!("How old are you? ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("You have to enter a number.");
    you.age = age
        .trim()
        .parse::<usize>()
        .expect("You have to enter a number.");

    //input color of eye
    println!("What color are your eyes:");
    for i in 0..5 {
        println!("{:?} {:?}", i, EYE_COLOR_NAMES[i]);
    }
    print!("> ");
    let mut eyes = String::new();
    io::stdin()
        .read_line(&mut eyes)
        .expect("You have to enter a number.");
    you.eyes = eyes
        .trim()
        .parse::<usize>()
        .expect("You have to enter a number.");
    if you.eyes >= 5 {
        panic!("Do it right, that's not an option.");
    }

    //input income
    println!("How much do you make an hour? ");
    let mut income = String::new();
    io::stdin()
        .read_line(&mut income)
        .expect("You have to enter a positive floating point number.");
    you.income = income
        .trim()
        .parse::<f32>()
        .expect("Enter a floating point number.");
    if you.income < 0.0f32 {
        panic!("Enter a positive floating point number.");
    }

    println!("----- RESULTS -----");
    println!("First Name: {}", you.first_name);
    println!("Last Name: {}", you.last_name);
    println!("Age: {:?}", you.age);
    println!("Eyes: {}", EYE_COLOR_NAMES[you.eyes]);
    println!("Income: {:?}", you.income);
}

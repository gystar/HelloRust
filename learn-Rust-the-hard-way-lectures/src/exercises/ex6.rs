fn main() {
    let distance = 100;
    let power = 2.345f32;
    let super_power = 56789.4532f64;
    let initial = 'A';
    let first_name = vec!["Zed"];
    let last_name = vec!["Shaw"];
    println!("You are {} miles away.", distance);
    println!("You have {} levels of power.", power);
    println!("You have {} awesome super powers.", super_power);
    println!("I have an initial {}.", initial);
    println!("I have a first name {:?}.", first_name);
    println!("I have a last name {:?}.", last_name);
    println!(
        "My whole name is {:?} {}. {:?}.",
        first_name[0], initial, last_name
    );
}

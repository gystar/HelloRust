fn main() {
    let mut numbers: [i32; 4] = [0; 4];
    let mut name: [char; 4] = ['\0'; 4];

    // first, print them out raw
    println!(
        "numbers: {:?} {:?} {:?} {:?}\n",
        numbers[0], numbers[1], numbers[2], numbers[3]
    );

    println!(
        "name each: {:?} {:?} {:?} {:?}\n",
        name[0], name[1], name[2], name[3]
    );

    println!("name: {:?}\n", name);

    // setup the numbers
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    numbers[3] = 4;

    // setup the name
    name[0] = 'Z';
    name[1] = 'e';
    name[2] = 'd';
    name[3] = '\0';

    // then print them out initialized
    println!(
        "numbers: {:?} {:?} {:?} {:?}\n",
        numbers[0], numbers[1], numbers[2], numbers[3]
    );

    println!(
        "name each: {:?} {:?} {:?} {:?}\n",
        name[0], name[1], name[2], name[3]
    );

    // print the name like a string
    println!("name: {:?}\n", name);

    // another way to use name
    let another: Vec<char> = "Zed".chars().collect();

    println!("another: {:?}\n", another);

    println!(
        "another each: {:?} {:?} {:?} {:?}\n",
        another[0], another[1], another[2], another[3]
    );
}

struct Person {
    name: String,
    age: u32,
    height: u32,
    weight: u32,
}

fn person_create(s: &str, age: u32, height: u32, weight: u32) -> Box<Person> {
    Box::new(Person {
        name: s.to_string(),
        age,
        height,
        weight,
    })
}

fn person_print(who: &Box<Person>) {
    println!("Name: {:?}", who.name);
    println!("\tAge: {:?}", who.age);
    println!("\tHeight: {:?}", who.height);
    println!("\tWeight: {:?}", who.weight);
}

fn main() {
    // make two people structures
    let mut joe = person_create("Joe Alex", 32, 64, 140);

    let mut frank = person_create("Frank Blank", 20, 72, 180);

    // print them out and where they are in memory
    println!("Joe is at memory location {:p}:\n", joe);
    person_print(&joe);

    println!("Frank is at memory location {:p}:\n", frank);
    person_print(&frank);

    // make everyone age 20 years and print them again
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    person_print(&joe);

    frank.age += 20;
    frank.weight += 20;
    person_print(&frank);

    // destroy them both so we clean up
}

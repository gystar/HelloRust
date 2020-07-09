use std::mem::{size_of, size_of_val};
fn main() {
    let areas = vec![10, 12, 13, 14, 20];
    let name = "Zed";
    let full_name = vec!['Z', 'e', 'd', ' ', 'A', '.', ' ', 'S', 'h', 'a', 'w', '\0'];

    // WARNING: On some systems you may have to change the
    // %ld in this code to a %u since it will use unsigned ints
    println!("The size of an int: {:?}\n", size_of::<i32>());
    println!("The size of areas (int[]): {:?}\n", size_of_val(&areas));
    println!(
        "The number of ints in areas: {:?}\n",
        size_of_val(&areas) / size_of::<i32>()
    );
    println!(
        "The first area is {:?}, the 2nd {:?}.\n",
        areas[0], areas[1]
    );

    println!("The size of a char: {:?}\n", 1);
    println!("The size of name (char[]): {:?}\n", size_of_val(&name));
    println!(
        "The number of chars: {:?}\n",
        size_of_val(&name) / size_of::<char>()
    );

    println!(
        "The size of full_name (char[]): {:?}\n",
        size_of_val(&full_name)
    );
    println!(
        "The number of chars: {:?}\n",
        size_of_val(&full_name) / size_of::<char>()
    );

    println!("name=\"{:?}\" and full_name=\"{:?}\"\n", name, full_name);
}

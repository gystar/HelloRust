use std::mem::{size_of, size_of_val};
fn main() {
    // create two arrays we care about
    let ages = [23, 43, 12, 89, 2];
    let names = ["Alan", "Frank", "Mary", "John", "Lisa"];

    // safely get the size of ages
    let count = ages.len();

    // first way using indexing
    for i in 0..count {
        println!("{:?} has {:?} years alive.", names[i], ages[i]);
    }

    println!("---\n");

    // setup the pointers to the start of the arrays
    let cur_age = ages;
    let cur_name = names;

    println!("---\n");

    // third way, pointers are just arrays
    for i in 0..count {
        println!("{:?} is {:?} years old again.", cur_name[i], cur_age[i]);
    }
    println!("---\n");
}

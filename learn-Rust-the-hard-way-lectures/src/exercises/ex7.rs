fn main() {
    let bugs = 100;
    let bug_rate = 1.2;

    println!(
        "You have {} bugs at the imaginary rate of {}.\n",
        bugs, bug_rate
    );

    let universe_of_defects = 1i32 * 1024i32 * 1024i32 * 1024i32;
    println!("The entire universe has {} bugs.\n", universe_of_defects);

    let expected_bugs = bugs as f32 * bug_rate;
    println!("You are expected to have {} bugs.\n", expected_bugs);

    let part_of_universe = expected_bugs / universe_of_defects as f32;
    println!(
        "That is only a {} portion of the universe.\n",
        part_of_universe,
    );

    // this makes no sense, just a demo of something weird
    let nul_byte: char = '\0';
    let care_percentage = bugs * (nul_byte as i32);
    println!("Which means you should care {}%%.\n", care_percentage);
}

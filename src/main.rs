use std::io;
use std::num::ParseIntError;

mod pinia;

fn main() {
    let mut parsed_input: Result<u32, ParseIntError>;

    print_options();

    while {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        parsed_input = input.trim().parse();
        let parsed_input_clone = parsed_input.clone();

        parsed_input.is_err() || !(1..=10).contains(&parsed_input_clone.unwrap())
    } {
        println!("Invalid input. Please enter a valid number between 1 and 10:");
        print_options();
    }

    match parsed_input.unwrap() {
        1 => install_pinia(),
        2 => install_husky(),
        3 => install_jenkins(),
        4 => install_jotai(),
        _ => unreachable!(),
    }
}

fn print_options() {
    println!("What do you want to install?");
    println!("1 - Pinia");
    println!("2 - Husky");
    println!("3 - Jenkins");
    println!("4 - Jotai");
}

fn install_pinia() {
    pinia::install();
}

fn install_husky() {
    println!("Installing Husky...");
}

fn install_jenkins() {
    println!("Installing Jenkins...");
}

fn install_jotai() {
    println!("Installing Jotai...");
}

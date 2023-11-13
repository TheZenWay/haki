use std::io;
use std::num::ParseIntError;
use std::process::Command;

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
        println!("Installing Pinia...");

    let npm_install = Command::new("npm")
        .arg("install")
        .arg("pinia")
        .spawn();

    match npm_install {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait for npm install");

            if status.success() {
                let mkdir_status = Command::new("mkdir")
                    .arg("store")
                    .status()
                    .expect("Failed to execute mkdir command");

                if mkdir_status.success() {
                    let mv_status = Command::new("mv")
                        .arg("store.js")
                        .arg("./store/store.js")
                        .status()
                        .expect("Failed to execute mv command");

                    if mv_status.success() {
                        println!("Pinia successfully installed!");
                    } else {
                        println!("Failed to move store.js. mv command failed.");
                    }
                } else {
                    println!("Failed to create the 'store' directory. mkdir command failed.");
                }
            } else {
                println!("Failed to install Pinia. npm install command failed.");
            }
        }
        Err(e) => {
            println!("Failed to run npm install: {}", e);
        }
    }
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

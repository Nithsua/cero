mod password;
mod services;

use crate::password::password::{Password, Passwords};
use password::password_generator::generate_password;

use std::io::{self, Write};

#[allow(dead_code)]
fn cls() {
    print!("{}[2J", 27 as char);
    io::stdout().flush().unwrap();
}

fn password_generation_interface() -> String {
    let mut temp_string = String::new();
    let mut length: u8;
    loop {
        temp_string.clear();
        print!("Length of the password(8-32): ");
        io::stdout().flush().expect("Unable to flush the stdout");
        io::stdin().read_line(&mut temp_string).unwrap();
        length = match temp_string.trim().parse::<u8>() {
            Ok(len) => len,
            Err(_) => {
                println!("Unable to parse the input, try entering a correct option again");
                continue;
            }
        };
        if length >= 8 && length <= 32 {
            break;
        }
        println!("The password length should range from 8 to 32");
    }
    temp_string.clear();
    print!("Include Special Characters (y/n): ");
    io::stdout().flush().expect("Unable to flush the stdout");
    io::stdin().read_line(&mut temp_string).unwrap();
    let include_special_characters = match temp_string.trim() {
        "n" => false,
        _ => true,
    };
    temp_string.clear();
    print!("Include Numbers (y/n): ");
    io::stdout().flush().expect("Unable to flush the stdout");
    io::stdin().read_line(&mut temp_string).unwrap();
    let include_numbers = match temp_string.trim() {
        "n" => false,
        _ => true,
    };
    temp_string.clear();
    print!("Include Spaces (y/n): ");
    io::stdout().flush().expect("Unable to flush the stdout");
    io::stdin().read_line(&mut temp_string).unwrap();
    let include_spaces = match temp_string.trim() {
        "n" => false,
        _ => true,
    };

    generate_password(
        length,
        include_numbers,
        include_special_characters,
        include_spaces,
    )
}

fn delete_interface() {
    let passwords = services::local_store::read_all_the_data();
    let length = print_the_list(&passwords);
    if length == 0 {
        return;
    }
    let mut temp = String::new();
    println!("Enter the number of the password to be deleted: ");
    io::stdin().read_line(&mut temp).unwrap();
    let temp = temp.trim().parse::<u32>().unwrap();
    if temp > (length as u32) || temp == 0 {
        println!("Take a another look");
    } else {
        services::local_store::delete_from_local_store(&passwords.get((temp - 1) as usize).id)
    }
}

fn print_the_list(passwords: &Passwords) -> usize {
    let length = passwords.length_of_passwords();
    if length == 0 {
        println!("Your Passwords list is clean over here, try adding some");
        return 0;
    }

    let mut i = 1;
    for v in passwords.get_passwords() {
        println!(
            "{}. {}{{ url:{}, username:{}, password:{} }}",
            i, v.name, v.url, v.username, v.password
        );
        i += 1;
    }
    length
}

fn main() {
    loop {
        let mut temp_string = String::new();
        // cls();
        println!("\nCero Password Manager");
        println!("1. Generate New Password");
        println!("2. Store Password");
        // println!("3. Search");
        println!("4. List Everything");
        println!("5. Edit Password");
        println!("6. Delete Password");
        println!("0. Exit");
        print!("Option: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Unable to read the input");
        let choice = match temp_string.trim().parse::<u8>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Unable to parse the input make sure you are entering the right option");
                continue;
            }
        };
        match choice {
            1 => println!(
                "The generated password is: {}",
                password_generation_interface()
            ),
            2 => {
                let mut name = String::new();
                let mut url = String::new();
                let mut username = String::new();
                let mut password = String::new();
                let mut temp_string = String::new();
                println!("Enter the name of the password");
                io::stdin().read_line(&mut name).unwrap();
                println!("Enter the URL: ");
                io::stdin().read_line(&mut url).expect("");
                println!("Enter the username: ");
                io::stdin().read_line(&mut username).expect("");
                println!("Do you want auto generated password? (y/n): ");
                io::stdin().read_line(&mut temp_string).expect("");
                if temp_string.trim() == "n" || temp_string.trim() == "N" {
                    println!("Enter the password: ");
                    io::stdin().read_line(&mut password).expect("");
                } else {
                    password = password_generation_interface();
                }
                let password = Password::new(
                    name.trim().to_string(),
                    url.trim().to_string(),
                    username.trim().to_string(),
                    password.trim().to_string(),
                );
                services::local_store::add_to_local_store(password.id.clone(), password);
            }
            4 => {
                print_the_list(&services::local_store::read_all_the_data());
            }
            5 => {
                let passwords = services::local_store::read_all_the_data();
                let length = print_the_list(&passwords);
                println!("Enter the no of password to be modified: ");
                temp_string.clear();
                io::stdin().read_line(&mut temp_string).unwrap();
                let choice = temp_string.trim().parse::<u32>().unwrap();
                if choice == 0 && choice > length as u32 {
                    println!("Take another look at the options");
                } else {
                    let mut password = passwords.get((choice - 1) as usize).clone();
                    println!("What do you want to change?");
                    println!("1. Name");
                    println!("2. URL");
                    println!("3. Username");
                    println!("4. Password");
                    temp_string.clear();
                    io::stdin().read_line(&mut temp_string).unwrap();
                    match temp_string.trim().parse::<u32>().unwrap() {
                        1 => {
                            println!("Enter the new Name: ");
                            temp_string.clear();
                            io::stdin().read_line(&mut temp_string).unwrap();
                            password.name = temp_string.trim().to_string();
                        }
                        2 => {
                            println!("Enter the new URL: ");
                            temp_string.clear();
                            io::stdin().read_line(&mut temp_string).unwrap();
                            password.url = temp_string.trim().to_string();
                        }
                        3 => {
                            println!("Enter the new Username: ");
                            temp_string.clear();
                            io::stdin().read_line(&mut temp_string).unwrap();
                            password.username = temp_string.trim().to_string();
                        }
                        4 => {
                            println!("Do you want auto generated password? (y/n): ");
                            temp_string.clear();
                            io::stdin().read_line(&mut temp_string).expect("");
                            if temp_string.trim() == "n" || temp_string.trim() == "N" {
                                println!("Enter the password: ");
                                temp_string.clear();
                                io::stdin().read_line(&mut temp_string).expect("");
                                password.password = temp_string.trim().to_string();
                            } else {
                                password.password = password_generation_interface()
                            }
                        }
                        _ => {
                            println!("Take another look at the options");
                            continue;
                        }
                    }
                    services::local_store::add_to_local_store(password.id.clone(), password);
                }
            }
            6 => delete_interface(),
            0 => return,
            _ => println!("Wrong choice, take a another look"),
        }
    }
}

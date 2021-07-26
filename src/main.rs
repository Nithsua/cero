mod password;
use password::password_generator::generate_password;
use std::io::{self, Write};

#[allow(dead_code)]
fn cls() {
    print!("{}[2J", 27 as char);
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        let mut temp_string = String::new();
        // cls();
        println!("\nCero Password Manager");
        println!("1.Create New Password");
        println!("0.Exit");
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
            1 => {
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
                println!(
                    "{}",
                    generate_password(
                        16,
                        include_numbers,
                        include_special_characters,
                        include_spaces
                    )
                );
            }
            0 => return,
            _ => println!("Wrong choice, take a another look"),
        }
    }
}

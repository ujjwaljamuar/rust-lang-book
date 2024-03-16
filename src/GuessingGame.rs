use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io, u32};

fn main() {
    println!("Radhe Radhe");
    loop {
        println!("Please input a number : ");

        let random_number = rand::thread_rng().gen_range(1..11);

        let mut guess_string = String::new();

        io::stdin()
            .read_line(&mut guess_string)
            .expect("Failed to read line");

        let guess_number: u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Actual number is : {}", random_number);
        print!("Your guessed number is: {}", guess_string);

        match guess_number.cmp(&random_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too large".red()),
            Ordering::Equal => {
                println!("{}", "WON!!".green());
                break;
            }
        }
    }
}

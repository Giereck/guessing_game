use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {}", secret_number);
        
    loop {       
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("An error occurred while taking the user input.");

        println!("Your input was {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {

            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big.".red()),
            Ordering::Less => println!("{}", "Too small.".red())
        }    
    }
}

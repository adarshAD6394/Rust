use std::io;
use std::cmp::Ordering;
use colored::*;
use rand::Rng;

fn main() {
    println!("Hello, Rusty!");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_num);

    loop {
        println!(" Guess a number!");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a Valid number");
                continue; 
            }
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
        println!("You guessed:{}", guess);
    }
}
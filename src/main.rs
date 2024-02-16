use clap::{App, Arg};
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let matches = App::new("Guess the Number Game")
        .version("1.0")
        .author("akash2061")
        .about("A simple command-line guessing game.")
        .arg(
            Arg::with_name("attempts")
                .short("a")
                .long("attempts")
                .value_name("ATTEMPTS")
                .help("Sets the number of attempts allowed")
                .takes_value(true),
        )
        .get_matches();

    let max_attempts = matches
        .value_of("attempts")
        .and_then(|a| a.parse().ok())
        .unwrap_or(5);

    println!("Welcome to My CLI Game.");
    println!("------------------------------------------");
    println!(
        "Rules:\nYou have {} Attempts to guess the Secret Number.",
        max_attempts
    );
    println!("------------------------------------------\n");
    let random = rand::thread_rng().gen_range(1..101);
    let mut i = 0;
    //println!("{}", random);
    loop {
        if i == max_attempts {
            let message = format!("The Number is: {}", random).italic().yellow();
            println!("{}", message);
            println!("{}", "Game Over...!".italic().yellow());
            break;
        }
        i += 1;
        println!("Attempt {}...\nGuess the Number: ", i);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) if num >= 0 => num,
            Ok(_) => {
                println!("{}", "Enter a non-negative Number.".red());
                continue;
            }
            Err(_) => {
                println!("{}", "Enter a Number.".red());
                continue;
            }
        };
        let message = format!("Your Number: {}\n", guess).cyan();
        println!("{}", message);

        match guess.cmp(&random) {
            Ordering::Less => {
                println!("{}", "Too Small!!!".red());
                println!("{}", "Try Again...\n".red());
            }
            Ordering::Greater => {
                println!("{}", "Too Large!!!".red());
                println!("{}", "Try Again...\n".red());
            }
            Ordering::Equal => {
                println!("{}", "You Win!!!\n".green());
                break;
            }
        }
    }
}

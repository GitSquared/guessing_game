extern crate rand;
extern crate colored;

use std::io::*;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

static GREETING: &str = "=> Guess the number!";
static FIRST_PROMPT: &str = "Please input your guess:";
static PROMPT: &str = "New guess?";
static TOO_LOW: &str = "Too small!";
static TOO_HIGH: &str = "Too big!";
static WIN: &str = "Congratulations! You won!";

fn main() {
    println!("{}", GREETING.yellow().bold().italic());

    let secret_number = rand::thread_rng().gen_range(1, 101); // .gen_range is from Rng.
    let mut is_first_run: bool = true;

    loop {
        let mut guess = String::new();

        if is_first_run == true {
            is_first_run = false;
            print!("   {} ", FIRST_PROMPT.bold());
        }

        stdout().flush()
            .expect("Failed to flush stdout");
        stdin().read_line(&mut guess)
            .expect("Failed to read line :(");

        // Sanitize input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { // _ means "any kind of value" (like *)
                print!("{}", "⨯  Please enter a valid number: ".red().bold());
                continue; // next round of loop
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("{}  {} {} ", "▲".yellow().bold(), TOO_LOW.yellow().bold(), PROMPT.bold()),
            Ordering::Greater => print!("{}  {} {} ", "▼".cyan().bold(), TOO_HIGH.cyan().bold(), PROMPT.bold()),
            Ordering::Equal => {
                println!("{}  {}", "✓".green().bold(), WIN.green().bold());
                break; // Break loop
            }
        }
    }
}

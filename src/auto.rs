extern crate rand;
extern crate colored;
extern crate meval;

use std::io::*;
use std::cmp::Ordering;
use std::thread;
use std::time;
use rand::Rng;
use colored::*;

static GREETING: &str = "Welcome to autoguess!";
static FIRST_PROMPT: &str = "Please choose the range for the random number the computer will try to guess (0-?): ";

fn pause() {
    print!("Press any key to continue...");

    stdout().flush()
        .expect("Failed to flush stdout");

    // Read a single byte and discard
    let _ = stdin().read(&mut [0u8]).unwrap();
}

fn main() {
    println!("{}", GREETING.yellow().bold().italic());
    print!("{}", FIRST_PROMPT.bold());

    let mut max = String::new();

    stdout().flush()
        .expect("Failed to flush stdout");
    stdin().read_line(&mut max)
        .expect("Failed to read line :(");

    // Sanitize input
    let max: f64 = meval::eval_str(max)
        .expect("Invalid number or expression entered")
        .round();

    assert!(max.is_normal());

    let mut max = max as i64;
    if max <= 0 {
        max = i64::max_value();
    }

    let secret_number: i64 = rand::thread_rng().gen_range(0, max);
    println!("The number the computer has to guess is: {}", secret_number.to_string().yellow().bold());

    pause();

    // It's not fun if it's instant, right?
    let step_sleep_timer = time::Duration::from_millis(30);

    let mut x: i64 = 0;
    let mut y: i64 = max;
    let mut attempts: i64 = 0;

    loop {
        attempts = attempts+1;
        let guess: i64 = ((y-x)/2)+x;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{} {}", "▲".yellow().bold(), guess);
                x = guess;
                thread::sleep(step_sleep_timer);
            },
            Ordering::Greater => {
                println!("{} {}", "▼".cyan().bold(), guess);
                y = guess;
                thread::sleep(step_sleep_timer);
            },
            Ordering::Equal => {
                println!("{} {}{}{}", "✓".green().bold(), "The number was guessed in ".green().bold(), attempts.to_string().yellow().bold(), " attempts.".green().bold());
                break;
            }
        }
    }
}

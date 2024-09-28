use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Welcome to the Number Guessing Game!".bold().cyan());

    let mut total_score = 0;

    loop {
        println!("\nChoose a difficulty level:");
        println!("1. Easy (1-10)");
        println!("2. Medium (1-100)");
        println!("3. Hard (1-1000)");
        println!("4. Exit");

        let mut level_input = String::new();
        io::stdin()
            .read_line(&mut level_input)
            .expect("Failed to read line");

        let difficulty = match level_input.trim() {
            "1" => (1, 10, 5),
            "2" => (1, 100, 7),
            "3" => (1, 1000, 10),
            "4" => {
                println!("{}", "\nThanks for playing!".green().bold());
                break;
            }
            _ => {
                println!("{}", "Invalid choice, please select 1, 2, 3, or 4.".red());
                continue;
            }
        };

        let (min, max, max_attempts) = difficulty;
        let secret_number = rand::thread_rng().gen_range(min..=max);
        let mut attempts = 0;

        println!(
            "\nYou have chosen the {} level. You have {} attempts to guess the number between {} and {}.",
            match level_input.trim() {
                "1" => "Easy",
                "2" => "Medium",
                "3" => "Hard",
                _ => "",
            },
            max_attempts,
            min,
            max
        );

        loop {
            if attempts >= max_attempts {
                println!("{}", "\nYou've run out of attempts!".red().bold());
                println!(
                    "The secret number was: {}",
                    secret_number.to_string().yellow()
                );
                break;
            }

            println!(
                "\nAttempt {}/{}: Enter your guess:",
                attempts + 1,
                max_attempts
            );

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Please enter a valid number.".red());
                    continue;
                }
            };

            attempts += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small!".blue()),
                Ordering::Greater => println!("{}", "Too big!".magenta()),
                Ordering::Equal => {
                    println!("{}", "\n🎉 You guessed it! 🎉".green().bold());
                    let score = (max_attempts - attempts + 1) * 10;
                    total_score += score;
                    println!(
                        "You earned {} points! Total score: {}",
                        score.to_string().yellow(),
                        total_score.to_string().cyan()
                    );
                    break;
                }
            }
        }

        println!("{}", "\nDo you want to play again? (yes/no)".bold());
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().eq_ignore_ascii_case("no") {
            println!("{}", "\nThanks for playing!".green().bold());
            break;
        }
    }
}

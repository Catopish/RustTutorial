use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("ctrl+c for aborting the program");

    loop {
        println!("Input your number : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,

            Err(_) => {
                println!("\nPlease input number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nYour guess is smaller!"),
            Ordering::Greater => println!("\nYour guess is bigger!"),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            }
        }
    }
}

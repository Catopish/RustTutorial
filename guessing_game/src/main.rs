use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    //example of using const, all upercase and underscore between words
    //const SECRET_NUMBER == rand::thread_rng().gen_range(1..=100);

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("ctrl+c for aborting the program");

    loop {
        println!("Input your number : ");

        //declaring variable to guess
        let mut guess = String::new();

        //for user inputting numbers
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //converting input into number, and check if it string
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

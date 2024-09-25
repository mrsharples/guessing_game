use std::io;
use std::cmp::Ordering;
use rand::Rng;

use guessing_game::Guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess_count = 0;

    // println!("The secret number is: {secret_number}");

    loop {
        println!("\nPlease input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("\nYou guessed: {}", guess.value());

        guess_count += 1;

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("It took {} guesses.", guess_count);
                break;
            },
        }
    }

}
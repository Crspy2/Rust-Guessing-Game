use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read input");

        let guess: i32 = guess.trim().parse().expect("Please enter a valid number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\x1B[31;1mYou guessed too low!\x1B[0m"),
            Ordering::Greater => println!("\x1B[31;1mYou guessed too high!\x1B[0m"),
            Ordering::Equal => {
                println!("\x1B[32;1mYou guessed the number!\x1B[0m");
                break;
            }
        }
    }
}

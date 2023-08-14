use std::io;    // User entry
use rand::Rng;  // For random number

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);      // Generate a number from 1 to 100

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        . read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

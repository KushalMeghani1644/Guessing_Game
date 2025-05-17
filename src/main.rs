use rand::Rng;
use std::io;
fn main() {
    //Rust code for a number guessing game
    let secret_number = rand::random_range(1..=100);
    println!("Welcome to the Number Guessing Game!");
    println!("In how many attempts do you want to guess the number: ");
    let mut attempts = String::new();
    io::stdin()
        .read_line(&mut attempts)
        .expect("Failed to read line");
    let max_attempts: u32 = attempts.trim().parse().expect("Please enter a number");
    println!("You have {} attempts to guess the number!", max_attempts);
    for attempts in 1..=max_attempts {
        let mut user_guess = String::new();
        println!("Please enter your guess: ");
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        let guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        if guess == secret_number {
            println!("Congrats! You guesses the number!");
        } else if guess < secret_number {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }
    println!("Game over! The secret number was: {}", secret_number);
}

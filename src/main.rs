use rand::Rng;
use std::io;
fn main() {
    //Rust code for a number guessing game
    println!("Welcome to the Number Guessing Game!");
    let secret_number = loop {
        println!("Choose difficulty level: ");
        println!("1. Easy(1 to 40)");
        println!("2. Medium(1 to 60)");
        println!("3. Hard(1 to 100)");

        let mut difficulty_input = String::new();
        io::stdin()
            .read_line(&mut difficulty_input)
            .expect("Failed to read line!");

        match difficulty_input.trim() {
            "1" => break rand::random_range(1..=40),
            "2" => break rand::random_range(1..=60),
            "3" => break rand::random_range(1..=100),
            _ => println!("Please enter 1, 2, or 3."),
        }
    };
    let max_attempts = loop {
        println!("In how many attempts can you guess the number(max 50): ");

        let mut attempts_input = String::new();
        io::stdin()
            .read_line(&mut attempts_input)
            .expect("Failed to read line");

        match attempts_input.trim().parse::<u32>() {
            Ok(num) if num > 0 && num <= 50 => break num,
            Ok(_) => println!("You can't have more than 50 attempts!"),
            _ => println!("Please enter a valid positive number!"),
        }
    };

    println!("You have {} attempts to guess the number!", max_attempts);

    let mut attempts = 0;
    while attempts < max_attempts {
        let mut user_guess = String::new();
        println!(
            "Attempt {}/{} - Please enter your guess: ",
            attempts + 1,
            max_attempts
        );
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

        attempts += 1;

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

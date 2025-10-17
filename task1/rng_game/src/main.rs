use rand::Rng;
use std::{io, process::exit};

fn repeat_game() {
    println!("Hello, world!");
    println!("This is a Rust Guessing Game program.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().parse::<u32>().unwrap() == secret_number {
            println!("Congratulations! You guessed the secret number.");
            println!("to continue the game press 1 else any other number(only) to exit");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            let user_input: u32 = user_input.trim().parse().expect("Please enter a number!");

            if user_input == 1 {
                println!("the game will restart now!");
                repeat_game();
            } else {
                println!("Thank you for playing!");
                break;
                exit(0);
            }
        } else {
            if (guess.trim().parse::<u32>().unwrap() < secret_number) {
                println!("Too small!");
            } else {
                println!("Too big!");
            }
        }
    }
}
fn main() {
    repeat_game();
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        attempts += 1;

        if attempts == 5 {
            println!("You've used all your attempts! The secret number was: {secret_number}");
            break;
        }

        println!("You have {} attempts left.", 5 - attempts);
    }
}

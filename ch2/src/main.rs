use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Read a line from standard input into a new, empty string
        let mut guess = String::new();

        // Print a prompt and flush stdout (no newline)
        print!("Please input your guess: ");

        // Read a line from stdin and store it in the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string to a number
        // If it fails, continue the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the guess
        println!("You guessed: {}", guess);

        // Match the guess to the secret number
        // Break the loop if the guess is correct
        // Print a hint if it's incorrect
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            _ => {
                println!("You win!");
                break;
            }
        }
    }

}

use std::io; // Import the io module

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Fix the method call syntax
    println!("You guessed: {}", guess);
}

use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn read_input() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("DEBUG: You inputted: {}", input);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("Failed to open log file");
    writeln!(file, "User inputted: {}", input).expect("Failed to write to log file");

    if input.trim() != "exit" {
        read_input();
    }
}

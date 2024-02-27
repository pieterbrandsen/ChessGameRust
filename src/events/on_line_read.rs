use super::super::handlers::command_handler;
use std::io::{self};

#[allow(unconditional_recursion)]
pub fn read_input() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    command_handler::handle_command(input);
    read_input();
}

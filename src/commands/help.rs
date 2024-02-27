use crate::commands;

pub fn execute_command() {
    println!("List of commands:");
    commands::exit::help();
    println!("End of list");
}

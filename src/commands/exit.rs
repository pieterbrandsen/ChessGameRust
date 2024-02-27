// publish help string of exit command
pub fn help() {
    println!("* exit: Exit the program");
}

pub fn execute_command() {
    println!("Exiting application...");
    std::process::exit(0)
}

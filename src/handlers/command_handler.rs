use super::super::chess::state_management;
use super::super::commands;

pub fn handle_command(command: String) {
    match command.trim() {
        "help" => commands::help::execute_command(),
        "exit" => commands::exit::execute_command(),
        _ => {
            let moved = state_management::handle_partial_move(command);
            if moved && state_management::side_is_checkmate() {
                println!("Checkmate! Game over.");
                commands::exit::execute_command();
            }
        }
    }
}

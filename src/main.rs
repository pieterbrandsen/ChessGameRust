mod events {
    pub mod on_line_read; // Import the on_line_read module from the events folder
}

mod handlers {
    pub mod command_handler;
}

mod commands {
    pub mod exit;
    pub mod help;
    pub mod move_piece;
}

mod chess {
    pub mod state_management;
}

fn main() {
    println!("White moves first, type first piece to move, then where to. Type 'help' for a list of commands.");
    events::on_line_read::read_input(); // Call the guess_number function from the on_line_read module
}

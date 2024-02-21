mod events {
    pub mod on_line_read; // Import the on_line_read module from the events folder
}

fn main() {
    events::on_line_read::read_input(); // Call the guess_number function from the on_line_read module
}

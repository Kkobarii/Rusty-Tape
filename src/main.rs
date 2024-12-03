pub mod parser;
pub mod ram;
pub mod ui;

use crate::parser::Parser;
use crate::ui::UiHandler;

fn main() {
    let filename = "data/read_first_half.ram";
    match Parser::parse_file(filename) {
        Ok(machine) => {
            let mut ui = UiHandler {
                machine, // Load with a program
            };
            ui.run().expect("TODO: panic message");
        }
        Err(message) => {
            println!("\nMachine {filename} failed to parse:\n{message}")
        }
    }


    ();
}

pub mod parser;
pub mod ui;
pub mod ram;

use crate::parser::Parser;
use crate::ui::UiHandler;

fn main() {
    let filename = "data/load_store.ram";
    match Parser::parse_file(filename) {
        Ok(machine) => {
            let mut ui = UiHandler {
                machine: machine.with_input(vec![1, 2, 3, 4]), // Load with a program
            };
            ui.run().expect("TODO: panic message");
        }
        Err(message) => {
            println!("\nMachine {filename} failed to parse:\n{message}")
        }
    }


    ();
}

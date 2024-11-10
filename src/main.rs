pub mod parser;
pub mod ram;

use parser::Parser;

fn main() {
    let filename = "data/read_first_half.ram";
    match Parser::parse_file(filename) {
        Ok(machine) => {
            println!("\nMachine {filename} parsed successfully:\n{:?}", machine)
        }
        Err(message) => {
            println!("\nMachine {filename} failed to parse:\n{message}")
        }
    }
}

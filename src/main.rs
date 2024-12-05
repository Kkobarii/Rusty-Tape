pub mod parser;
pub mod ui;
pub mod ram;

use crate::ui::handler::UiHandler;

fn main() {
    let mut ui = UiHandler::default();
    ui.run().expect("TODO: panic message");
}

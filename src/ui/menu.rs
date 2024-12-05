use crate::parser::Parser;
use crate::ram::machine::RamMachine;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub enum MenuHandleResult {
    Continue,
    Machine(RamMachine),
    Exit,
}

pub struct Menu {
    logo: String,
    input: String,
    error: Option<String>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            logo: r#"
    ____             __           ______               
   / __ \__  _______/ /___  __   /_  __/___ _____  ___ 
  / /_/ / / / / ___/ __/ / / /    / / / __ `/ __ \/ _ \
 / _, _/ /_/ (__  ) /_/ /_/ /    / / / /_/ / /_/ /  __/
/_/ |_|\__,_/____/\__/\__, /    /_/  \__,_/ .___/\___/ 
                     /____/              /_/           
         
"#.to_string(),
            input: String::new(),
            error: None,
        }
    }

    pub fn draw_frame(&self, f: &mut Frame) {
        let [title, info, error, input] = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(50), // Title
                    Constraint::Percentage(10), // Info
                    Constraint::Length(3), // Error
                    Constraint::Length(3), // Input
                ].as_ref())
            .areas(f.area());

        f.render_widget(self.draw_logo(), title);
        f.render_widget(self.draw_info(), info);
        if let Some(error_message) = &self.error {
            f.render_widget(Paragraph::new(error_message.clone()).block(Block::default().borders(Borders::ALL).title("Error")), error);
        }
        f.render_widget(self.draw_input(), input);
    }

    fn draw_info(&self) -> Paragraph {
        let info_text = "Welcome to Rusty Tape, my own Rust-powered RAM simulator.\n\
                     Please input the path to your RAM code below.";
        Paragraph::new(info_text).block(Block::default()).centered()
    }

    fn draw_input(&self) -> Paragraph {
        Paragraph::new(self.input.clone())
            .block(Block::default().borders(Borders::ALL).title("Input"))
    }

    pub fn draw_logo(&self) -> Paragraph {
        Paragraph::new(self.logo.clone()).centered()
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> MenuHandleResult {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
                MenuHandleResult::Continue
            }
            KeyCode::Backspace => {
                self.input.pop();
                MenuHandleResult::Continue
            }
            KeyCode::Enter => {
                match self.parse_machine() {
                    Ok(machine) => MenuHandleResult::Machine(machine.with_input(vec![5,1,2,3,4,5])),
                    Err(err) => {
                        self.error = Some(err);
                        MenuHandleResult::Continue
                    }
                }
            }
            KeyCode::Esc => MenuHandleResult::Exit,
            _ => MenuHandleResult::Continue,
        }
    }

    fn parse_machine(&self) -> Result<RamMachine, String> {
        let path = &self.input;
        Parser::parse_file(path)
    }
}
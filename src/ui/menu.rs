use crate::parser::Parser;
use crate::ram::machine::RamMachine;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph};
use ratatui::Frame;

pub enum MenuHandleResult {
    Continue,
    Machine(String, RamMachine),
    Exit,
}

#[derive(PartialEq)]
pub enum MenuState {
    SelectingFile,
    SpecifyingInput,
}

pub struct Menu {
    logo: String,
    input: String,
    error: Option<String>,
    found_files: Vec<String>,
    state: MenuState,
    selected_file: Option<usize>,
    selected_machine: Option<RamMachine>,
}

impl Menu {
    pub fn new() -> Menu {
        let logo = r#"
    ____             __           ______               
   / __ \__  _______/ /___  __   /_  __/___ _____  ___ 
  / /_/ / / / / ___/ __/ / / /    / / / __ `/ __ \/ _ \
 / _, _/ /_/ (__  ) /_/ /_/ /    / / / /_/ / /_/ /  __/
/_/ |_|\__,_/____/\__/\__, /    /_/  \__,_/ .___/\___/ 
                     /____/              /_/           
                       
"#.to_string();

        let mut menu = Menu {
            logo,
            input: String::new(),
            error: None,
            found_files: Vec::new(),
            state: MenuState::SelectingFile,
            selected_file: None,
            selected_machine: None,
        };
        menu.scan_for_files();
        menu
    }

    pub fn draw_frame(&self, f: &mut Frame) {
        // Initialize layout
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Min(60), // Left
                    Constraint::Percentage(100), // Right
                ].as_ref())
            .areas(f.area());

        let [title_area, table_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(15), // Title
                    Constraint::Min(0), // Table
                ].as_ref())
            .areas(left);

        // min height is number of lines if error is present
        let error_min_height = match &self.error {
            Some(err) => err.lines().count() as u16 + 2,
            None => 0,
        };
        let [code_area, error_area, input_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(100), // Code
                    Constraint::Min(error_min_height), // Error
                    Constraint::Min(5), // Input
                ].as_ref())
            .areas(right);

        // Draw widgets
        f.render_widget(self.draw_title(), title_area);
        f.render_widget(self.draw_file_list(), table_area);
        f.render_widget(self.draw_code(), code_area);
        if self.error.is_some() {
            f.render_widget(self.draw_error(), error_area);
        }
        f.render_widget(self.draw_input(), input_area);
    }

    pub fn draw_title(&self) -> List {
        let indent = " ";

        let title_widget = ListItem::new(Text::from(self.logo.clone()).centered().fg(Color::Yellow));

        let greeting = "Welcome to Rusty Tape, my own Rust-powered\n RAM machine simulator!";
        let greeting_widget = ListItem::new(Text::from(format!("{}{}", indent, greeting)));

        let info_widgets = match self.state {
            MenuState::SelectingFile => vec![
                "Up/Down arrows to select a file.",
                "Enter to load the file.",
                "Esc to exit.",
            ],
            MenuState::SpecifyingInput => vec![
                "Please enter the input tape.",
                "Enter to confirm.",
                "Esc to go back.",
            ],
        }.into_iter()
            .map(|line| format!("{}{}", indent, line))
            .map(|line| ListItem::new(Text::from(line)))
            .collect::<Vec<ListItem>>();

        let mut items = vec![title_widget, greeting_widget, ListItem::new(Span::raw(""))];
        items.extend(info_widgets);

        List::new(items)
            .block(Block::default().borders(Borders::NONE).padding(Padding::symmetric(1, 0)))
    }

    fn draw_file_list(&self) -> List {
        let border_color = match self.state {
            MenuState::SelectingFile => Color::White,
            MenuState::SpecifyingInput => Color::DarkGray,
        };

        let items: Vec<ListItem> = self.found_files.iter().enumerate().map(|(i, file)| {
            let (arrow, style) = if Some(i) == self.selected_file {
                ("> ", Style::default().fg(Color::Yellow))
            } else {
                ("  ", Style::default())
            };

            let path_parts: Vec<&str> = file.split('/').collect();
            let (dir_path, filename) = if path_parts.len() > 1 {
                (path_parts[..path_parts.len() - 1].join("/") + "/", path_parts[path_parts.len() - 1])
            } else {
                ("".to_string(), file.as_str())
            };

            let arrow_span = Span::styled(arrow, style);
            let dir_span = Span::styled(dir_path, Style::default().fg(Color::DarkGray));
            let file_span = Span::styled(filename, style);

            ListItem::new(Line::from(vec![arrow_span, dir_span, file_span]))
        }).collect();

        List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .fg(border_color)
                .title("Files")
                .padding(Padding::symmetric(1, 1))
            )
    }

    fn draw_code(&self) -> List {
        if let Some(selected_file) = self.selected_file {
            if let Ok(contents) = std::fs::read_to_string(&self.found_files[selected_file]) {
                let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
                let max_index_width = lines.len().to_string().len();

                let items: Vec<ListItem> = lines.iter().enumerate()
                    .map(|(i, line)| {
                        let index_span = Span::styled(
                            format!("{:width$}| ", i, width = max_index_width),
                            Style::default().fg(Color::DarkGray),
                        );

                        let line_span = Span::styled(
                            line.clone(),
                            Style::default(),
                        );

                        ListItem::new(Line::from(vec![index_span, line_span]))
                    })
                    .collect();

                return List::new(items)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("Code")
                        .padding(Padding::symmetric(1, 1))
                    )
            }
        }

        List::new::<Vec<ListItem>>(vec![])
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Code")
                .padding(Padding::symmetric(1, 1))
            )
    }
    
    fn draw_error(&self) -> Paragraph {
        Paragraph::new(self.error.clone().unwrap()).fg(Color::Red)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::White)
                    .title("Error")
                    .padding(Padding::horizontal(2))
            )
    }

    fn draw_input(&self) -> Paragraph {
        let border_color = match self.state {
            MenuState::SelectingFile => Color::DarkGray,
            MenuState::SpecifyingInput => Color::White,
        };

        Paragraph::new(format!("[{}]", self.input))
            .block(Block::default()
                .borders(Borders::ALL)
                .fg(border_color)
                .title("Input")
                .padding(Padding::symmetric(2, 1))
            )
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> MenuHandleResult {
        match self.state {
            MenuState::SelectingFile => self.handle_selecting_file(key),
            MenuState::SpecifyingInput => self.handle_specifying_input(key),
        }
    }

    fn handle_selecting_file(&mut self, key: KeyEvent) -> MenuHandleResult {
        match key.code {
            // move selection up
            KeyCode::Up => {
                self.move_file_selection(-1);
                MenuHandleResult::Continue
            }
            // move selection down
            KeyCode::Down => {
                self.move_file_selection(1);
                MenuHandleResult::Continue
            }
            // confirm file selection
            KeyCode::Enter => self.confirm_file_selection(),
            // exit
            KeyCode::Esc => MenuHandleResult::Exit,
            // ignore other keys
            _ => MenuHandleResult::Continue,
        }
    }

    fn handle_specifying_input(&mut self, key: KeyEvent) -> MenuHandleResult {
        match key.code {
            // add character to input
            KeyCode::Char(c) => {
                self.input_char(c);
                MenuHandleResult::Continue
            }
            // remove last character from input
            KeyCode::Backspace => {
                self.delete_char();
                MenuHandleResult::Continue
            }
            // confirm input
            KeyCode::Enter => self.confirm_input(),
            // return to file selection
            KeyCode::Esc => {
                self.state = MenuState::SelectingFile;
                MenuHandleResult::Continue
            }
            // ignore other keys
            _ => MenuHandleResult::Continue,
        }
    }

    fn move_file_selection(&mut self, direction: i32) {
        self.error = None;
        if let Some(selected) = self.selected_file {
            let new_selection = selected as i32 + direction;
            if new_selection >= 0 && new_selection < self.found_files.len() as i32 {
                self.selected_file = Some(new_selection as usize);
            }
        } else {
            self.selected_file = Some(0);
        }
    }

    fn confirm_file_selection(&mut self) -> MenuHandleResult {
        if let Some(selected) = self.selected_file {
            match self.parse_machine(&self.found_files[selected]) {
                Ok(machine) => {
                    self.error = None;
                    self.state = MenuState::SpecifyingInput;
                    self.selected_machine = Some(machine);
                    MenuHandleResult::Continue
                }
                Err(err) => {
                    self.error = Some(err);
                    MenuHandleResult::Continue
                }
            }
        } else {
            self.error = Some("No file selected".to_string());
            MenuHandleResult::Continue
        }
    }

    fn input_char(&mut self, c: char) {
        // if char is a digit, add it to the input
        // if char is a comma or space, add ", " to the input
        match c {
            '0'..='9' => {
                // if the last character is minus, do not add 0
                if self.input.ends_with('-') && c == '0' {
                    return;
                }
                // if the whole previous number is 0, do not add another char
                if let Some(number) = self.input.rsplit(", ").next() { 
                    if number == "0" {
                        return;
                    }
                }
                self.input.push(c);
            }
            ',' | ' ' => {
                // if the last character is a comma or space, ignore
                if self.input.is_empty() || self.input.ends_with(", ") || self.input.ends_with('-') {
                    return;
                }
                self.input.push_str(", ");
            }
            '-' => {
                // if the last character is a comma or space, add it
                // otherwise, ignore
                if self.input.is_empty() || self.input.ends_with(", ") {
                    self.input.push(c);
                }
            }
            _ => {}
        }
    }

    fn delete_char(&mut self) {
        // if the last character is a space, remove ", " from the input
        // otherwise, remove the last character
        if self.input.ends_with(", ") {
            self.input = self.input[..self.input.len() - 2].to_string();
        } else {
            self.input.pop();
        }
    }

    fn confirm_input(&mut self) -> MenuHandleResult {
        match self.parse_input() {
            Ok(input_tape) => {
                self.error = None;
                self.state = MenuState::SelectingFile;
                if let Some(machine) = self.selected_machine.take() {
                    let filename = self.found_files[self.selected_file.unwrap()]
                        .clone()
                        .split('/')
                        .last()
                        .unwrap()
                        .to_string()
                        .replace(".ram", "");
                    MenuHandleResult::Machine(filename, machine.with_input(input_tape))
                } else {
                    self.error = Some("The machine somehow escaped".to_string());
                    MenuHandleResult::Continue
                }
            }
            Err(err) => {
                self.error = Some(err);
                MenuHandleResult::Continue
            }
        }
    }

    fn scan_for_files(&mut self) {
        self.found_files = walkdir::WalkDir::new(".")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "ram").unwrap_or(false))
            .map(|entry| entry.path().display().to_string())
            .collect();
    }

    fn parse_input(&self) -> Result<Vec<i32>, String> {
        if self.input.is_empty() {
            return Ok(Vec::new());
        }

        if self.input.trim().ends_with(',') {
            return Err("Input cannot end with a comma.".to_string());
        }

        self.input.split(',')
            .map(|s| {
                let trimmed = s.trim();
                trimmed.parse::<i32>().map_err(|_| {
                    if trimmed.parse::<i64>().is_ok() {
                        format!(
                            "Input value {trimmed} is too {}.", 
                            if trimmed.starts_with('-') { "small" } else { "large" }
                        )
                    } else {
                        "Invalid input.".to_string()
                    }
                })
            })
            .collect()
    }

    fn parse_machine(&self, filename: &str) -> Result<RamMachine, String> {
        Parser::parse_file(filename)
    }
}
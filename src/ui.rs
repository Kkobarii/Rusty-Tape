use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal::{disable_raw_mode, enable_raw_mode},
};
use crate::ram::RamMachine;

/// The UI handler struct.
pub struct UiHandler {
    pub machine: RamMachine,
}

impl UiHandler {
    /// Initializes the UI and handles events.
    pub fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, ratatui::crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(60), // Code Section
                            Constraint::Percentage(30), // Memory Section
                            Constraint::Percentage(10), // Tape Section
                        ]
                            .as_ref(),
                    )
                    .split(f.size());

                // Code Section
                let code_items: Vec<ListItem> = self
                    .machine.get_program()
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let style = if i == self.machine.get_instruction_pointer() {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };

                        let line = if let Some(label) = line.label.as_ref() {
                            format!("{}: {}", label, line.op)
                        } else {
                            format!("{}", line.op)
                        };

                        ListItem::new(Span::styled(line.clone(), style))
                    })
                    .collect();
                let code_list = List::new(code_items).block(Block::default().borders(Borders::ALL).title("Code"));
                f.render_widget(code_list, chunks[0]);

                // Memory Section
                let memory_items: Vec<Span> = self
                    .machine.get_whole_memory()
                    .iter()
                    .map(|(i, &value)| {
                        let style = if *i == self.machine.get_instruction_pointer() {
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };
                        Span::styled(format!("[{}] {}", i, value), style)
                    })
                    .collect();
                let memory_block = List::new(memory_items).block(Block::default().borders(Borders::ALL).title("Memory"));
                f.render_widget(memory_block, chunks[1]);

                // Tape Section
                let tape_text = format!(
                    "Input Tape: {:?}\nOutput Tape: {:?}",
                    self.machine.get_input(), self.machine.get_output()
                );
                let tape_block =
                    Paragraph::new(tape_text).block(Block::default().borders(Borders::ALL).title("Tape"));
                f.render_widget(tape_block, chunks[2]);
            })?;

            // Handle input events
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(' ') => {
                        // Step instruction
                        match self.machine.step() {
                            Ok(_) => {}
                            Err(message) => {
                                return Err(io::Error::new(io::ErrorKind::Other, message));
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Exit to menu
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            ratatui::crossterm::terminal::LeaveAlternateScreen
        )?;
        Ok(())
    }
}

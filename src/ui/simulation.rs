use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Span, Style, Line};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::ram::instruction_op::InstructionOp::Empty;
use crate::ram::machine::RamMachine;

pub enum SimulationHandleResult {
    Continue,
    Finish,
    Exit,
    Error(String),
}

pub struct Simulation {
    machine: RamMachine,
    label_indent: usize
}

impl Simulation {
    pub fn new(machine: RamMachine) -> Self {
        // label indent is longest label name
        let label_indent = machine.get_program().iter()
            .filter_map(|instruction| instruction.label.as_ref())
            .map(|label| label.len())
            .max()
            .unwrap_or(0);
        
        Simulation { machine, label_indent }
    }

    pub fn draw_frame(&self, f: &mut Frame) {
        let [left, memory_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(85), // Left
                    Constraint::Percentage(20), // Memory
                ].as_ref(),
            )
            .areas(f.area());

        let [code_area, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70), // Code
                    Constraint::Percentage(30), // Bottom
                ].as_ref(),
            )
            .areas(left);
        
        let [info_area, tapes] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30), // Info
                    Constraint::Percentage(70), // Tapes
                ].as_ref(),
            )
            .areas(bottom);
        
        let [input_tape_area, output_tape_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), // Input
                    Constraint::Percentage(50), // Output
                ].as_ref(),
            )
            .areas(tapes);

        f.render_widget(self.draw_code(), code_area);
        f.render_widget(self.draw_memory(), memory_area);
        f.render_widget(self.draw_input_tape(), input_tape_area);
        f.render_widget(self.draw_output_tape(), output_tape_area);
        f.render_widget(self.draw_info(), info_area);
    }

    fn draw_info(&self) -> Paragraph {
        let info_text = format!("Instruction Pointer: {}", self.machine.get_instruction_pointer());
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Info"))
    }

    fn draw_output_tape(&self) -> Paragraph {
        let output_tape_text = format!("{:?}", self.machine.get_output());
        Paragraph::new(output_tape_text).block(Block::default().borders(Borders::ALL).title("Output tape"))
    }

    fn draw_input_tape(&self) -> Paragraph {
        let input_tape_text = format!("{:?}", self.machine.get_input());
        Paragraph::new(input_tape_text).block(Block::default().borders(Borders::ALL).title("Input tape"))
    }

    fn draw_memory(&self) -> List {
        let mut memory_items: Vec<(&i32, &i32)> = self.machine.get_memory().iter().collect();
        memory_items.sort_by_key(|&(i, _)| i);
        let memory_list_items: Vec<ListItem> = memory_items
            .iter()
            .map(|(i, &value)| {
                ListItem::new(Span::from(format!("[{}] {}", i, value)))
            })
            .collect();

        List::new(memory_list_items).block(Block::default().borders(Borders::ALL).title("Memory"))
    }

    fn draw_code(&self) -> List {
        let code_items: Vec<ListItem> = self
            .machine.get_program()
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let label_span = if let Some(label) = &line.label {
                    let label_str = format!("{}:", label);
                    Span::styled(
                        format!("{:width$} ", label_str, width = self.label_indent + 1),
                        Style::default().fg(Color::Gray),
                    )
                } else {
                    Span::raw(" ".repeat(self.label_indent + 2))
                };

                let line_span = Span::styled(
                    format!("{}", line.op),
                    if i == self.machine.get_instruction_pointer() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    },
                );

                let comment_span = if let Some(comment) = &line.comment {
                    if line.op == Empty {
                        Span::styled(format!("# {}", comment), Style::default().fg(Color::DarkGray))
                    } else {
                        Span::styled(format!(" # {}", comment), Style::default().fg(Color::DarkGray))
                    }
                } else {
                    Span::raw("")
                };

                ListItem::new(Line::from(vec![label_span, line_span, comment_span]))
            })
            .collect();

        List::new(code_items).block(Block::default().borders(Borders::ALL).title("Code"))
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> SimulationHandleResult {
        match key.code {
            KeyCode::Char(' ') => {
                // Step instruction
                match self.machine.step() {
                    Ok(true) => SimulationHandleResult::Finish,
                    Ok(false) => SimulationHandleResult::Continue,
                    Err(message) => SimulationHandleResult::Error(message)
                }
            }
            KeyCode::Esc => SimulationHandleResult::Exit,
            _ => SimulationHandleResult::Continue,
        }
    }
}
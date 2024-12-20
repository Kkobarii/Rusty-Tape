use crate::ram::instruction_op::InstructionOp::Empty;
use crate::ram::machine::RamMachine;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding};
use ratatui::Frame;
use ratatui::style::Stylize;

pub enum SimulationHandleResult {
    Continue,
    Exit,
}

#[derive(PartialEq)]
pub enum SimulationState {
    Running,
    Finished,
    Error(String),
}

pub struct Simulation {
    name: String,
    machine: RamMachine,
    label_indent: usize,
    state: SimulationState,
}

impl Simulation {
    pub fn new(name: String, machine: RamMachine) -> Self {
        let label_indent = machine.get_program().iter()
            .filter_map(|instruction| instruction.label.as_ref())
            .map(|label| label.len())
            .max()
            .unwrap_or(0);
        
        Simulation {
            name,
            machine,
            label_indent,
            state: SimulationState::Running
        }
    }

    pub fn draw_frame(&self, f: &mut Frame) {
        // Split the frame into areas
        let [left, memory_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(100), // Left
                    Constraint::Min(20), // Memory
                ].as_ref(),
            )
            .areas(f.area());

        let [code_area, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(100), // Code
                    Constraint::Min(12), // Bottom
                ].as_ref(),
            )
            .areas(left);
        
        let [info_area, tapes] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Min(40), // Info
                    Constraint::Percentage(100), // Tapes
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

        // Draw widgets
        f.render_widget(self.draw_code(), code_area);
        f.render_widget(self.draw_memory(), memory_area);
        f.render_widget(self.draw_input_tape(), input_tape_area);
        f.render_widget(self.draw_output_tape(), output_tape_area);
        f.render_widget(self.draw_info(), info_area);
    }

    fn draw_info(&self) -> List {
        let mut info_widgets: Vec<ListItem> = vec![
            ListItem::new(Text::from(format!("Running simulation: {}", self.name))),
            ListItem::new(Text::from(format!("│ Instruction Pointer: {}", self.machine.get_instruction_pointer()))),
            ListItem::new(Text::from(format!("│ Input Pointer: {}", self.machine.get_input_pointer()))),
            ListItem::new(Text::from(format!("│ Output Tape Length: {}", self.machine.get_output().len()))),
            ListItem::new(Text::raw("")),
        ];

        match &self.state {
            SimulationState::Error(message) => {
                info_widgets.push(ListItem::new(Text::styled(
                    "Error!",
                    Style::default().fg(Color::Red),
                )));
                info_widgets.push(ListItem::new(Text::styled(
                    format!("│ {}", message),
                    Style::default().fg(Color::Red),
                )));
                info_widgets.push(ListItem::new(Text::styled(
                    "│ Press 'Esc' to exit",
                    Style::default().fg(Color::Red),
                )));
            }
            SimulationState::Finished => {
                info_widgets.push(ListItem::new(Text::styled(
                    "Simulation Finished!",
                    Style::default().fg(Color::Green),
                )));
                info_widgets.push(ListItem::new(Text::styled(
                    "│ Press 'Esc' to exit",
                    Style::default().fg(Color::Green),
                )));
            }
            _ => {}
        }

        List::new(info_widgets)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Info")
                .padding(Padding::symmetric(2, 1))
            )
    }

    fn draw_output_tape(&self) -> List {
        let mut pointer_line_spans = vec![Span::raw(" ")];
        let mut number_line_spans = vec![Span::raw("[")];

        for value in self.machine.get_output() {
            let width = value.to_string().len();

            // Add formatted number and space for pointer
            number_line_spans.push(Span::raw(format!("{:width$}, ", value, width = width)));
            pointer_line_spans.push(Span::raw(format!("{:width$}  ", " ", width = width)));
        }

        // Add the final pointer and placeholder
        pointer_line_spans.push(Span::styled("^", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        number_line_spans.push(Span::styled("_", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        number_line_spans.push(Span::raw("]"));

        let pointer_line = Line::from(pointer_line_spans);
        let number_line = Line::from(number_line_spans);

        List::new(vec![
            ListItem::new(number_line),
            ListItem::new(pointer_line),
        ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Output tape")
                    .fg(match self.state {
                        SimulationState::Error(_) | SimulationState::Finished => Color::DarkGray,
                        _ => Color::Reset,
                    })
                    .padding(Padding::symmetric(2, 1)),
            )
    }

    fn draw_input_tape(&self) -> List {
        let input_pointer = self.machine.get_input_pointer();
        let input_values = self.machine.get_input();

        let mut pointer_line_spans = vec![Span::raw(" ")];
        let mut number_line_spans = vec![Span::raw("[")];

        for (i, value) in input_values.iter().enumerate() {
            let is_pointer = i == input_pointer;
            let style = match i {
                _ if is_pointer => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                _ if i < input_pointer => Style::default().fg(Color::DarkGray),
                _ => Style::default(),
            };
            let pointer_style = if is_pointer {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let value_width = value.to_string().len();
            let separator = if i == input_values.len() - 1 { "" } else { ", " };

            number_line_spans.push(Span::styled(format!("{:width$}{}", value, separator, width = value_width), style));

            pointer_line_spans.extend([
                Span::raw(" ".repeat(value_width - 1)),
                Span::styled(if is_pointer { "^" } else { " " }, pointer_style),
                Span::raw(" ".repeat(separator.len())),
            ]);
        }

        // Handle the case where the pointer is at the end
        if input_pointer == input_values.len() {
            pointer_line_spans.extend([
                Span::styled(
                    if input_pointer == 0 { "" } else { "  " },
                    Style::default()
                ),
                Span::styled(
                    "^", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]);

            number_line_spans.extend([
                Span::styled(
                    if input_pointer == 0 { "" } else { ", " },
                    Style::default().fg(Color::DarkGray)
                ),
                Span::styled(
                    "_", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]);
        } 

        number_line_spans.push(Span::raw("]"));

        let input_tape_items = vec![
            ListItem::new(Line::from(number_line_spans)),
            ListItem::new(Line::from(pointer_line_spans)),
        ];

        List::new(input_tape_items).block(
            Block::default()
                .borders(Borders::ALL)
                .fg(match self.state {
                    SimulationState::Error(_) | SimulationState::Finished => Color::DarkGray,
                    _ => Color::Reset,
                })
                .title("Input tape")
                .padding(Padding::symmetric(2, 1)),
        )
    }

    fn draw_memory(&self) -> List {
        let mut memory_items: Vec<(&i32, &i32)> = self.machine.get_memory().iter().collect();
        memory_items.sort_by_key(|&(i, _)| i);
        let max_index_width = memory_items.last().map_or(0, |(i, _)| i.to_string().len());

        let memory_list_items: Vec<ListItem> = memory_items
            .iter()
            .map(|(i, &value)| {
                let index_span = Span::styled(
                    format!("{:width$}| ", i, width = max_index_width),
                    Style::default().fg(Color::DarkGray)
                );
                let value_span = Span::styled(
                    format!("{}", value),
                    Style::default()
                );
                ListItem::new(Line::from(vec![index_span, value_span]))
            })
            .collect();

        List::new(memory_list_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .fg(match self.state {
                    SimulationState::Error(_) | SimulationState::Finished => Color::DarkGray,
                    _ => Color::Reset,
                })
                .title("Memory")
                .padding(Padding::symmetric(2, 1))
            )
    }

    fn draw_code(&self) -> List {
        let max_index_width = self.machine.get_program().len().to_string().len();
        let code_items: Vec<ListItem> = self
            .machine.get_program()
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let index_span = Span::styled(
                    format!("{:width$}| ", i, width = max_index_width),
                    if i == self.machine.get_instruction_pointer() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                );

                let label_span = if let Some(label) = &line.label {
                    let label_str = format!("{}:", label);
                    Span::from(format!("{:width$} ", label_str, width = self.label_indent + 1))
                } else {
                    Span::raw(" ".repeat(self.label_indent + 2))
                };

                let line_span = Span::styled(
                    format!("{}", line.op),
                    if i == self.machine.get_instruction_pointer() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    }
                );

                let comment_span = line.comment.as_ref().map_or_else(
                    || Span::raw(""),
                    |comment| {
                        let prefix = if line.op == Empty { "# " } else { " # " };
                        Span::styled(
                            format!("{prefix}{comment}"),
                            Style::default()
                                .fg(Color::DarkGray)
                        )
                    },
                );

                ListItem::new(
                    Line::from(vec![
                        index_span,
                        label_span,
                        line_span,
                        comment_span
                    ])
                )
            })
            .collect();

        List::new(code_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .fg(match self.state {
                    SimulationState::Error(_) | SimulationState::Finished => Color::DarkGray,
                    _ => Color::Reset,
                })
                .title("Code")
                .padding(Padding::symmetric(1, 1))
            )
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> SimulationHandleResult {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => {
                if self.state != SimulationState::Running {
                    return SimulationHandleResult::Continue;
                }
                
                match self.machine.step() {
                    Ok(true) => {
                        self.state = SimulationState::Finished;
                        SimulationHandleResult::Continue
                    }
                    Ok(false) => SimulationHandleResult::Continue,
                    Err(message) => {
                        self.state = SimulationState::Error(message);
                        SimulationHandleResult::Continue
                    }
                }
            }
            KeyCode::Esc => SimulationHandleResult::Exit,
            _ => SimulationHandleResult::Continue,
        }
    }
}
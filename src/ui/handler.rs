use crate::ui::simulation::{SimulationHandleResult, Simulation};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    event::{self, Event},
    execute, terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use std::io;
use crate::ui::menu::{Menu, MenuHandleResult};



/// The UI handler struct.
pub struct UiHandler {
    simulation: Option<Simulation>,
    menu: Menu
}

impl Default for UiHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl UiHandler {
    
    pub fn new() -> Self {
        UiHandler {
            simulation: None,
            menu: Menu::new()
        }
    }
    
    /// Initializes the UI and handles events.
    pub fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, ratatui::crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // let filename = "data/do_everything.ram";
        // let machine = Parser::parse_file(filename).expect("Failed to parse file")
        //     .with_input(vec![5, 1, 2, 3, 4, 5]);
        // self.simulation = Some(Simulation::new(machine));
        
        // Main loop
        let err = loop {
            terminal.draw(|f| {
                if let Some(ref mut simulation) = self.simulation { 
                    simulation.draw_frame(f) 
                } else {
                    self.menu.draw_frame(f)
                }
            })?;

            // Handle input events
            if let Event::Key(key) = event::read()? {
                if let Some(ref mut simulation) = self.simulation { 
                    match simulation.handle_input(key) {
                        SimulationHandleResult::Continue => continue,
                        SimulationHandleResult::Finish => {
                            self.simulation = None;
                            continue;
                        },
                        SimulationHandleResult::Exit => break Ok(()),
                        SimulationHandleResult::Error(message) => break Err(message),
                    }
                } else {
                    match self.menu.handle_input(key) {
                        MenuHandleResult::Continue => continue,
                        MenuHandleResult::Exit => break Ok(()),
                        MenuHandleResult::Machine(machine) => { 
                            self.simulation = Some(Simulation::new(machine));
                        },
                    }
                }
            }
        };

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            ratatui::crossterm::terminal::LeaveAlternateScreen
        )?;
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}

use std::io::{stdout, Stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::Result;
use tui::{backend::CrosstermBackend, Terminal};

pub type ApplicationTerminal = Terminal<CrosstermBackend<Stdout>>;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn run<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut ApplicationTerminal, &mut bool) -> Result<()>,
    {
        enable_raw_mode()?;

        let stdout = stdout();

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let mut keep_running = true;
        while keep_running {
            f(&mut terminal, &mut keep_running)?;
        }

        terminal.clear()?;
        disable_raw_mode()
    }
}

// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

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
        F: FnMut(&mut ApplicationTerminal) -> Result<bool>,
    {
        enable_raw_mode()?;

        let stdout = stdout();

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let mut keep_running = true;
        while keep_running {
            keep_running = f(&mut terminal)?;
        }

        terminal.clear()?;
        disable_raw_mode()
    }
}

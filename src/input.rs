use crossterm::event::{read, Event, KeyCode};
use crossterm::Result;

use crate::ApplicationState;

pub struct InputHandler {
    pub user_input: String,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            user_input: String::default(),
        }
    }

    pub fn handle_event(&mut self, state: &mut ApplicationState) -> Result<()> {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc => {
                    state.keep_running = false;
                }
                KeyCode::Char(c) => {
                    self.user_input.push(c);
                }
                KeyCode::Backspace => {
                    self.user_input.pop();
                }
                KeyCode::Enter => {}
                _ => {}
            };
        }

        Ok(())
    }
}

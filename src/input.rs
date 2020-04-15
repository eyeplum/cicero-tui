use crate::ApplicationState;

use crossterm::{
    event::{read, Event, KeyCode},
    Result,
};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {}
    }

    pub fn handle_event(&self, state: &mut ApplicationState) -> Result<()> {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc => {
                    state.keep_running = false;
                }
                KeyCode::Char(c) => {
                    state.user_input.push(c);
                }
                KeyCode::Backspace => {
                    state.user_input.pop();
                }
                KeyCode::Enter => {}
                _ => {}
            };
        }

        Ok(())
    }
}

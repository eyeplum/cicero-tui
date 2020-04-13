use crate::ApplicationState;

use std::fmt;

use crossterm::{
    event::{read, Event, KeyCode},
    Result,
};

pub enum InputMode {
    Normal,
    Input,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "-- NORMAL --"),
            Self::Input => write!(f, "-- INPUT --"),
        }
    }
}

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {}
    }

    pub fn handle_event(&self, state: &mut ApplicationState) -> Result<()> {
        if let Event::Key(event) = read()? {
            match state.active_input_mode {
                InputMode::Normal => {
                    match event.code {
                        KeyCode::Char('i') => {
                            state.active_input_mode = InputMode::Input;
                            // TODO: Set cursor
                        }
                        KeyCode::Char('q') => {
                            state.keep_running = false;
                        }
                        _ => {}
                    };
                }
                InputMode::Input => {
                    match event.code {
                        KeyCode::Esc => {
                            state.active_input_mode = InputMode::Normal;
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
                    // TODO: Set cursor
                }
            }
        }

        Ok(())
    }
}

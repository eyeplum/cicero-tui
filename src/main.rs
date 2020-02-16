#![warn(clippy::all)]

use std::fmt;
use std::io::stdout;

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

enum InputMode {
    Normal,
    Input,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "-- NORMAL --"),
            Self::Input => write!(f, "-- INPUT --"),
        }
    }
}

#[derive(Default)]
struct Application {
    active_input_mode: InputMode,
    user_input: String,
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let stdout = stdout();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut application = Application::default();

    let mut keep_running = true;
    while keep_running {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(f.size());

            Paragraph::new([Text::raw(&application.user_input)].iter())
                .block(Block::default().title("Input").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);

            Block::default()
                .title("Grapheme Clusters")
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);

            Paragraph::new([Text::raw(application.active_input_mode.to_string())].iter())
                .style(Style::default().fg(Color::Blue))
                .render(&mut f, chunks[2]);
        })?;

        if let Event::Key(event) = read()? {
            match application.active_input_mode {
                InputMode::Normal => {
                    match event.code {
                        KeyCode::Char('i') => {
                            application.active_input_mode = InputMode::Input;
                            // TODO: Set cursor
                        }
                        KeyCode::Char('q') => {
                            keep_running = false;
                        }
                        _ => {}
                    };
                }
                InputMode::Input => {
                    match event.code {
                        KeyCode::Esc => {
                            application.active_input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            application.user_input.push(c);
                        }
                        KeyCode::Backspace => {
                            application.user_input.pop();
                        }
                        KeyCode::Enter => {}
                        _ => {}
                    };
                    // TODO: Set cursor
                }
            }
        }
    }

    terminal.clear()?;
    disable_raw_mode()
}

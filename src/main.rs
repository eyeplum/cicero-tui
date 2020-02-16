use std::io::stdout;

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

enum InputMode {
    Normal,
    Edit,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
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
    terminal.hide_cursor()?;
    terminal.clear()?;

    let mut application = Application::default();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Length(0)].as_ref())
                .direction(Direction::Vertical)
                .split(f.size());

            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);

            Block::default()
                .title("Grapheme Clusters")
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);
        })?;

        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char('e') => {
                    application.active_input_mode = InputMode::Edit;
                }
                KeyCode::Esc => {
                    application.active_input_mode = InputMode::Normal;
                }
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(c) => {
                    application.user_input.push(c);
                }
                KeyCode::Backspace => {
                    application.user_input.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    terminal.clear()?;
    disable_raw_mode()
}

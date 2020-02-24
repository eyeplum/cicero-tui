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
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget},
    Terminal,
};

use unic::segment::Graphemes;

enum InputMode {
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

struct Application {
    active_input_mode: InputMode,
    user_input: String,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            active_input_mode: InputMode::Normal,
            user_input: String::default(),
        }
    }
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

            let graphemes = Graphemes::new(&application.user_input);
            let grapheme_strings = graphemes.fold(vec![], |mut sum, grapheme| {
                let code_points_string =
                    grapheme
                        .chars()
                        .fold("".to_owned(), |mut code_points_sum, chr| {
                            if code_points_sum.is_empty() {
                                code_points_sum = format!("U+{:04X}", chr as u32);
                            } else {
                                code_points_sum =
                                    format!("{}, U+{:04X}", code_points_sum, chr as u32);
                            }
                            code_points_sum
                        });
                sum.push(code_points_string);
                sum
            });
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Graphemes"))
                .items(&grapheme_strings)
                .select(None)
                .style(Style::default())
                .highlight_style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .modifier(Modifier::BOLD),
                )
                .highlight_symbol(">")
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

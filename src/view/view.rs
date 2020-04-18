use std::io::Stdout;

use crossterm::event::{read, Event, KeyCode};
use crossterm::Result;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text};
use tui::Frame;

use crate::renderer::ApplicationTerminal;
use crate::view::stateful_graphemes::StatefulGraphemes;
use crate::ApplicationState;

type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub struct View {
    user_input: String,
    graphemes: StatefulGraphemes,
}

impl View {
    pub fn new() -> Self {
        View {
            user_input: String::default(),
            graphemes: StatefulGraphemes::default(),
        }
    }

    pub fn update(
        &mut self,
        terminal: &mut ApplicationTerminal,
        app_state: &mut ApplicationState,
    ) -> Result<()> {
        terminal.draw(|mut frame| {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(frame.size());

            self.draw_user_input(&mut frame, chunks[0]);
            self.draw_graphemes_list(&mut frame, chunks[1]);
            self.draw_help_text(&mut frame, chunks[2]);
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                // Application
                KeyCode::Esc => app_state.keep_running = false,

                // Grapheme list
                KeyCode::Up => self.graphemes.select_previous(),
                KeyCode::Down => self.graphemes.select_next(),
                KeyCode::Enter => { /* TODO: Show character detail */ }

                // Text input
                KeyCode::Char(c) => {
                    self.user_input.push(c);
                    self.graphemes = StatefulGraphemes::new(&self.user_input);
                }
                KeyCode::Backspace => {
                    self.user_input.pop();
                    self.graphemes = StatefulGraphemes::new(&self.user_input);
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn draw_user_input(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let user_input_items = [Text::raw(&self.user_input)];
        let user_input_paragraph = Paragraph::new(user_input_items.iter())
            .block(Block::default().title("Input").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(user_input_paragraph, rect);
    }

    fn draw_graphemes_list(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let grapheme_items = self.graphemes.rows.iter().map(|row| Text::raw(row));
        let graphemes_list = List::new(grapheme_items.into_iter())
            .block(Block::default().borders(Borders::ALL).title("Graphemes"))
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .modifier(Modifier::BOLD),
            )
            .highlight_symbol(">");

        frame.render_stateful_widget(graphemes_list, rect, &mut self.graphemes.state);
    }

    fn draw_help_text(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let help_item = [Text::raw("ESC to quit")];
        let help_text =
            Paragraph::new(help_item.iter()).style(Style::default().fg(Color::LightGreen));

        frame.render_widget(help_text, rect);
    }
}

use crate::renderer::ApplicationTerminal;

use std::borrow::Cow;
use std::io::Stdout;

use crossterm::Result;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text};
use tui::{Frame, Terminal};

use unic::{segment::Graphemes, ucd::name::Name};

type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub struct View;

impl View {
    pub fn new() -> Self {
        View {}
    }

    pub fn update(&self, terminal: &mut ApplicationTerminal, user_input: &str) -> Result<()> {
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

            View::draw_user_input(user_input, &mut frame, chunks[0]);
            View::draw_graphemes_list(user_input, &mut frame, chunks[1]);
            View::draw_help_text(&mut frame, chunks[2]);
        })?;

        Ok(())
    }

    fn draw_user_input(user_input: &str, frame: &mut TerminalFrame, rect: Rect) {
        let user_input_items = [Text::raw(user_input)];
        let user_input_paragraph = Paragraph::new(user_input_items.iter())
            .block(Block::default().title("Input").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(user_input_paragraph, rect);
    }

    fn draw_graphemes_list(user_input: &str, frame: &mut TerminalFrame, rect: Rect) {
        let graphemes = Graphemes::new(user_input);

        let grapheme_items = graphemes.fold(vec![], |mut sum, grapheme| {
            grapheme.chars().for_each(|chr| {
                let mut code_point_str = format!("U+{:04X}", chr as u32);
                if code_point_str.len() < 8 {
                    code_point_str =
                        format!("{}{}", " ".repeat(8 - code_point_str.len()), code_point_str);
                }
                let name = match Name::of(chr) {
                    None => "".to_owned(),
                    Some(name) => name.to_string(),
                };

                sum.push(Text::Raw(Cow::from(format!(
                    "{}  {}  {}",
                    code_point_str, chr, name
                ))));
            });
            sum.push(Text::Raw(Cow::from("")));
            sum
        });
        let graphemes_list = List::new(grapheme_items.into_iter())
            .block(Block::default().borders(Borders::ALL).title("Graphemes"))
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .modifier(Modifier::BOLD),
            )
            .highlight_symbol(">");

        frame.render_widget(graphemes_list, rect);
    }

    fn draw_help_text(frame: &mut TerminalFrame, rect: Rect) {
        let help_item = [Text::raw("ESC to quit")];
        let help_text =
            Paragraph::new(help_item.iter()).style(Style::default().fg(Color::LightGreen));

        frame.render_widget(help_text, rect);
    }
}

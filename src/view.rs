use crate::application_state::ApplicationState;
use crate::renderer::ApplicationTerminal;

use crossterm::Result;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget},
};

use unic::{segment::Graphemes, ucd::name::Name};

pub struct View;

impl View {
    pub fn new() -> Self {
        View {}
    }

    pub fn update(
        &self,
        terminal: &mut ApplicationTerminal,
        state: &ApplicationState,
    ) -> Result<()> {
        terminal.draw(|mut frame| {
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
                .split(frame.size());

            Paragraph::new([Text::raw(&state.user_input)].iter())
                .block(Block::default().title("Input").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .render(&mut frame, chunks[0]);

            let graphemes = Graphemes::new(&state.user_input);
            let grapheme_strings = graphemes.fold(vec![], |mut sum, grapheme| {
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

                    // TODO: How to make `chr` aligned?
                    sum.push(format!("{}  {}  {}", code_point_str, chr, name));
                });
                sum.push("".to_owned());
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
                .render(&mut frame, chunks[1]);

            Paragraph::new([Text::raw(state.active_input_mode.to_string())].iter())
                .style(Style::default().fg(Color::Blue))
                .render(&mut frame, chunks[2]);
        })?;

        Ok(())
    }
}

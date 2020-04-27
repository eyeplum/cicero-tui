use tui::layout::{Alignment, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph, Text};
use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{Age, GeneralCategory, Name};

use crate::view::code_point_description;
use crate::view::main_view::TerminalFrame;

pub struct CharacterDetailView {
    pub chr: char,
}

impl CharacterDetailView {
    pub fn new(chr: char) -> Self {
        CharacterDetailView { chr }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let code_point_description = code_point_description(self.chr);

        let name_description = match Name::of(self.chr) {
            Some(name) => name.to_string(),
            None => "".to_owned(),
        };

        let age_description = match Age::of(self.chr) {
            Some(age) => format!("Unicode {}", age.actual().to_string()),
            None => "N/A".to_owned(),
        };

        let gc = GeneralCategory::of(self.chr);
        let gc_description = format!("{}({})", gc.human_name(), gc.abbr_name());

        let text = [
            Text::raw(format!("Code Point: {}\n", code_point_description)),
            Text::raw(format!("Name: {}\n", name_description)),
            Text::raw(format!("Age: {}\n", age_description)),
            Text::raw(format!("General Category: {}\n", gc_description)),
        ];
        let paragraph = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(&code_point_description),
            )
            .style(Style::default())
            .alignment(Alignment::Center)
            .wrap(true);

        frame.render_widget(paragraph, rect);
    }
}

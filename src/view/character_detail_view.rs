use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{Age, GeneralCategory, Name};

use crate::ucd::Plane;
use crate::view::character_preview_canvas::CharacterPreviewCanvas;
use crate::view::code_point_description;
use crate::view::main_view::TerminalFrame;

pub struct CharacterDetailView {
    pub chr: char,
    character_preview_canvas: CharacterPreviewCanvas,
}

impl CharacterDetailView {
    pub fn new(chr: char) -> Self {
        CharacterDetailView {
            chr,
            character_preview_canvas: CharacterPreviewCanvas::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(10)].as_ref())
            .direction(Direction::Vertical)
            .split(rect);

        self.character_preview_canvas
            .draw(frame, chunks[0], self.chr);

        self.draw_character_properties(frame, chunks[1]);
    }

    const NOT_AVAILABLE_DISPLAY_TEXT: &'static str = "N/A";

    fn draw_character_properties(&self, frame: &mut TerminalFrame, rect: Rect) {
        let code_point_description = code_point_description(self.chr);

        let name_description = match Name::of(self.chr) {
            Some(name) => name.to_string(),
            None => "".to_owned(),
        };

        let age_description = match Age::of(self.chr) {
            Some(age) => format!("Unicode {}", age.actual().to_string()),
            None => CharacterDetailView::NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
        };

        let gc = GeneralCategory::of(self.chr);
        let gc_description = format!("{}({})", gc.human_name(), gc.abbr_name());

        let plane_name = Plane::of(self.chr).name;

        let block_name = match unic::ucd::Block::of(self.chr) {
            Some(block) => block.name.to_owned(),
            None => CharacterDetailView::NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
        };

        let text = [
            Text::styled("General\n\n", Style::default().modifier(Modifier::BOLD)),
            Text::raw(format!("Code Point: {}\n", code_point_description)),
            Text::raw(format!("Name: {}\n", name_description)),
            Text::raw(format!("Age: {}\n", age_description)),
            Text::raw(format!("Plane: {}\n", plane_name)),
            Text::raw(format!("Block: {}\n", block_name)),
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

// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{Age, GeneralCategory, Name};

use super::character_preview_canvas::CharacterPreviewCanvas;
use super::main_view::TerminalFrame;
use crate::ucd::{code_point_description, Plane};

pub struct CharacterDetailView {
    pub chr: char,
    character_preview_canvas: CharacterPreviewCanvas,
}

impl CharacterDetailView {
    pub fn new(chr: char, preferred_preview_font_path: Option<&String>) -> Self {
        CharacterDetailView {
            chr,
            character_preview_canvas: CharacterPreviewCanvas::new(chr, preferred_preview_font_path),
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(10)].as_ref())
            .direction(Direction::Vertical)
            .split(rect);

        self.character_preview_canvas.draw(frame, chunks[0]);
        self.draw_character_properties(frame, chunks[1]);
    }

    pub fn get_current_preview_font_path(&self) -> Option<String> {
        self.character_preview_canvas.get_current_preview_font()
    }

    pub fn previous_preview_font(&mut self) {
        self.character_preview_canvas.previous_preview_font();
    }

    pub fn next_preview_font(&mut self) {
        self.character_preview_canvas.next_preview_font();
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

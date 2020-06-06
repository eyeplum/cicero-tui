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

use tui::layout::{Constraint, Direction, Layout, Rect};

use super::character_preview_canvas::CharacterPreviewCanvas;
use super::character_property_view::CharacterPropertyView;
use super::main_view::TerminalFrame;

pub struct CharacterDetailView {
    character_preview_canvas: CharacterPreviewCanvas,
    character_property_view: CharacterPropertyView,
}

impl CharacterDetailView {
    pub fn new(chr: char, preferred_preview_font_path: Option<&String>) -> Self {
        CharacterDetailView {
            character_preview_canvas: CharacterPreviewCanvas::new(chr, preferred_preview_font_path),
            character_property_view: CharacterPropertyView::new(chr),
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(10)].as_ref())
            .direction(Direction::Vertical)
            .split(rect);

        self.character_preview_canvas.draw(frame, chunks[0]);
        self.character_property_view.draw(frame, chunks[1]);
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

    pub fn scroll_down(&mut self) {
        self.character_property_view.scroll_down();
    }

    pub fn scroll_up(&mut self) {
        self.character_property_view.scroll_up();
    }
}

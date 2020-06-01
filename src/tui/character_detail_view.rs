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

use tui::layout::{Constraint, Direction, Layout, Margin, Rect};
use tui::widgets::{Block, Borders, Row, Table, TableState};

use super::character_preview_canvas::CharacterPreviewCanvas;
use super::main_view::TerminalFrame;
use crate::ucd::{code_point_description, CharacterProperties};

const NOT_AVAILABLE_DISPLAY_TEXT: &str = "N/A";

fn add_padding_to_column_data(string: &str, column_width: u16) -> String {
    if (column_width as usize) < string.len() {
        return string.to_owned();
    }
    format!(
        "{}{}",
        " ".repeat(column_width as usize - string.len()),
        string
    )
}

pub struct CharacterDetailView {
    pub chr: char,

    character_preview_canvas: CharacterPreviewCanvas,
    character_property_view_state: TableState,
}

impl CharacterDetailView {
    pub fn new(chr: char, preferred_preview_font_path: Option<&String>) -> Self {
        CharacterDetailView {
            chr,
            character_preview_canvas: CharacterPreviewCanvas::new(chr, preferred_preview_font_path),
            character_property_view_state: TableState::default(),
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

    fn draw_character_properties(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let code_point_description = code_point_description(self.chr);

        // Draw character property table
        {
            let character_properties = CharacterProperties::new(self.chr);
            let age_string = format!(
                "Unicode {}",
                character_properties
                    .age
                    .unwrap_or_else(|| NOT_AVAILABLE_DISPLAY_TEXT.to_owned())
            );
            let block_name_string = character_properties
                .block_name
                .unwrap_or_else(|| NOT_AVAILABLE_DISPLAY_TEXT.to_owned());

            let column_width = ((rect.width - 4) as f32 * 0.38).floor() as u16;
            let items = [
                Vec::<String>::default(),
                vec![
                    add_padding_to_column_data("Code Point", column_width),
                    code_point_description.clone(),
                ],
                vec![
                    add_padding_to_column_data("Name", column_width),
                    character_properties.name,
                ],
                vec![add_padding_to_column_data("Age", column_width), age_string],
                vec![
                    add_padding_to_column_data("Plane", column_width),
                    character_properties.plane_name,
                ],
                vec![
                    add_padding_to_column_data("Block", column_width),
                    block_name_string,
                ],
                vec![
                    add_padding_to_column_data("General Category", column_width),
                    character_properties.general_category,
                ],
            ];

            let header = Vec::<&str>::default();
            let rows = items.iter().map(|item| Row::Data(item.iter()));

            let table = Table::new(header.iter(), rows)
                .column_spacing(2)
                .widths(&[Constraint::Percentage(38), Constraint::Percentage(62)]);

            let mut table_rect = rect.inner(&Margin {
                horizontal: 2,
                vertical: 0,
            });
            table_rect.y -= 1;
            table_rect.height += 1;
            frame.render_stateful_widget(
                table,
                table_rect,
                &mut self.character_property_view_state,
            );
        }

        // Draw borders
        {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(&code_point_description);
            frame.render_widget(block, rect);
        }
    }
}

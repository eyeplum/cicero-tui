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

use tui::layout::{Constraint, Margin, Rect};
use tui::widgets::{Block, Borders, Row, Table, TableState};

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

pub struct CharacterPropertyView {
    character_properties: CharacterProperties,
    character_property_view_state: TableState,
}

impl CharacterPropertyView {
    pub fn new(chr: char) -> Self {
        CharacterPropertyView {
            character_properties: CharacterProperties::new(chr),
            character_property_view_state: TableState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let code_point_description = code_point_description(self.character_properties.character);

        // Draw character property table
        {
            let age_string = format!(
                "Unicode {}",
                self.character_properties
                    .age
                    .clone()
                    .unwrap_or_else(|| NOT_AVAILABLE_DISPLAY_TEXT.to_owned())
            );
            let block_name_string = self
                .character_properties
                .block_name
                .clone()
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
                    self.character_properties.name.clone(),
                ],
                vec![add_padding_to_column_data("Age", column_width), age_string],
                vec![
                    add_padding_to_column_data("Plane", column_width),
                    self.character_properties.plane_name.clone(),
                ],
                vec![
                    add_padding_to_column_data("Block", column_width),
                    block_name_string,
                ],
                vec![
                    add_padding_to_column_data("General Category", column_width),
                    self.character_properties.general_category.clone(),
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

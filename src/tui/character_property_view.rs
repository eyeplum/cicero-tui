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

use std::borrow::Cow;

use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListState, Text};

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

#[derive(Debug, Default)]
struct PropertyRow {
    title: &'static str,
    value: String,

    // The character this row links to, e.g. decomposition, or variants of Unihan, etc.
    link: Option<char>,
}

type NameAliases = Option<&'static [&'static str]>;
type CharacterComponents = Option<Vec<char>>;

impl PropertyRow {
    fn new(title: &'static str, value: String) -> Self {
        PropertyRow {
            title,
            value,
            link: None, // TODO: Link not implemented
        }
    }

    fn from_character_properties(character_properties: &CharacterProperties) -> Vec<Self> {
        let mut property_rows = vec![
            PropertyRow::default(),
            PropertyRow::new(
                "Code Point",
                code_point_description(character_properties.character),
            ),
            PropertyRow::new("Name", character_properties.name.clone()),
            PropertyRow::new(
                "Age",
                format!(
                    "Unicode {}",
                    character_properties
                        .age
                        .clone()
                        .unwrap_or_else(|| NOT_AVAILABLE_DISPLAY_TEXT.to_owned())
                ),
            ),
            PropertyRow::new("Plane", character_properties.plane_name.to_owned()),
            PropertyRow::new(
                "Block",
                character_properties
                    .block_name
                    .unwrap_or_else(|| NOT_AVAILABLE_DISPLAY_TEXT)
                    .to_owned(),
            ),
            PropertyRow::new(
                "General Category",
                character_properties.general_category.to_string(),
            ),
            PropertyRow::default(),
        ];

        property_rows.extend(PropertyRow::from_name_aliases(
            "Name Corrections",
            character_properties.name_corrections,
        ));
        property_rows.extend(PropertyRow::from_name_aliases(
            "Control Code Names",
            character_properties.control_code_names,
        ));
        property_rows.extend(PropertyRow::from_name_aliases(
            "Alternative Names",
            character_properties.alternative_names,
        ));
        property_rows.extend(PropertyRow::from_name_aliases(
            "Figments",
            character_properties.figments,
        ));
        property_rows.extend(PropertyRow::from_name_aliases(
            "Name Abbreviations",
            character_properties.name_abbreviations,
        ));

        property_rows.push(PropertyRow::default());

        property_rows.push(PropertyRow::new(
            "Cased",
            if character_properties.is_cased {
                "Yes".to_owned()
            } else {
                "No".to_owned()
            },
        ));
        property_rows.extend(PropertyRow::from_character_components(
            "Uppercase",
            &character_properties.uppercase,
        ));
        property_rows.extend(PropertyRow::from_character_components(
            "Lowercase",
            &character_properties.lowercase,
        ));

        property_rows.push(PropertyRow::default());

        property_rows
    }

    fn from_name_aliases(title: &'static str, name_aliases: NameAliases) -> Vec<Self> {
        let mut property_rows = vec![];
        match name_aliases {
            Some(aliases) => {
                for (index, name_alias) in aliases.iter().enumerate() {
                    property_rows.push(PropertyRow::new(
                        if index == 0 { title } else { "" },
                        (*name_alias).to_owned(),
                    ));
                }
            }
            None => property_rows.push(PropertyRow::new(
                title,
                NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
            )),
        }
        property_rows
    }

    fn from_character_components(
        title: &'static str,
        character_components: &CharacterComponents,
    ) -> Vec<Self> {
        let mut property_rows = vec![];
        match character_components {
            Some(components) => {
                for (index, component) in components.iter().enumerate() {
                    let component_description =
                        format!("{} {}", code_point_description(*component), *component);
                    property_rows.push(PropertyRow::new(
                        if index == 0 { title } else { "" },
                        component_description,
                    ));
                }
            }
            None => property_rows.push(PropertyRow::new(
                title,
                NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
            )),
        }
        property_rows
    }
}

pub struct CharacterPropertyView {
    character_properties: CharacterProperties,

    // The character properties are drawn in two Lists, one on the left hand side for the titles,
    // one on the right hand side for the values. Since they must be "scrolling" as if they were the
    // some List, they share the same ListState (and have identical number of rows).
    shared_list_state: ListState,
}

impl CharacterPropertyView {
    pub fn new(chr: char) -> Self {
        CharacterPropertyView {
            character_properties: CharacterProperties::new(chr),
            shared_list_state: ListState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        // Draw character property lists
        {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Percentage(32),
                        Constraint::Length(2),
                        Constraint::Min(10),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .horizontal_margin(1)
                .split(rect);

            let rows = PropertyRow::from_character_properties(&self.character_properties);

            let title_list = List::new(rows.iter().map(|row| {
                Text::Styled(
                    Cow::from(add_padding_to_column_data(row.title, chunks[0].width)),
                    Style::new().fg(Color::LightGreen),
                )
            }));
            frame.render_stateful_widget(title_list, chunks[0], &mut self.shared_list_state);

            let value_list = List::new(rows.iter().map(|row| Text::Raw(Cow::from(&row.value))));
            frame.render_stateful_widget(value_list, chunks[2], &mut self.shared_list_state);
        }

        // Draw borders
        {
            let code_point_description =
                code_point_description(self.character_properties.character);
            let block = Block::default()
                .borders(Borders::ALL)
                .title(&code_point_description);
            frame.render_widget(block, rect);
        }
    }
}

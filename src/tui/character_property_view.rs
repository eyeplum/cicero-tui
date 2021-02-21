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

use hex_slice::AsHex;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Text};

use super::main_view::TerminalFrame;
use crate::ucd::{code_point_to_string, CharacterProperties};

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
            PropertyRow::new(
                "Code Point",
                code_point_to_string(character_properties.character),
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
                    .unwrap_or(NOT_AVAILABLE_DISPLAY_TEXT)
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

        property_rows.push(PropertyRow::from_bool(
            "Is Cased",
            character_properties.is_cased,
        ));
        property_rows.extend(PropertyRow::from_optional_character_components(
            "Uppercase",
            &character_properties.uppercase.as_ref(),
        ));
        property_rows.extend(PropertyRow::from_optional_character_components(
            "Lowercase",
            &character_properties.lowercase.as_ref(),
        ));

        property_rows.push(PropertyRow::default());

        property_rows.push(PropertyRow::new(
            "Ccc",
            character_properties.ccc_description(),
        ));

        match &character_properties.decomposition {
            Some(decomposition) => {
                property_rows.push(PropertyRow::new(
                    "Decomposition Type",
                    decomposition.decomposition_type.to_string(),
                ));
                property_rows.extend(PropertyRow::from_character_components(
                    "Decompositions",
                    &decomposition.components,
                ));
            }
            None => {
                property_rows.push(PropertyRow::new(
                    "Decomposition Type",
                    NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
                ));
                property_rows.push(PropertyRow::new(
                    "Decompositions",
                    NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
                ));
            }
        }

        property_rows.push(PropertyRow::default());

        property_rows.push(PropertyRow::new(
            "Bidi Class",
            character_properties.bidi_class.to_string(),
        ));
        property_rows.push(PropertyRow::from_bool(
            "Is Bidi Control",
            character_properties.is_bidi_control,
        ));
        property_rows.push(PropertyRow::from_bool(
            "Is Bidi Mirroed",
            character_properties.is_bidi_mirrored,
        ));

        property_rows.push(PropertyRow::default());

        property_rows.push(PropertyRow::new(
            "Mandarin",
            character_properties
                .mandarin
                .unwrap_or(NOT_AVAILABLE_DISPLAY_TEXT)
                .to_owned(),
        ));
        property_rows.push(PropertyRow::from_optional_character(
            "Traditional Variant",
            character_properties.traditional_variant,
        ));
        property_rows.push(PropertyRow::from_optional_character(
            "Simplified Variant",
            character_properties.simplified_variant,
        ));

        property_rows.push(PropertyRow::default());

        property_rows.push(PropertyRow::new(
            "UTF-8",
            format!("{:#04x}", character_properties.utf8.as_hex()),
        ));
        property_rows.push(PropertyRow::new(
            "UTF-16",
            format!("{:#06x}", character_properties.utf16.as_hex()),
        ));

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

    fn from_character_components(title: &'static str, character_components: &[char]) -> Vec<Self> {
        let mut property_rows = vec![];
        for (index, component) in character_components.iter().enumerate() {
            property_rows.push(PropertyRow::from_character(
                if index == 0 { title } else { "" },
                *component,
            ));
        }
        property_rows
    }

    fn from_optional_character_components(
        title: &'static str,
        character_components: &Option<&Vec<char>>,
    ) -> Vec<Self> {
        match character_components {
            Some(components) => PropertyRow::from_character_components(title, components),
            None => vec![PropertyRow::new(
                title,
                NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
            )],
        }
    }

    fn from_character(title: &'static str, chr: char) -> Self {
        PropertyRow::new(title, format!("{} {}", code_point_to_string(chr), chr))
    }

    fn from_optional_character(title: &'static str, optional_chr: Option<char>) -> Self {
        match optional_chr {
            Some(chr) => PropertyRow::from_character(title, chr),
            None => PropertyRow::new(title, NOT_AVAILABLE_DISPLAY_TEXT.to_owned()),
        }
    }

    fn from_bool(title: &'static str, b: bool) -> Self {
        PropertyRow::new(title, if b { "Yes".to_owned() } else { "No".to_owned() })
    }

    fn is_default(&self) -> bool {
        self.title.is_empty() && self.value.is_empty() && self.link.is_none()
    }
}

pub struct CharacterPropertyView {
    character_properties: CharacterProperties,

    // The character properties are drawn in two Lists, one on the left hand side for the titles,
    // one on the right hand side for the values. Since they must be "scrolling" as if they were the
    // some List, they share the same ListState (and have identical number of rows).
    shared_list_state: ListState,
    rows: Vec<PropertyRow>,
}

impl CharacterPropertyView {
    pub fn new(chr: char) -> Self {
        CharacterPropertyView {
            character_properties: CharacterProperties::new(chr),
            shared_list_state: ListState::default(),
            rows: vec![],
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

            self.rows = PropertyRow::from_character_properties(&self.character_properties);

            let title_list = List::new(self.rows.iter().map(|row| {
                Text::Styled(
                    Cow::from(add_padding_to_column_data(row.title, chunks[0].width)),
                    Style::new().fg(Color::LightGreen),
                )
            }))
            .highlight_style(Style::default().fg(Color::LightGreen));
            frame.render_stateful_widget(title_list, chunks[0], &mut self.shared_list_state);

            let value_list =
                List::new(self.rows.iter().map(|row| Text::Raw(Cow::from(&row.value))))
                    .highlight_style(Style::default().modifier(Modifier::BOLD));
            frame.render_stateful_widget(value_list, chunks[2], &mut self.shared_list_state);
        }

        // Draw borders
        {
            let code_point_description = code_point_to_string(self.character_properties.character);
            let block = Block::default()
                .borders(Borders::ALL)
                .title(&code_point_description);
            frame.render_widget(block, rect);
        }
    }

    pub fn scroll_down(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let proposed_selection = match self.shared_list_state.selected() {
            Some(selected) => usize::min(selected + 1, self.rows.len() - 1),
            None => 0,
        };

        if self.rows[proposed_selection].is_default() {
            if proposed_selection + 1 < self.rows.len() - 1
                && !self.rows[proposed_selection + 1].is_default()
            {
                self.shared_list_state.select(Some(proposed_selection + 1));
            }
        } else {
            self.shared_list_state.select(Some(proposed_selection));
        }
    }

    pub fn scroll_up(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let proposed_selection = match self.shared_list_state.selected() {
            Some(selected) => {
                if selected > 0 {
                    selected - 1
                } else {
                    0
                }
            }
            None => self.rows.len() - 1,
        };

        if self.rows[proposed_selection].is_default() {
            if proposed_selection > 0 && !self.rows[proposed_selection - 1].is_default() {
                self.shared_list_state.select(Some(proposed_selection - 1));
            }
        } else {
            self.shared_list_state.select(Some(proposed_selection));
        }
    }
}

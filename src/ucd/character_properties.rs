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

use serde::Serialize;
use unic::char::property::EnumeratedCharProperty;
use unic::segment::Graphemes;
use unic::ucd::{is_cased, name_aliases_of, Age, Block, GeneralCategory, Name, NameAliasType};

use std::fmt;

use super::{code_point_description, Plane};

const TREE_GRAPH_EDGE: &str = "├── ";
const TREE_GRAPH_CORNER: &str = "└── ";

#[derive(Serialize, Debug)]
pub struct GraphemeProperties {
    grapheme: String,
    characters: Vec<CharacterProperties>,
}

impl GraphemeProperties {
    pub fn from_string(string: &str) -> Vec<GraphemeProperties> {
        Graphemes::new(string)
            .map(|grapheme| GraphemeProperties {
                grapheme: grapheme.to_owned(),
                characters: grapheme.chars().map(CharacterProperties::new).collect(),
            })
            .collect()
    }
}

impl fmt::Display for GraphemeProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.grapheme)?;
        for (index, character) in self.characters.iter().enumerate() {
            let tree_graph = if index + 1 == self.characters.len() {
                TREE_GRAPH_CORNER
            } else {
                TREE_GRAPH_EDGE
            };
            write!(f, "{}{}", tree_graph, character)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct CharacterProperties {
    pub character: char,

    pub code_point: u32,
    pub name: String,
    pub age: Option<String>,
    pub general_category: StringValuedProperty,
    pub plane_name: &'static str,
    pub block_name: Option<&'static str>,

    pub name_corrections: Option<&'static [&'static str]>,
    pub control_code_names: Option<&'static [&'static str]>,
    pub alternative_names: Option<&'static [&'static str]>,
    pub figments: Option<&'static [&'static str]>,
    pub name_abbreviations: Option<&'static [&'static str]>,

    pub is_cased: bool,
    pub uppercase: Option<Vec<char>>, // TODO: Implement uppercase in rust-unic
    pub lowercase: Option<Vec<char>>, // TODO: Implement lowercase in rust-unic
                                      // TODO: Implement title case in rust-unic
}

impl CharacterProperties {
    pub fn new(character: char) -> Self {
        CharacterProperties {
            character,

            code_point: character as u32,
            name: match Name::of(character) {
                Some(name) => name.to_string(),
                None => "".to_owned(),
            },
            age: match Age::of(character) {
                Some(age) => Some(age.actual().to_string()),
                None => None,
            },
            general_category: StringValuedProperty::new(GeneralCategory::of(character)),
            plane_name: Plane::of(character).name,
            block_name: match Block::of(character) {
                Some(block) => Some(block.name),
                None => None,
            },

            name_corrections: name_aliases_of(character, NameAliasType::NameCorrections),
            control_code_names: name_aliases_of(character, NameAliasType::ControlCodeNames),
            alternative_names: name_aliases_of(character, NameAliasType::AlternateNames),
            figments: name_aliases_of(character, NameAliasType::Figments),
            name_abbreviations: name_aliases_of(character, NameAliasType::NameAbbreviations),

            is_cased: is_cased(character),
            uppercase: if is_cased(character) {
                Some(character.to_uppercase().collect())
            } else {
                None
            },
            lowercase: if is_cased(character) {
                Some(character.to_lowercase().collect())
            } else {
                None
            },
        }
    }
}

impl fmt::Display for CharacterProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}  {}  {}",
            code_point_description(self.character),
            self.character,
            self.name,
        )?;
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct StringValuedProperty {
    pub abbr: &'static str,
    pub long: &'static str,
    pub human_readable: &'static str,
}

impl StringValuedProperty {
    pub fn new<P>(character_property: P) -> Self
    where
        P: EnumeratedCharProperty,
    {
        StringValuedProperty {
            abbr: character_property.abbr_name(),
            long: character_property.long_name(),
            human_readable: character_property.human_name(),
        }
    }
}

impl fmt::Display for StringValuedProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.human_readable, self.abbr)
    }
}

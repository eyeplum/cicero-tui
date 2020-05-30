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

use serde::{Deserialize, Serialize};
use unic::char::property::EnumeratedCharProperty;
use unic::segment::Graphemes;
use unic::ucd::{Age, Block, GeneralCategory, Name};

use crate::ucd::Plane;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterProperties {
    character: char,
    code_point: u32,
    name: String,
    age: Option<String>,
    general_category: String,
    plane_name: String,
    block_name: Option<String>,
}

impl CharacterProperties {
    pub fn new(character: char) -> Self {
        let name = match Name::of(character) {
            Some(name) => name.to_string(),
            None => "".to_owned(),
        };

        let age = match Age::of(character) {
            Some(age) => Some(age.actual().to_string()),
            None => None,
        };

        let gc = GeneralCategory::of(character);
        let general_category = format!("{}({})", gc.human_name(), gc.abbr_name());

        let plane_name = Plane::of(character).name.to_owned();

        let block_name = match Block::of(character) {
            Some(block) => Some(block.name.to_owned()),
            None => None,
        };

        CharacterProperties {
            character,
            code_point: character as u32,
            name,
            age,
            general_category,
            plane_name,
            block_name,
        }
    }
}

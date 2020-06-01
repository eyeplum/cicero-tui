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

use std::fmt;

use super::{code_point_description, Plane};

const TREE_GRAPH_EDGE: &str = "├── ";
const TREE_GRAPH_CORNER: &str = "└── ";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterProperties {
    pub character: char,

    pub code_point: u32,
    pub name: String,
    pub age: Option<String>,
    pub general_category: String,
    pub plane_name: String,
    pub block_name: Option<String>,
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

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
use unic::ucd::{
    bidi::{is_bidi_control, is_bidi_mirrored, BidiClass},
    is_cased, name_aliases_of,
    normal::{decompose_compatible, DecompositionType},
    Age, Block, CanonicalCombiningClass, GeneralCategory, Name, NameAliasType,
};

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
    // TODO: Implement titlecase in rust-unic
    pub uppercase: Option<Vec<char>>, // TODO: Implement uppercase in rust-unic
    pub lowercase: Option<Vec<char>>, // TODO: Implement lowercase in rust-unic

    pub ccc: u8,
    pub decomposition: Option<Decomposition>,

    pub bidi_class: StringValuedProperty,
    pub is_bidi_control: bool,
    pub is_bidi_mirrored: bool,
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

            ccc: CanonicalCombiningClass::of(character).number(),
            decomposition: Decomposition::new(character),

            bidi_class: StringValuedProperty::new(BidiClass::of(character)),
            is_bidi_control: is_bidi_control(character),
            is_bidi_mirrored: is_bidi_mirrored(character),
        }
    }

    pub fn ccc_description(&self) -> String {
        let long_description = match self.ccc {
            0 => Some("Not_Reordered".to_owned()),
            1 => Some("Overlay".to_owned()),
            6 => Some("Han_Reading".to_owned()),
            7 => Some("Nukta".to_owned()),
            8 => Some("Kana_Voicing".to_owned()),
            9 => Some("Virama".to_owned()),

            // Although not all numbers are valid in this range (e.g. CCC199 is invalid),
            // we are relying on rust-unic to ensure the ccc_num is always valid
            10..=199 => Some(format!("CCC{}", self.ccc)),

            200 => Some("Attached_Below_Left".to_owned()),
            202 => Some("Attached_Below".to_owned()),
            214 => Some("Attached_Above".to_owned()),
            216 => Some("Attached_Above_Right".to_owned()),
            218 => Some("Below_Left".to_owned()),
            220 => Some("Below".to_owned()),
            222 => Some("Below_Right".to_owned()),
            224 => Some("Left".to_owned()),
            226 => Some("Right".to_owned()),
            228 => Some("Above_Left".to_owned()),
            230 => Some("Above".to_owned()),
            232 => Some("Above_Right".to_owned()),
            233 => Some("Double_Below".to_owned()),
            234 => Some("Double_Above".to_owned()),
            240 => Some("Iota_Subscript".to_owned()),
            _ => None,
        };

        match long_description {
            Some(long_description) => format!("{}({})", long_description, self.ccc.to_string()),
            None => self.ccc.to_string(),
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

#[derive(Serialize, Debug)]
pub struct Decomposition {
    pub decomposition_type: StringValuedProperty,
    pub components: Vec<char>,
}

impl Decomposition {
    pub fn new(chr: char) -> Option<Self> {
        let decomposition_type = StringValuedProperty::new(DecompositionType::of(chr)?);

        let mut components = vec![];
        decompose_compatible(chr, |component| components.push(component));

        Some(Decomposition {
            decomposition_type,
            components,
        })
    }
}

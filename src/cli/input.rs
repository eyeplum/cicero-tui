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

use std::char;

use clap::ArgMatches;

pub const OPTION_NAME_INPUT_TYPE: &str = "input_type";
pub const OPTION_VALUE_INPUT_TYPE_STRING: &str = "string";
pub const OPTION_VALUE_INPUT_TYPE_CODE_POINTS: &str = "code-points";

pub const ARGUMENT_VALUE_NAME_INPUT: &str = "INPUT";

fn characters_from_input_string(input_string: &str) -> Vec<char> {
    input_string
        .split(',')
        .map(|component| {
            if !component.to_lowercase().starts_with("u+") || component[2..].is_empty() {
                return None;
            }
            match u32::from_str_radix(&component[2..], 16) {
                Ok(code_point) => char::from_u32(code_point),
                Err(_) => None,
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

#[derive(Debug)]
pub enum Input {
    String(String),
    Characters(Vec<char>),
}

impl ToString for Input {
    fn to_string(&self) -> String {
        match self {
            Input::String(string) => string.clone(),
            Input::Characters(characters) => {
                let mut string = String::new();
                string.reserve(characters.len());
                for chr in characters {
                    string.push(*chr);
                }
                string
            }
        }
    }
}

pub fn parse_input(args: &ArgMatches) -> Option<Input> {
    let input_string = args.value_of(ARGUMENT_VALUE_NAME_INPUT)?;
    match args.value_of(OPTION_NAME_INPUT_TYPE) {
        Some(input_type) => match input_type {
            OPTION_VALUE_INPUT_TYPE_CODE_POINTS => Some(Input::Characters(
                characters_from_input_string(input_string),
            )),
            _ => Some(Input::String(input_string.to_owned())),
        },
        None => Some(Input::String(input_string.to_owned())),
    }
}

// TODO: Unit tests

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

use super::{
    Error, Result, FLAG_NAME_CODE_POINT_INPUT_MODE, FLAG_NAME_GENERATE_FLAMEGRAPH,
    FLAG_NAME_TUI_MODE,
};
use crate::ucd::string_to_code_point;

pub const OPTION_NAME_INPUT_TYPE: &str = "input_type";
pub const OPTION_VALUE_INPUT_TYPE_STRING: &str = "string";
pub const OPTION_VALUE_INPUT_TYPE_CODE_POINTS: &str = "code-points";

pub const ARGUMENT_VALUE_NAME_INPUT: &str = "INPUT";

fn characters_from_input_string(input_string: &str) -> Vec<char> {
    input_string
        .split(',')
        .map(string_to_code_point)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

#[derive(Debug)]
pub enum Input {
    String(String),
    Characters(Vec<char>),
    GenerateFlamegraph,
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
            Input::GenerateFlamegraph => String::new(), // This variant is not representable as a string
        }
    }
}

pub fn parse_input(args: &ArgMatches) -> Result<Input> {
    if args.is_present(FLAG_NAME_GENERATE_FLAMEGRAPH) {
        return Ok(Input::GenerateFlamegraph);
    }

    let input_string = match args.value_of(ARGUMENT_VALUE_NAME_INPUT) {
        Some(v) => Ok(v),
        None => {
            if args.is_present(FLAG_NAME_TUI_MODE) {
                Ok("") // Fallback to an empty string
            } else {
                Err(Box::new(Error::MissingInput))
            }
        }
    }?;

    match args.value_of(OPTION_NAME_INPUT_TYPE) {
        Some(input_type) => match input_type {
            OPTION_VALUE_INPUT_TYPE_STRING => Ok(Input::String(input_string.to_owned())),
            OPTION_VALUE_INPUT_TYPE_CODE_POINTS => Ok(Input::Characters(
                characters_from_input_string(input_string),
            )),
            _ => Err(Box::new(Error::UnrecognizedInputType(
                input_type.to_owned(),
            ))),
        },
        None => {
            if args.is_present(FLAG_NAME_CODE_POINT_INPUT_MODE) {
                Ok(Input::Characters(characters_from_input_string(
                    input_string,
                )))
            } else {
                Ok(Input::String(input_string.to_owned()))
            }
        }
    }
}

// TODO: Unit tests

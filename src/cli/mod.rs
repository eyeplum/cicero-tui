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

//!
//! This this module implements the command line interface output.
//!

use clap::ArgMatches;

use std::error;
use std::fmt;

use crate::ucd::GraphemeProperties;

pub const FLAG_NAME_TUI_MODE: &str = "tui_mode";

pub const OPTION_NAME_OUTPUT_FORMAT: &str = "output_format";
pub const OPTION_VALUE_NAME_OUTPUT_FORMAT: &str = "FORMAT";

pub const OPTION_NAME_INPUT_TYPE: &str = "input_type";
pub const OPTION_VALUE_NAME_INPUT_TYPE: &str = "TYPE";

pub const ARGUMENT_VALUE_NAME_INPUT: &str = "INPUT";

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cicero Error")
    }
}

impl error::Error for Error {}

pub fn generate_output(args: ArgMatches) -> Result<String> {
    // TODO: Support code points input format
    // TODO: Support text output format
    // TODO: Support tree output format
    match args.value_of(ARGUMENT_VALUE_NAME_INPUT) {
        Some(input) => Ok(serde_json::to_string_pretty(
            &GraphemeProperties::from_string(input),
        )?),
        None => Ok(args.usage().to_owned()),
    }
}

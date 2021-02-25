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
//! This module implements the command line interface of Cicero.
//!

use std::error;
use std::fmt;

mod input;
mod output;

pub use input::{
    parse_input, ARGUMENT_VALUE_NAME_INPUT, OPTION_NAME_INPUT_TYPE,
    OPTION_VALUE_INPUT_TYPE_CODE_POINTS, OPTION_VALUE_INPUT_TYPE_STRING,
};
pub use output::{
    generate_output, OPTION_NAME_OUTPUT_FORMAT, OPTION_VALUE_OUTPUT_FORMAT_JSON,
    OPTION_VALUE_OUTPUT_FORMAT_TEXT,
};

pub const FLAG_NAME_TUI_MODE: &str = "tui_mode";
pub const FLAG_NAME_CODE_POINT_INPUT_MODE: &str = "code_point_input_mode";

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum Error {
    UnrecognizedInputType(String),
    UnrecognizedOutputFormat(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnrecognizedInputType(input_type) => {
                write!(f, "Unrecognized input type '{}'", input_type)
            }
            Error::UnrecognizedOutputFormat(output_format) => {
                write!(f, "Unrecognized output format '{}'", output_format)
            }
        }
    }
}

impl error::Error for Error {}

#[macro_export]
macro_rules! create_cli {
    () => {
        App::new("Cicero: A Unicode Tool")
            .version(&*format!(
                "{} (Unicode Version {})",
                crate_version!(),
                UNICODE_VERSION
            ))
            .arg(
                Arg::with_name(cli::FLAG_NAME_TUI_MODE)
                    .short("t")
                    .long("tui")
                    .help("Shows Terminal UI"),
            )
            .arg(
                Arg::with_name(cli::FLAG_NAME_CODE_POINT_INPUT_MODE)
                    .short("u")
                    .help(&format!(
                        "Parses {} as comma separated code points,\n\
                     same as '--input-type={}',\n\
                     ignored if '--input-type' is specified",
                        cli::ARGUMENT_VALUE_NAME_INPUT,
                        cli::OPTION_VALUE_INPUT_TYPE_CODE_POINTS,
                    )),
            )
            .arg(
                Arg::with_name(cli::OPTION_NAME_OUTPUT_FORMAT)
                    .short("o")
                    .long("output-format")
                    .takes_value(true)
                    .value_name("FORMAT")
                    .help(&format!(
                        "Specifies output format, '{}' by default,\n\
                     valid values: {}, {}",
                        cli::OPTION_VALUE_OUTPUT_FORMAT_TEXT,
                        cli::OPTION_VALUE_OUTPUT_FORMAT_TEXT,
                        cli::OPTION_VALUE_OUTPUT_FORMAT_JSON,
                    )),
            )
            .arg(
                Arg::with_name(cli::OPTION_NAME_INPUT_TYPE)
                    .short("i")
                    .long("input-type")
                    .takes_value(true)
                    .value_name("TYPE")
                    .help(&format!(
                        "Specifies input type, '{}' by default,\n\
                     valid values: {}, {}",
                        cli::OPTION_VALUE_INPUT_TYPE_STRING,
                        cli::OPTION_VALUE_INPUT_TYPE_STRING,
                        cli::OPTION_VALUE_INPUT_TYPE_CODE_POINTS,
                    )),
            )
            .arg(
                Arg::with_name(cli::ARGUMENT_VALUE_NAME_INPUT)
                    .help("a string or comma separated code points"),
            )
    };
}

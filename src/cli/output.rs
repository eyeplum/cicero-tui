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

use clap::ArgMatches;

use super::{parse_input, Error, Result};
use crate::ucd::GraphemeProperties;

pub const OPTION_NAME_OUTPUT_FORMAT: &str = "output_format";
pub const OPTION_VALUE_OUTPUT_FORMAT_TEXT: &str = "text";
pub const OPTION_VALUE_OUTPUT_FORMAT_JSON: &str = "json";

fn graphems_to_string(graphemes: &[GraphemeProperties]) -> String {
    graphemes
        .iter()
        .map(GraphemeProperties::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate_output(args: ArgMatches) -> Result<String> {
    let input = parse_input(&args)?;
    let graphemes = GraphemeProperties::from_string(&input.to_string());
    match args.value_of(OPTION_NAME_OUTPUT_FORMAT) {
        Some(output_format) => match output_format {
            OPTION_VALUE_OUTPUT_FORMAT_TEXT => Ok(graphems_to_string(&graphemes)),
            OPTION_VALUE_OUTPUT_FORMAT_JSON => Ok(serde_json::to_string_pretty(&graphemes)?),
            _ => Err(Box::new(Error::UnrecognizedOutputFormat(
                output_format.to_owned(),
            ))),
        },
        None => Ok(graphems_to_string(&graphemes)),
    }
}

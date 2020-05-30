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

use super::{parse_input, Result};
use crate::ucd::GraphemeProperties;

pub const OPTION_NAME_OUTPUT_FORMAT: &str = "output_format";

pub fn generate_output(args: ArgMatches) -> Result<String> {
    match parse_input(&args) {
        Some(input) => {
            // TODO: Support text output format
            // TODO: Support tree output format
            Ok(serde_json::to_string_pretty(
                &GraphemeProperties::from_string(&input.to_string()),
            )?)
        }
        None => Ok(args.usage().to_owned()),
    }
}

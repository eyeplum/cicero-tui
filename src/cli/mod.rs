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

pub const FLAG_NAME_TUI_MODE: &str = "tui_mode";

pub const OPTION_NAME_OUTPUT_FORMAT: &str = "output_format";
pub const OPTION_VALUE_OUTPUT_FORMAT: &str = "FORMAT";

pub const OPTION_NAME_INPUT_TYPE: &str = "input_type";
pub const OPTION_VALUE_INPUT_TYPE: &str = "TYPE";

pub const ARGUMENT_VALUE_INPUT: &str = "INPUT";

pub fn generate_output(args: ArgMatches) -> String {
    // TODO: Generate CLI output based on args
    args.usage().to_owned()
}

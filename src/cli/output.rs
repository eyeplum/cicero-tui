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
use crate::cli::input::Input;
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

    if let Input::GenerateFlamegraph = input {
        return chrome_tracing::describe_unicode_as_events();
    }

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

// TODO: Should this module be a separate file?
mod chrome_tracing {
    use serde::Serialize;
    use unic::ucd::{Block, BlockIter};

    use super::Result;
    use crate::ucd::{Plane, PLANE_COUNT};

    #[derive(Serialize)]
    struct Event {
        name: &'static str,
        cat: &'static str,
        ph: char,
        ts: u32,
        pid: u64,
        tid: u64,
    }

    pub fn describe_unicode_as_events() -> Result<String> {
        let blocks: Vec<Block> = BlockIter::new().collect();

        let mut events = Vec::with_capacity(blocks.len() + PLANE_COUNT as usize);

        for i in 0..PLANE_COUNT {
            let plane = Plane::at(i as usize);
            events.push(Event {
                name: plane.name,
                cat: "Plane",
                ph: 'B',
                ts: plane.range.start,
                pid: 0,
                tid: 0,
            });
            events.push(Event {
                name: plane.name,
                cat: "Plane",
                ph: 'E',
                ts: plane.range.end,
                pid: 0,
                tid: 0,
            });
        }

        for block in blocks {
            events.push(Event {
                name: block.name,
                cat: "Block",
                ph: 'B',
                ts: block.range.low as u32,
                pid: 0,
                tid: 0,
            });
            events.push(Event {
                name: block.name,
                cat: "Block",
                ph: 'E',
                ts: block.range.high as u32,
                pid: 0,
                tid: 0,
            });
        }

        Ok(serde_json::to_string_pretty(&events)?)
    }
}

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

#![warn(clippy::all)]

#[macro_use(defer)]
extern crate scopeguard;

use clap::{App, Arg, ArgMatches};
use crossterm::Result;

mod preview;
mod tui;
mod ucd;

fn run_tui() -> Result<()> {
    let mut state = tui::ApplicationState::default();

    let renderer = tui::Renderer::new();
    let mut main_view = tui::MainView::new();

    renderer.run(|terminal| {
        main_view.update(terminal, &mut state)?;
        Ok(state.keep_running)
    })
}

fn run_cli(args: ArgMatches) -> Result<()> {
    println!("!!ERROR!! Cicero CLI is not implemented yet");
    println!("{}", args.usage());
    Ok(())
}

fn main() -> Result<()> {
    let args = App::new("Cicero: A Unicode Tool")
        .version("0.1.0")
        .arg(
            Arg::with_name("tui_mode")
                .short("t")
                .long("tui")
                .help("Show Terminal UI"),
        )
        .arg(
            Arg::with_name("output_format")
                .short("o")
                .long("output-format")
                .takes_value(true)
                .value_name("FORMAT")
                .help(
                    "Specify output format, text if not provided\n\
                     Valid values: text, json",
                ),
        )
        .arg(
            Arg::with_name("input_type")
                .short("i")
                .long("input-type")
                .takes_value(true)
                .value_name("TYPE")
                .help(
                    "Specify input type, if not provided a best guess is performed on the input\n\
                     Valid values: code-points, string",
                ),
        )
        .arg(Arg::with_name("INPUT").help("a string or comma separated code points"))
        .get_matches();

    if args.is_present("tui_mode") {
        run_tui()
    } else {
        run_cli(args)
    }
}

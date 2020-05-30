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

mod cli;
mod preview;
mod tui;
mod ucd;

use cli::Result;

fn run_tui() -> Result<()> {
    let mut state = tui::ApplicationState::default();

    let renderer = tui::Renderer::new();
    let mut main_view = tui::MainView::new();

    match renderer.run(|terminal| {
        main_view.update(terminal, &mut state)?;
        Ok(state.keep_running)
    }) {
        Ok(()) => Ok(()),
        Err(error) => Err(Box::new(error)),
    }
}

fn run_cli(args: ArgMatches) -> Result<()> {
    println!("{}", cli::generate_output(args)?);
    Ok(())
}

fn main() -> Result<()> {
    let args = App::new("Cicero: A Unicode Tool")
        .version("0.1.0")
        .arg(
            Arg::with_name(cli::FLAG_NAME_TUI_MODE)
                .short("t")
                .long("tui")
                .help("Show Terminal UI"),
        )
        .arg(
            Arg::with_name(cli::OPTION_NAME_OUTPUT_FORMAT)
                .short("o")
                .long("output-format")
                .takes_value(true)
                .value_name("FORMAT")
                .help(
                    "Specify output format, 'text' by default\n\
                     valid values: text, json, tree",
                ),
        )
        .arg(
            Arg::with_name(cli::OPTION_NAME_INPUT_TYPE)
                .short("i")
                .long("input-type")
                .takes_value(true)
                .value_name("TYPE")
                .help(&format!(
                    "Specify input type, '{}' by default\n\
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
        .get_matches();

    if args.is_present(cli::FLAG_NAME_TUI_MODE) {
        // TODO: Pass input to the tui
        run_tui()
    } else {
        run_cli(args)
    }
}

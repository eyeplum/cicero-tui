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

use clap::{crate_version, App, Arg, ArgMatches};
use unic::ucd::version::UNICODE_VERSION;

mod cli;
mod preview;
mod settings;
mod tui;
mod ucd;

use cli::Result;

fn run_tui(args: ArgMatches) -> Result<()> {
    let mut state = tui::ApplicationState::default();

    let user_input = cli::parse_input(&args)?;
    let mut main_view = tui::MainView::new(user_input.to_string());

    let renderer = tui::Renderer::new();
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
    let args = create_cli!().get_matches();
    if args.is_present(cli::FLAG_NAME_TUI_MODE) {
        run_tui(args)
    } else {
        run_cli(args)
    }
}

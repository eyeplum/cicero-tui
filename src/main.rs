#![warn(clippy::all)]

#[macro_use(defer)]
extern crate scopeguard;

use clap::{App, Arg, ArgMatches};
use crossterm::Result;

mod application_state;
mod preview;
mod renderer;
mod ucd;
mod view;

use application_state::ApplicationState;
use renderer::Renderer;
use view::MainView;

fn run_tui() -> Result<()> {
    let mut state = ApplicationState::default();

    let renderer = Renderer::new();
    let mut main_view = MainView::new();

    renderer.run(|terminal, keep_running| {
        main_view.update(terminal, &mut state)?;
        *keep_running = state.keep_running;

        Ok(())
    })
}

fn run_cli(args: ArgMatches) -> Result<()> {
    println!("!!ERROR!! Cicero CLI is not implemented");
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
                     valid values: text, json",
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
                     Valid values: codepoints, string, file",
                ),
        )
        .arg(Arg::with_name("INPUT").help("a string, a file, or comma separated codepoints"))
        .get_matches();

    if args.is_present("tui_mode") {
        run_tui()
    } else {
        run_cli(args)
    }
}

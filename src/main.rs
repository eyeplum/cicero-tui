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
                     Valid values: codepoints, string",
                ),
        )
        .arg(Arg::with_name("INPUT").help("a string or comma separated codepoints"))
        .get_matches();

    if args.is_present("tui_mode") {
        run_tui()
    } else {
        run_cli(args)
    }
}

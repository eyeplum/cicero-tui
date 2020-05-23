#![warn(clippy::all)]

#[macro_use(defer)]
extern crate scopeguard;

use crossterm::Result;

mod application_state;
use application_state::ApplicationState;

mod preview;

mod renderer;
use renderer::Renderer;

mod ucd;

mod view;
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

fn run_cli(ch: String) -> Result<()> {
    println!("Character info for '{}': TODO", ch);
    Ok(())
}

fn main() -> Result<()> {
    // TODO: Implement argument parsing correctly
    let mut args = std::env::args();
    if args.len() >= 2 {
        run_cli(args.nth(1).unwrap())
    } else {
        run_tui()
    }
}

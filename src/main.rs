#![warn(clippy::all)]

use crossterm::Result;

mod application_state;
use application_state::ApplicationState;

mod preview;
use preview::CharacterPreview;

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

fn run_cli(font_path: String, ch: char) -> Result<()> {
    println!(
        "Character preview for '{}' with font at path: '{}':",
        ch, font_path
    );

    let preview = CharacterPreview::new(&font_path).unwrap();
    let bitmap = preview.preview_for(ch).unwrap();

    for row in bitmap.iter() {
        for p in row.iter() {
            print!(
                "{}",
                match *p {
                    p if p == 0 => " ",
                    p if p < 128 => "+",
                    _ => "*",
                }
            );
        }
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    // TODO: Implement argument parsing correctly
    let mut args = std::env::args();
    if args.len() >= 3 {
        run_cli(
            args.nth(1).unwrap(),
            args.nth(0).unwrap().chars().nth(0).unwrap(),
        )
    } else {
        run_tui()
    }
}

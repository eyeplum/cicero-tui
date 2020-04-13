#![warn(clippy::all)]

use crossterm::Result;

mod application_state;
use application_state::ApplicationState;

mod input;
use input::{InputHandler, InputMode};

mod renderer;
use renderer::Renderer;

mod view;
use view::View;

fn main() -> Result<()> {
    let mut state = ApplicationState::default();

    let renderer = Renderer::new();
    let view = View::new();
    let input_handler = InputHandler::new();

    renderer.run(|terminal, keep_running| {
        view.update(terminal, &state)?;

        input_handler.handle_event(&mut state)?;
        *keep_running = state.keep_running;

        Ok(())
    })
}

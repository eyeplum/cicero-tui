#![warn(clippy::all)]

use crossterm::Result;

mod application_state;
use application_state::ApplicationState;

mod renderer;
use renderer::Renderer;

mod view;
use view::MainView;

fn main() -> Result<()> {
    let mut state = ApplicationState::default();

    let renderer = Renderer::new();
    let mut main_view = MainView::new();

    renderer.run(|terminal, keep_running| {
        main_view.update(terminal, &mut state)?;
        *keep_running = state.keep_running;

        Ok(())
    })
}

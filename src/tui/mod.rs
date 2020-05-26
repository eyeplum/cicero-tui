//!
//! This module implements the Terminal User Interface of Cicero
//!

mod character_detail_view;
mod character_preview_canvas;
mod main_view;
mod renderer;
mod stateful_graphemes;

pub use main_view::MainView;
pub use renderer::Renderer;

pub struct ApplicationState {
    pub keep_running: bool,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState { keep_running: true }
    }
}

fn code_point_description(chr: char) -> String {
    format!("U+{:04X}", chr as u32)
}

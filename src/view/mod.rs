mod character_detail_view;
mod character_preview_canvas;
mod main_view;
mod stateful_graphemes;

pub use main_view::MainView;

fn code_point_description(chr: char) -> String {
    format!("U+{:04X}", chr as u32)
}

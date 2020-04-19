mod stateful_graphemes;
mod view;

pub use view::View;

fn code_point_description(chr: char) -> String {
    format!("U+{:04X}", chr as u32)
}

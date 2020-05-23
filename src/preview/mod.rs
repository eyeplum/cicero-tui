mod character_preview;
mod font_match;

pub use character_preview::{CharacterPreview, RenderSize};

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    GlyphNotFound { chr: char },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GlyphNotFound { chr } => write!(
                f,
                "Glyph for U+{:04X} not found in all system fonts",
                *chr as u32
            ),
        }
    }
}

impl error::Error for Error {}

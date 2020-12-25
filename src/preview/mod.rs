// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

//!
//! This module implements character preview, which consists of two main functionalities:
//! - Font matching, which finds the available fonts in the system and matches them with a given
//!   character.
//! - Glyph shaping, which turns a given character into a rendered bitmap using a font.
//!

mod character_preview;
mod font_match;
mod stateful_vec;

pub use character_preview::{CharacterPreview, RenderSize, RenderedCharacter};

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    PreviewNotSupported, // Only used on Windows at the moment
    GlyphNotFound { chr: char },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PreviewNotSupported => {
                write!(f, "Character preview is not supported on this system")
            }
            Error::GlyphNotFound { chr } => write!(
                f,
                "Failed to find glyph for U+{:04X} in any fonts on this system",
                *chr as u32
            ),
        }
    }
}

impl error::Error for Error {}

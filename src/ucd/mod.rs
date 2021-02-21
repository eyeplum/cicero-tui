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
//! This module supplements `unic::ucd` by providing missing APIs or wrapper APIs for accessing the
//! Unicode Character Database.
//!
//! Some APIs in this module works with Unicode code points (as `u32`) instead of the built-in
//! `char` because those APIs are intended to also work with isolated surrogate code points -
//! Cicero is a Unicode tool after all, so sometimes it needs to work with all aspects of Unicode.
//!

mod character_properties;
mod plane;

pub use character_properties::{CharacterProperties, GraphemeProperties};
pub use plane::Plane;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Range {
    start: u32,
    end: u32,
}

pub fn code_point_to_string(chr: char) -> String {
    format!("U+{:04X}", chr as u32)
}

pub fn string_to_code_point(input: &str) -> Option<char> {
    if !input.to_lowercase().starts_with("u+") || input[2..].is_empty() {
        return None;
    }

    use std::char;
    match u32::from_str_radix(&input[2..], 16) {
        Ok(code_point) => char::from_u32(code_point),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_point_to_string() {
        assert_eq!("U+0020", code_point_to_string('\u{0020}'));
        assert_eq!("U+34FF", code_point_to_string('\u{34FF}'));
        assert_eq!("U+10FFFF", code_point_to_string('\u{10FFFF}'));
    }

    #[test]
    fn test_string_to_code_point() {
        assert_eq!(Some('\u{0020}'), string_to_code_point("U+20"));
        assert_eq!(Some('\u{0020}'), string_to_code_point("U+0020"));
        assert_eq!(Some('\u{34FF}'), string_to_code_point("U+34FF"));
        assert_eq!(Some('\u{10FFFF}'), string_to_code_point("U+10FFFF"));

        assert_eq!(Some('\u{0020}'), string_to_code_point("u+20"));
        assert_eq!(Some('\u{0020}'), string_to_code_point("u+0020"));
        assert_eq!(Some('\u{34ff}'), string_to_code_point("u+34ff"));
        assert_eq!(Some('\u{10ffff}'), string_to_code_point("u+10ffff"));

        assert_eq!(None, string_to_code_point("Invalid"));
        assert_eq!(None, string_to_code_point("U+11FFFF"));
    }
}

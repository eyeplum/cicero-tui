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

use serde::{Serialize, Serializer};

use std::path::PathBuf;

use crate::ucd::code_point_description;

#[derive(Serialize)]
pub struct Settings {
    use_fontconfig: bool,
    font_search_paths: Vec<PathBuf>,
    preview_fonts: Vec<PreviewFontSetting>,
}

#[derive(Serialize)]
pub struct PreviewFontSetting {
    code_point_range: Option<CodePointRange>,
    font_name: String,
}

enum CodePointRange {
    Raw { first: char, last: char },
    Plane { name: String },
    Block { name: String },
}

impl Serialize for CodePointRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CodePointRange::Raw { first, last } => serializer.serialize_str(&format!(
                "{}..{}",
                code_point_description(*first),
                code_point_description(*last)
            )),
            CodePointRange::Plane { name } => serializer.serialize_str(name),
            CodePointRange::Block { name } => serializer.serialize_str(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_settings() -> Settings {
        Settings {
            use_fontconfig: true,
            font_search_paths: vec![
                PathBuf::from("/test/path/fonts"),
                PathBuf::from("/test/path/fonts2"),
            ],
            preview_fonts: vec![
                PreviewFontSetting {
                    code_point_range: None,
                    font_name: "TestFontName-Regular".to_owned(),
                },
                PreviewFontSetting {
                    code_point_range: Some(CodePointRange::Raw {
                        first: '\u{0020}',
                        last: '\u{00FF}',
                    }),
                    font_name: "TestFontName-Regular".to_owned(),
                },
                PreviewFontSetting {
                    code_point_range: Some(CodePointRange::Block {
                        name: "Basic Latin".to_owned(),
                    }),
                    font_name: "TestFontName-Regular".to_owned(),
                },
                PreviewFontSetting {
                    code_point_range: Some(CodePointRange::Plane {
                        name: "Basic Multilingual Plane".to_owned(),
                    }),
                    font_name: "TestFontName-Regular".to_owned(),
                },
            ],
        }
    }

    const TEST_SETTINGS_TOML_STRING: &str = include_str!("test_resources/test_settings.toml");

    #[test]
    fn test_serialization() {
        let serialized_toml_string = toml::to_string(&get_test_settings()).unwrap();
        assert_eq!(serialized_toml_string, TEST_SETTINGS_TOML_STRING);
    }

    #[test]
    fn test_deserialization() {
        // TODO: Not implemented
    }
}

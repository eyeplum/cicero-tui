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

use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use unic::ucd::BlockIter;

use crate::ucd::{code_point_to_string, string_to_code_point, Plane};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    use_fontconfig: bool,
    font_search_paths: Vec<PathBuf>,
    preview_fonts: Vec<PreviewFontSetting>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreviewFontSetting {
    code_point_range: Option<CodePointRange>,
    font_name: String,
}

const CODE_POINT_RANGE_SEPARATOR: &str = "..";

#[derive(Debug, Eq, PartialEq)]
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
                "{}{}{}",
                code_point_to_string(*first),
                CODE_POINT_RANGE_SEPARATOR,
                code_point_to_string(*last)
            )),
            CodePointRange::Plane { name } => serializer.serialize_str(name),
            CodePointRange::Block { name } => serializer.serialize_str(name),
        }
    }
}

impl<'de> Deserialize<'de> for CodePointRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CodePointRangeVisitor {
            plane_names: BTreeSet<&'static str>,
            block_names: BTreeSet<&'static str>,
        }

        impl CodePointRangeVisitor {
            fn new() -> Self {
                let plane_names = {
                    let mut set = BTreeSet::new();
                    for name in Plane::all_plane_names() {
                        set.insert(*name);
                    }
                    set
                };

                let block_names = {
                    let block_iter = BlockIter::new();
                    let mut set = BTreeSet::new();
                    for name in block_iter {
                        set.insert(name.name);
                    }
                    set
                };

                CodePointRangeVisitor {
                    plane_names,
                    block_names,
                }
            }
        }

        impl<'de> Visitor<'de> for CodePointRangeVisitor {
            type Value = CodePointRange;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "A string representing either an inclusive range, a plane name, or a block name")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut range_components = v.split(CODE_POINT_RANGE_SEPARATOR);
                let first_str = range_components.next();
                let last_str = range_components.next();
                if first_str.is_some() && last_str.is_some() {
                    let first =
                        string_to_code_point(first_str.unwrap()).ok_or(E::custom(format!(
                            "Invalid first code point for range: '{}'",
                            first_str.unwrap()
                        )))?;
                    let last = string_to_code_point(last_str.unwrap()).ok_or(E::custom(
                        format!("Invalid last code point for range: '{}'", last_str.unwrap()),
                    ))?;
                    return Ok(CodePointRange::Raw { first, last });
                }

                if self.plane_names.contains(v) {
                    return Ok(CodePointRange::Plane { name: v.to_owned() });
                }

                if self.block_names.contains(v) {
                    return Ok(CodePointRange::Block { name: v.to_owned() });
                }

                Err(E::custom(format!("Unrecognized code point range: '{}'", v)))
            }
        }

        deserializer.deserialize_str(CodePointRangeVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SETTINGS_TOML_STRING: &str = include_str!("test_resources/test_settings.toml");

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

    fn sanitize_line_breaks(input: &str) -> String {
        input.replace("\r\n", "\n")
    }

    #[test]
    fn test_serialization() {
        let serialized_toml_string = toml::to_string(&get_test_settings()).unwrap();
        assert_eq!(
            sanitize_line_breaks(&serialized_toml_string),
            sanitize_line_breaks(TEST_SETTINGS_TOML_STRING)
        );
    }

    #[test]
    fn test_deserialization() {
        let deserialized_settings: Settings =
            toml::from_str(&sanitize_line_breaks(TEST_SETTINGS_TOML_STRING)).unwrap();
        assert_eq!(deserialized_settings, get_test_settings());
    }
}

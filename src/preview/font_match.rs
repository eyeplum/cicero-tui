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

use std::path::PathBuf;

use super::{Error, Result};
use crate::settings::Settings;

#[non_exhaustive]
#[derive(Clone)]
pub struct FontDescriptor {
    pub path: PathBuf,
    pub family_name: String,
    pub full_name: String,
}

pub fn fonts_for(chr: char, settings: &Settings) -> Result<Vec<FontDescriptor>> {
    #[cfg(target_family = "unix")]
    if settings.uses_fontconfig() {
        with_fontconfig::match_fonts_for_character(chr, settings)
    } else {
        no_fontconfig::match_fonts_for_character(chr, settings)
    }

    #[cfg(target_family = "windows")]
    no_fontconfig::match_fonts_for_character(chr, settings)
}

#[cfg(target_family = "unix")]
mod with_fontconfig {
    use std::ffi;
    use std::os::raw::c_char;
    use std::path::PathBuf;
    use std::slice;

    use fontconfig::fontconfig as fc;
    use scopeguard::defer;

    use super::{filter_fonts_with_preview_font_settings, Error, FontDescriptor, Result, Settings};

    const FC_PROPERTY_FILE: &str = "file";
    const FC_PROPERTY_FAMILY_NAME: &str = "family";
    const FC_PROPERTY_FULL_NAME: &str = "fullname";

    pub fn match_fonts_for_character(
        chr: char,
        settings: &Settings,
    ) -> Result<Vec<FontDescriptor>> {
        let all_fonts = all_fonts_matched_by_fontconfig(chr)?;
        let specified_preview_font_names = settings.get_preview_fonts_for(chr);
        Ok(filter_fonts_with_preview_font_settings(
            all_fonts,
            &specified_preview_font_names,
        ))
    }

    fn all_fonts_matched_by_fontconfig(chr: char) -> Result<Vec<FontDescriptor>> {
        unsafe {
            let char_set = fc::FcCharSetCreate();
            defer! {
                fc::FcCharSetDestroy(char_set);
            }
            fc::FcCharSetAddChar(char_set, chr as u32);

            let pattern = fc::FcPatternCreate();
            defer! {
                fc::FcPatternDestroy(pattern);
            }
            fc::FcPatternAddCharSet(pattern, ffi::CString::new("charset")?.as_ptr(), char_set);

            let object_set = fc::FcObjectSetCreate();
            defer! {
                fc::FcObjectSetDestroy(object_set);
            }
            fc::FcObjectSetAdd(object_set, ffi::CString::new(FC_PROPERTY_FILE)?.as_ptr());
            fc::FcObjectSetAdd(
                object_set,
                ffi::CString::new(FC_PROPERTY_FAMILY_NAME)?.as_ptr(),
            );
            fc::FcObjectSetAdd(
                object_set,
                ffi::CString::new(FC_PROPERTY_FULL_NAME)?.as_ptr(),
            );

            let font_set = fc::FcFontList(std::ptr::null_mut(), pattern, object_set);
            defer! {
                fc::FcFontSetDestroy(font_set);
            }

            if (*font_set).nfont <= 0 {
                return Err(Box::new(Error::GlyphNotFound { chr }));
            }

            let patterns_slice = slice::from_raw_parts::<*mut fc::FcPattern>(
                (*font_set).fonts,
                (*font_set).nfont as usize,
            );
            let font_descriptors = patterns_slice
                .iter()
                .filter_map(|pattern| try_create_font_descriptor_from_fc_pattern(*pattern))
                .collect();

            Ok(font_descriptors)
        }
    }

    fn fc_pattern_get_string_property(
        pattern: *mut fc::FcPattern,
        property_name: &str,
    ) -> Option<String> {
        let mut value: *mut u8 = std::ptr::null_mut();

        let property_name = ffi::CString::new(property_name);
        if property_name.is_err() {
            return None;
        }
        let property_name = property_name.unwrap();

        unsafe {
            let result = fc::FcPatternGetString(
                pattern,
                property_name.as_ptr(),
                0,
                &mut value as *mut *mut u8,
            );
            if result != fc::FcResultMatch {
                return None;
            }

            let property_value_str = ffi::CStr::from_ptr(value as *mut c_char).to_str();
            if property_value_str.is_err() {
                return None;
            }
            let property_value_str = property_value_str.unwrap();

            Some(property_value_str.to_owned())
        }
    }

    fn try_create_font_descriptor_from_fc_pattern(
        pattern: *mut fc::FcPattern,
    ) -> Option<FontDescriptor> {
        let path = fc_pattern_get_string_property(pattern, FC_PROPERTY_FILE)?;
        let family_name = fc_pattern_get_string_property(pattern, FC_PROPERTY_FAMILY_NAME)?;
        let full_name = fc_pattern_get_string_property(pattern, FC_PROPERTY_FULL_NAME)?;
        Some(FontDescriptor {
            path: PathBuf::from(path),
            family_name,
            full_name,
        })
    }
}

mod no_fontconfig {
    use std::path::PathBuf;

    use freetype::{Face, Library};
    use walkdir::WalkDir;

    use super::{filter_fonts_with_preview_font_settings, Error, FontDescriptor, Result, Settings};

    pub fn match_fonts_for_character(
        chr: char,
        settings: &Settings,
    ) -> Result<Vec<FontDescriptor>> {
        let font_search_paths = settings
            .font_search_paths
            .as_ref()
            .ok_or(Error::MissingFontSearchPath)?;
        let library = Library::init()?;
        let all_fonts = all_fonts_in_search_path(font_search_paths, &library);

        let specified_preview_font_names = settings.get_preview_fonts_for(chr);

        Ok(filter_fonts_with_preview_font_settings(
            all_fonts,
            &specified_preview_font_names,
        ))
    }

    fn all_fonts_in_search_path(
        font_search_paths: &[PathBuf],
        library: &Library,
    ) -> Vec<FontDescriptor> {
        let mut fonts = vec![];

        for font_search_path in font_search_paths {
            if !font_search_path.is_dir() {
                // Ignore non-directory file paths
                continue;
            }

            // Work the search path ignoring dir iterating errors
            for entry in WalkDir::new(font_search_path)
                .into_iter()
                .filter_map(|entry_result| entry_result.ok())
            {
                let new_face_result = library.new_face(entry.path(), 0);
                if new_face_result.is_err() {
                    // Ignore errors when parsing fonts
                    continue;
                }
                let font_face = new_face_result.unwrap();

                let font_descriptor =
                    try_create_font_descriptor_from_face(entry.path().to_owned(), &font_face);
                if font_descriptor.is_none() {
                    // Ignore invalid font descriptors
                    continue;
                }
                let font_descriptor = font_descriptor.unwrap();

                fonts.push(font_descriptor);
            }
        }

        fonts
    }

    fn try_create_font_descriptor_from_face(path: PathBuf, face: &Face) -> Option<FontDescriptor> {
        let family_name = face.family_name()?;
        let postscript_name = face.postscript_name()?;
        Some(FontDescriptor {
            path,
            family_name,
            full_name: postscript_name, // Using PostScript name as full name
        })
    }
}

fn filter_fonts_with_preview_font_settings(
    all_available_fonts: Vec<FontDescriptor>,
    specified_preview_font_names: &[String],
) -> Vec<FontDescriptor> {
    if specified_preview_font_names.is_empty() {
        all_available_fonts
    } else {
        all_available_fonts
            .iter()
            .filter(|font_descriptor| {
                fn is_matching_name(name: &str, acceptable_names: &[String]) -> bool {
                    acceptable_names
                        .iter()
                        .any(|acceptable_name| name.contains(acceptable_name))
                }

                if is_matching_name(&font_descriptor.family_name, specified_preview_font_names) {
                    return true;
                }

                if is_matching_name(&font_descriptor.full_name, specified_preview_font_names) {
                    return true;
                }

                // Neither family name nor full name matches
                false
            })
            .cloned()
            .collect()
    }
}

// TODO: Unit tests

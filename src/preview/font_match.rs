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

use std::fs::read_dir;

use freetype::Library;

use super::{Error, Result};
use crate::settings::Settings;

#[cfg(target_family = "unix")]
pub fn fonts_for(chr: char, settings: &Settings) -> Result<Vec<String>> {
    // TODO: call fonts_for_no_fontconfig if was set

    use std::ffi;
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::slice;

    use fontconfig::fontconfig as fc;
    use scopeguard::defer;

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
        fc::FcObjectSetAdd(object_set, ffi::CString::new("file")?.as_ptr());

        let font_set = fc::FcFontList(std::ptr::null_mut(), pattern, object_set);
        defer! {
            fc::FcFontSetDestroy(font_set);
        }

        if (*font_set).nfont <= 0 {
            return Err(Box::new(Error::GlyphNotFound { chr }));
        }

        let mut font_paths = vec![];
        {
            font_paths.reserve((*font_set).nfont as usize);

            let patterns_slice = slice::from_raw_parts::<*mut fc::FcPattern>(
                (*font_set).fonts,
                (*font_set).nfont as usize,
            );

            for pattern in patterns_slice {
                let mut value: *mut u8 = std::ptr::null_mut();
                let result = fc::FcPatternGetString(
                    *pattern,
                    ffi::CString::new("file")?.as_ptr(),
                    0,
                    &mut value as *mut *mut u8,
                );

                if result != fc::FcResultMatch {
                    return Err(Box::new(Error::GlyphNotFound { chr }));
                }

                let font_path = CStr::from_ptr(value as *mut c_char).to_str()?.to_owned();
                font_paths.push(font_path);
            }

            font_paths.sort();
        }
        Ok(font_paths)
    }
}

#[cfg(target_family = "windows")]
pub fn fonts_for(chr: char, settings: &Settings) -> Result<Vec<String>> {
    fonts_for_no_fontconfig(chr, settings)
}

fn fonts_for_no_fontconfig(chr: char, settings: &Settings) -> Result<Vec<String>> {
    let font_search_paths = settings
        .font_search_paths
        .as_ref()
        .ok_or(Error::MissingFontSearchPath)?;

    let matched_fonts = {
        let mut matched_fonts = vec![];

        let library = Library::init()?;
        for font_search_path in font_search_paths {
            if !font_search_path.is_dir() {
                // Ignore non-directory file paths
                continue;
            }

            let read_dir_result = read_dir(font_search_path);
            if read_dir_result.is_err() {
                // Ignore errors when creating directory iterators
                continue;
            }
            let dir_iter = read_dir_result.unwrap();

            for dir_entry in dir_iter {
                if dir_entry.is_err() {
                    // Ignore dir iterating errors
                    continue;
                }
                let entry = dir_entry.unwrap();

                let new_face_result = library.new_face(entry.path(), 0);
                if new_face_result.is_err() {
                    // Ignore errors when parsing fonts
                    continue;
                }
                let font_face = new_face_result.unwrap();

                let glyph_index = font_face.get_char_index(chr as usize);
                if glyph_index == 0 {
                    // No glyph is found for the given character in this font face
                    continue;
                }

                let entry_path = entry.path();
                let path_to_str_result = entry_path.to_str();
                if path_to_str_result.is_none() {
                    // Ignore file path encoding conversion error
                    continue;
                }
                let font_path = path_to_str_result.unwrap();

                // TODO: Do we have to do the copy?
                matched_fonts.push(font_path.to_owned());
            }
        }

        matched_fonts
    };
    Ok(matched_fonts)
}

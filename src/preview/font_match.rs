use std::ffi;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

use fontconfig::fontconfig as fc;

use crate::preview::{Error, Result};

pub fn fonts_for(chr: char) -> Result<Vec<String>> {
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

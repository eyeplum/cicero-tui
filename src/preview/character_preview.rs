use std::cmp::min;
use std::error;
use std::fmt;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2I;

#[cfg(unix)]
use fontconfig::fontconfig::{
    FcCharSetAddChar, FcCharSetCreate, FcFontList, FcObjectSetAdd, FcObjectSetCreate,
    FcPatternAddCharSet, FcPatternCreate, FcPatternGetString, FcResultMatch,
};
#[cfg(unix)]
use std::ffi;
#[cfg(unix)]
use std::ffi::CStr;
#[cfg(unix)]
use std::os::raw::c_char;

#[derive(Debug, Copy, Clone)]
pub struct RenderSize {
    pub width: usize,
    pub height: usize,
}

impl RenderSize {
    pub fn new(width: usize, height: usize) -> Self {
        RenderSize { width, height }
    }
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

fn search_for_glyph_in_system_fonts(chr: char) -> Result<(Font, u32)> {
    // Use font-kit to find the best Sans Serif font
    let system_source = SystemSource::new();
    let default_font = system_source
        .select_best_match(&[FamilyName::SansSerif], &Properties::default())?
        .load()?;
    if let Some(glyph_id) = default_font.glyph_for_char(chr) {
        return Ok((default_font, glyph_id));
    }

    #[cfg(unix)]
    // Failed to find a Sans Serif font for chr, try fontconfig (because servo-fontconfig-sys is
    // only available on Unix platforms, so font fallback is only available on Unix as well)
    // TODO: Fix memory leaks
    // TODO: Fix force unwraps
    // TODO: Prefer Sans Serif fonts in fallback
    unsafe {
        let char_set = FcCharSetCreate();
        FcCharSetAddChar(char_set, chr as u32);

        let pattern = FcPatternCreate();
        FcPatternAddCharSet(
            pattern,
            ffi::CString::new("charset").unwrap().as_ptr(),
            char_set,
        );

        let object_set = FcObjectSetCreate();
        FcObjectSetAdd(object_set, ffi::CString::new("file").unwrap().as_ptr());

        let font_set = FcFontList(std::ptr::null_mut(), pattern, object_set);
        if (*font_set).nfont > 0 {
            let mut value: *mut u8 = std::ptr::null_mut();
            let result = FcPatternGetString(
                *(*font_set).fonts,
                ffi::CString::new("file").unwrap().as_ptr(),
                0,
                &mut value as *mut *mut u8,
            );

            if result == FcResultMatch {
                let font_path = CStr::from_ptr(value as *mut c_char).to_str()?;
                let fallback_font = Font::from_path(font_path, 0)?;
                if let Some(glyph_id) = fallback_font.glyph_for_char(chr) {
                    return Ok((fallback_font, glyph_id));
                }
            }
        }
    }

    // Unable to find a font for chr
    Err(Box::new(Error::GlyphNotFound { chr }))
}

#[derive(Debug)]
pub struct CharacterPreview {
    pub bitmap: Vec<Vec<u8>>, // TODO: This naive 2D vector is not really optimized
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

impl CharacterPreview {
    pub fn new(chr: char, render_size: RenderSize) -> Result<CharacterPreview> {
        let (font, glyph_id) = search_for_glyph_in_system_fonts(chr)?;

        // TODO: What's the relation between point size and pixel size?
        let point_size = min(render_size.width, render_size.height) as f32;
        let transform = Transform2F::default();
        let hinting = HintingOptions::None;
        let rasterization = RasterizationOptions::Bilevel;

        let raster_bounds =
            font.raster_bounds(glyph_id, point_size, transform, hinting, rasterization)?;

        let mut canvas = Canvas::new(
            Vector2I::new(raster_bounds.width(), raster_bounds.height()),
            Format::A8,
        );

        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            point_size,
            Transform2F::from_translation(-raster_bounds.origin().to_f32()) * transform,
            hinting,
            rasterization,
        )?;

        let mut bitmap =
            vec![vec![0; raster_bounds.width() as usize]; raster_bounds.height() as usize];

        for y in 0..raster_bounds.height() {
            let row_start = y as usize * canvas.stride;
            let row_end = (y + 1) as usize * canvas.stride;
            let row = &canvas.pixels[row_start..row_end];
            for x in 0..raster_bounds.width() {
                bitmap[y as usize][x as usize] = row[x as usize];
            }
        }

        Ok(CharacterPreview {
            bitmap,
            original_glyph_size: RenderSize::new(
                raster_bounds.width() as usize,
                raster_bounds.height() as usize,
            ),
        })
    }
}

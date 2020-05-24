use std::cmp::min;

use freetype::Library;

use crate::preview::font_match::fonts_for;
use crate::preview::{Error, Result};

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

#[derive(Debug)]
pub struct CharacterPreview {
    pub bitmap: Vec<Vec<u8>>, // TODO: This naive 2D vector is not really optimized
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

impl CharacterPreview {
    pub fn new(chr: char, render_size: RenderSize) -> Result<CharacterPreview> {
        let fonts = fonts_for(chr)?;
        if fonts.is_empty() {
            return Err(Box::new(Error::GlyphNotFound { chr }));
        }

        let library = Library::init()?;

        let font_face = library.new_face(&fonts[0], 0)?;
        font_face.set_pixel_sizes(render_size.width as u32, render_size.height as u32)?;
        font_face.load_char(chr as usize, freetype::face::LoadFlag::RENDER)?;

        let glyph = font_face.glyph();

        let mut bitmap = vec![vec![0; render_size.width as usize]; render_size.height as usize];

        let glyph_bitmap = glyph.bitmap();
        let x_max = min(render_size.width, glyph_bitmap.width() as usize);
        let y_max = min(render_size.height, glyph_bitmap.rows() as usize);

        let glyph_bitmap_buffer = glyph_bitmap.buffer();

        for x in 0..x_max {
            for y in 0..y_max {
                bitmap[y][x] = glyph_bitmap_buffer[y * x_max + x];
            }
        }

        Ok(CharacterPreview {
            bitmap,
            original_glyph_size: RenderSize::new(
                glyph_bitmap.width() as usize,
                glyph_bitmap.rows() as usize,
            ),
        })
    }
}

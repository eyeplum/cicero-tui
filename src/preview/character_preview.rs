use std::cmp::min;

use freetype::{Face, Library};

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
pub struct RenderedCharacter {
    pub bitmap: Vec<Vec<u8>>, // TODO: This naive 2D vector is not really optimized
    pub glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

const STATEFUL_VEC_CURRENT_INVALID: usize = usize::max_value();

#[derive(Debug)]
struct StatefulVec<T> {
    storage: Vec<T>,
    current: usize,
}

impl<T> StatefulVec<T> {
    fn new(storage: Vec<T>, current: usize) -> Self {
        if current + 1 > storage.len() {
            StatefulVec {
                storage,
                current: STATEFUL_VEC_CURRENT_INVALID,
            }
        } else {
            StatefulVec { storage, current }
        }
    }

    fn has_previous(&self) -> bool {
        !self.storage.is_empty() && self.current != STATEFUL_VEC_CURRENT_INVALID && self.current > 0
    }

    fn select_previous(&mut self) {
        if !self.has_previous() {
            return;
        }

        self.current -= 1;
    }

    fn has_next(&self) -> bool {
        !self.storage.is_empty()
            && self.current != STATEFUL_VEC_CURRENT_INVALID
            && (self.current + 1 < self.storage.len())
    }

    fn select_next(&mut self) {
        if !self.has_next() {
            return;
        }

        self.current += 1;
    }

    fn current_item(&self) -> &T {
        &self.storage[self.current]
    }
}

pub struct CharacterPreview {
    pub chr: char,

    paths_for_matching_fonts: StatefulVec<String>,

    library: Library, // TODO: Make this a long-living object to avoid re-init it for each character
    current_font: Face,
}

impl CharacterPreview {
    pub fn new(chr: char) -> Result<CharacterPreview> {
        let font_paths = fonts_for(chr)?;
        if font_paths.is_empty() {
            return Err(Box::new(Error::GlyphNotFound { chr }));
        }

        let paths_for_matching_fonts = StatefulVec::new(font_paths, 0);

        let library = Library::init()?;
        let current_font = library.new_face(&paths_for_matching_fonts.current_item(), 0)?;

        Ok(CharacterPreview {
            chr,
            paths_for_matching_fonts,
            library,
            current_font,
        })
    }

    pub fn has_previous_font(&self) -> bool {
        self.paths_for_matching_fonts.has_previous()
    }

    pub fn select_previous_font(&mut self) -> Result<()> {
        self.paths_for_matching_fonts.select_previous();
        self.current_font = self
            .library
            .new_face(self.paths_for_matching_fonts.current_item(), 0)?;
        Ok(())
    }

    pub fn has_next_font(&self) -> bool {
        self.paths_for_matching_fonts.has_next()
    }

    pub fn select_next_font(&mut self) -> Result<()> {
        self.paths_for_matching_fonts.select_next();
        self.current_font = self
            .library
            .new_face(self.paths_for_matching_fonts.current_item(), 0)?;
        Ok(())
    }

    pub fn get_current_font_display_name(&self) -> String {
        let family_name = self
            .current_font
            .family_name()
            .unwrap_or_else(|| "Unknown Family".to_owned());
        let style_name = self
            .current_font
            .style_name()
            .unwrap_or_else(|| "Unknown Style".to_owned());
        format!("{} - {}", family_name, style_name)
    }

    pub fn render(&self, size: RenderSize) -> Result<RenderedCharacter> {
        self.current_font
            .set_pixel_sizes(size.width as u32, size.height as u32)?;
        self.current_font
            .load_char(self.chr as usize, freetype::face::LoadFlag::RENDER)?;

        let (bitmap, glyph_size) = {
            let mut pixels = vec![vec![0; size.width as usize]; size.height as usize];

            let glyph_bitmap = self.current_font.glyph().bitmap();
            let x_max = min(size.width, glyph_bitmap.width() as usize);
            let y_max = min(size.height, glyph_bitmap.rows() as usize);

            let glyph_bitmap_buffer = glyph_bitmap.buffer();

            for x in 0..x_max {
                for y in 0..y_max {
                    pixels[y][x] = glyph_bitmap_buffer[y * x_max + x];
                }
            }

            (pixels, RenderSize::new(x_max, y_max))
        };

        Ok(RenderedCharacter { bitmap, glyph_size })
    }
}

use freetype::{Face, GlyphSlot, Library};
use std::cmp::min;

#[derive(Debug)]
pub struct RenderSize {
    pub width: usize,
    pub height: usize,
}

impl RenderSize {
    pub fn new(width: usize, height: usize) -> Self {
        RenderSize { width, height }
    }
}

pub struct RenderedCharacter {
    pub bitmap: Vec<Vec<u8>>,
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

fn render_glyph(glyph: &GlyphSlot, size: &RenderSize) -> RenderedCharacter {
    let mut bitmap = vec![vec![0; size.width as usize]; size.height as usize];

    let glyph_bitmap = glyph.bitmap();
    let x_max = min(size.width, glyph_bitmap.width() as usize);
    let y_max = min(size.height, glyph_bitmap.rows() as usize);

    let glyph_bitmap_buffer = glyph_bitmap.buffer();

    for x in 0..x_max {
        for y in 0..y_max {
            bitmap[y][x] = glyph_bitmap_buffer[y * x_max + x];
        }
    }

    RenderedCharacter {
        bitmap,
        original_glyph_size: RenderSize::new(
            glyph_bitmap.width() as usize,
            glyph_bitmap.rows() as usize,
        ),
    }
}

pub struct CharacterPreview {
    font_face: Face,
}

impl CharacterPreview {
    pub fn new(font_path: &str) -> Option<Self> {
        match Library::init() {
            Ok(library) => match library.new_face(font_path, 0) {
                Ok(font_face) => Some(Self { font_face }),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn preview_for(&self, ch: char, size: &RenderSize) -> Option<RenderedCharacter> {
        if let Err(_) = self
            .font_face
            .set_pixel_sizes(size.width as u32, size.height as u32)
        {
            return None;
        }

        if let Err(_) = self
            .font_face
            .load_char(ch as usize, freetype::face::LoadFlag::RENDER)
        {
            return None;
        }

        Some(render_glyph(self.font_face.glyph(), size))
    }
}

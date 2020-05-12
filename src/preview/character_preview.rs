use freetype::{Face, GlyphSlot, Library};
use std::cmp::min;

const WIDTH: usize = 32;
const HEIGHT: usize = 16;

pub type Bitmap = [[u8; WIDTH]; HEIGHT];

fn glyph_to_bitmap(glyph: &GlyphSlot) -> Bitmap {
    let mut bitmap = [[0; WIDTH]; HEIGHT];

    let glyph_bitmap = glyph.bitmap();
    let x_max = min(WIDTH, glyph_bitmap.width() as usize);
    let y_max = min(HEIGHT, glyph_bitmap.rows() as usize);

    let glyph_bitmap_buffer = glyph_bitmap.buffer();

    for x in 0..x_max {
        for y in 0..y_max {
            bitmap[y][x] = glyph_bitmap_buffer[y * x_max + x];
        }
    }

    bitmap
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

    pub fn preview_for(&self, ch: char) -> Option<Bitmap> {
        // TODO: What is the size?
        if let Err(_) = self.font_face.set_pixel_sizes(WIDTH as u32, HEIGHT as u32) {
            return None;
        }

        if let Err(_) = self
            .font_face
            .load_char(ch as usize, freetype::face::LoadFlag::RENDER)
        {
            return None;
        }

        Some(glyph_to_bitmap(self.font_face.glyph()))
    }
}

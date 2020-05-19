use std::cmp::min;
use std::error;
use std::fmt;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2I;

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

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    GlyphNotFound { chr: char, font_name: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GlyphNotFound { chr, font_name } => write!(
                f,
                "Glyph for U+{:04X} not found in font '{}'",
                *chr as u32, font_name
            ),
        }
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub struct CharacterPreview {
    pub bitmap: Vec<Vec<u8>>, // TODO: This naive 2D vector is not really optimized
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

impl CharacterPreview {
    pub fn new(chr: char, render_size: RenderSize) -> Result<CharacterPreview> {
        // TODO: Implement font fallback
        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::default())?
            .load()?;

        let glyph_id = match font.glyph_for_char(chr) {
            Some(glyph_id) => glyph_id,
            None => {
                return Err(Box::new(Error::GlyphNotFound {
                    chr,
                    font_name: font.full_name(),
                }));
            }
        };

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

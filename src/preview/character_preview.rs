use std::cmp::min;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2I;

use crate::preview::Result;

#[cfg(unix)]
use crate::preview::unix_font_fallback::fallback_font_for;

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

fn find_font_and_glyph_for(chr: char) -> Result<(Font, u32)> {
    // Use font-kit to find the default Sans Serif font
    let system_source = SystemSource::new();
    let default_font = system_source
        .select_best_match(&[FamilyName::SansSerif], &Properties::default())?
        .load()?;
    if let Some(glyph_id) = default_font.glyph_for_char(chr) {
        return Ok((default_font, glyph_id));
    }

    #[cfg(unix)]
    // Unable to find a glyph for chr in the default font, look for a fallback font if on Unix
    return fallback_font_for(chr);

    #[cfg(not(unix))]
    // Unable to find a glyph for chr in the default font
    Err(Box::new(crate::preview::Error::GlyphNotFound { chr }))
}

#[derive(Debug)]
pub struct CharacterPreview {
    pub bitmap: Vec<Vec<u8>>, // TODO: This naive 2D vector is not really optimized
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

impl CharacterPreview {
    pub fn new(chr: char, render_size: RenderSize) -> Result<CharacterPreview> {
        let (font, glyph_id) = find_font_and_glyph_for(chr)?;

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

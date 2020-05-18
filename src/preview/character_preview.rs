use std::cmp::min;

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

#[derive(Debug)]
pub struct CharacterPreview {
    pub bitmap: Vec<Vec<u8>>,
    pub original_glyph_size: RenderSize, // TODO: Expose all glyph metrics
}

impl CharacterPreview {
    pub fn new(chr: char, render_size: RenderSize) -> Option<CharacterPreview> {
        if let Ok(font) =
            SystemSource::new().select_best_match(&[FamilyName::SansSerif], &Properties::default())
        {
            if let Ok(loaded_font) = font.load() {
                if let Some(glyph_id) = loaded_font.glyph_for_char(chr) {
                    let point_size = min(render_size.width, render_size.height) as f32; // TODO: What's the relation between point size and pixel size?
                    let transform = Transform2F::default();
                    let hinting = HintingOptions::None;
                    let rasterization = RasterizationOptions::Bilevel;

                    match loaded_font.raster_bounds(
                        glyph_id,
                        point_size,
                        transform,
                        hinting,
                        rasterization,
                    ) {
                        Err(_) => return None,
                        Ok(raster_bounds) => {
                            let mut canvas = Canvas::new(
                                Vector2I::new(raster_bounds.width(), raster_bounds.height()),
                                Format::A8,
                            );

                            match loaded_font.rasterize_glyph(
                                &mut canvas,
                                glyph_id,
                                point_size,
                                Transform2F::from_translation(-raster_bounds.origin().to_f32())
                                    * transform,
                                hinting,
                                rasterization,
                            ) {
                                Err(_) => None,
                                Ok(_) => {
                                    let mut bitmap = vec![
                                        vec![0; raster_bounds.width() as usize];
                                        raster_bounds.height() as usize
                                    ];

                                    for y in 0..raster_bounds.height() {
                                        let row_start = y as usize * canvas.stride;
                                        let row_end = (y + 1) as usize * canvas.stride;
                                        let row = &canvas.pixels[row_start..row_end];
                                        for x in 0..raster_bounds.width() {
                                            bitmap[y as usize][x as usize] = row[x as usize];
                                        }
                                    }

                                    Some(CharacterPreview {
                                        bitmap,
                                        original_glyph_size: RenderSize::new(
                                            raster_bounds.width() as usize,
                                            raster_bounds.height() as usize,
                                        ),
                                    })
                                }
                            }
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

use std::cmp::min;

use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders};

use crate::preview::{CharacterPreview, RenderSize};
use crate::view::main_view::TerminalFrame;

const BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL: usize = 2;
const BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL: usize = 4;

const RENDER_PADDING_IN_CELLS: usize = 4;

pub struct CharacterPreviewCanvas;

impl CharacterPreviewCanvas {
    pub fn new() -> Self {
        CharacterPreviewCanvas {}
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect, chr: char) {
        // TODO: Make sure subtractions never overflow
        let canvas_pixel_width = (rect.width as usize - RENDER_PADDING_IN_CELLS)
            * BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL;
        let canvas_pixel_height = (rect.height as usize - RENDER_PADDING_IN_CELLS)
            * BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL;

        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .paint(|ctx| {
                let render_pixel_length = min(canvas_pixel_width, canvas_pixel_height);
                let render_pixel_size = RenderSize::new(render_pixel_length, render_pixel_length);
                if let Ok(character_preview) = CharacterPreview::new(chr, render_pixel_size) {
                    let canvas_pixel_size =
                        RenderSize::new(canvas_pixel_width, canvas_pixel_height);

                    let shape = CharacterPreviewShape::new(character_preview, canvas_pixel_size);
                    ctx.draw(&shape);
                }
                // TODO: Handle error
            });

        frame.render_widget(canvas, rect);
    }
}

struct CharacterPreviewShape {
    character_preview: CharacterPreview,
    x_padding: usize,
    y_padding: usize,
}

impl CharacterPreviewShape {
    fn new(character_preview: CharacterPreview, canvas_pixel_size: RenderSize) -> Self {
        // TODO: Calculate padding according to glyph metrics
        let x_padding = (canvas_pixel_size.width - character_preview.original_glyph_size.width) / 2;
        let y_padding =
            (canvas_pixel_size.height - character_preview.original_glyph_size.height) / 2;

        CharacterPreviewShape {
            character_preview,
            x_padding,
            y_padding,
        }
    }
}

impl Shape for CharacterPreviewShape {
    fn draw(&self, painter: &mut Painter) {
        for (y, row) in self.character_preview.bitmap.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                if *pixel == 0u8 {
                    continue;
                }

                painter.paint(
                    x + self.x_padding as usize,
                    y + self.y_padding as usize,
                    Color::Reset,
                )
            }
        }
    }
}

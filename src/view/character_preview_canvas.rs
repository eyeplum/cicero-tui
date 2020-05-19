use std::cmp::min;

use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders};

use crate::preview::{CharacterPreview, RenderSize};
use crate::view::main_view::TerminalFrame;

const BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL: u16 = 2;
const BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL: u16 = 4;

const RENDER_PADDING_IN_CELLS: u16 = 4;

pub struct CharacterPreviewCanvas;

impl CharacterPreviewCanvas {
    pub fn new() -> Self {
        CharacterPreviewCanvas {}
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect, chr: char) {
        if rect.width < RENDER_PADDING_IN_CELLS || rect.height < RENDER_PADDING_IN_CELLS {
            return;
        }

        let canvas_pixel_width =
            (rect.width - RENDER_PADDING_IN_CELLS) * BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL;
        let canvas_pixel_height =
            (rect.height - RENDER_PADDING_IN_CELLS) * BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL;

        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .paint(|ctx| {
                let render_pixel_length = min(canvas_pixel_width, canvas_pixel_height);
                let render_pixel_size =
                    RenderSize::new(render_pixel_length as usize, render_pixel_length as usize);

                let canvas_pixel_size =
                    RenderSize::new(canvas_pixel_width as usize, canvas_pixel_height as usize);

                match CharacterPreview::new(chr, render_pixel_size) {
                    Ok(character_preview) => {
                        let x_padding = (canvas_pixel_size.width
                            - character_preview.original_glyph_size.width)
                            / 2;
                        let y_padding = (canvas_pixel_size.height
                            - character_preview.original_glyph_size.height)
                            / 2;
                        ctx.draw(&CharacterPreviewShape {
                            character_preview,
                            x_padding,
                            y_padding,
                        })
                    }
                    Err(_) => {
                        let x_padding = (canvas_pixel_size.width - render_pixel_size.width) / 2;
                        let y_padding = (canvas_pixel_size.height - render_pixel_size.height) / 2;
                        ctx.draw(&ToufuShape {
                            size: render_pixel_size,
                            x_padding,
                            y_padding,
                        })
                    }
                };
            });

        frame.render_widget(canvas, rect);
    }
}

struct CharacterPreviewShape {
    character_preview: CharacterPreview,
    x_padding: usize,
    y_padding: usize,
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

struct ToufuShape {
    size: RenderSize,
    x_padding: usize,
    y_padding: usize,
}

impl Shape for ToufuShape {
    fn draw(&self, painter: &mut Painter) {
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                painter.paint(x + self.x_padding, y + self.y_padding, Color::Reset)
            }
        }
    }
}

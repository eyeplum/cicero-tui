use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders};

use crate::preview::{Bitmap, CharacterPreview, RenderSize};
use crate::view::main_view::TerminalFrame;

pub struct CharacterPreviewCanvas {
    pub character_preview: CharacterPreview,
}

impl CharacterPreviewCanvas {
    pub fn new() -> Self {
        let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"; // FIXME: Remove hard coded font path
        CharacterPreviewCanvas {
            character_preview: CharacterPreview::new(font_path).unwrap(), // FIXME: Force unwrap
        }
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect, chr: char) {
        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .x_bounds([0.0, rect.width as f64])
            .y_bounds([0.0, rect.height as f64])
            .paint(|ctx| {
                ctx.draw(&CharacterPreviewShape {
                    bitmap: self
                        .character_preview
                        .preview_for(chr, &RenderSize::new(64, 64))
                        .unwrap(), // FIXME: Force unwrap
                });
            });

        frame.render_widget(canvas, rect);
    }
}

struct CharacterPreviewShape {
    bitmap: Bitmap,
}

impl Shape for CharacterPreviewShape {
    fn draw(&self, painter: &mut Painter) {
        for y in 0..self.bitmap.len() {
            for x in 0..self.bitmap[y].len() {
                match self.bitmap[y][x] {
                    p if p == 0 => {}
                    _ => painter.paint(x, y, Color::Reset),
                };
            }
        }
    }
}

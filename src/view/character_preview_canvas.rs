use std::cmp::min;

use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders};

use crate::preview::{CharacterPreview, RenderSize, RenderedCharacter};
use crate::view::main_view::TerminalFrame;

const BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL: usize = 2;
const BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL: usize = 4;

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
        let preview_render_pixel_size = min(
            (rect.width as usize - 1) * BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL,
            (rect.height as usize - 1) * BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL,
        );
        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .paint(|ctx| {
                ctx.draw(&CharacterPreviewShape {
                    rendered_character: self
                        .character_preview
                        .preview_for(
                            chr,
                            &RenderSize::new(preview_render_pixel_size, preview_render_pixel_size),
                        )
                        .unwrap(), // FIXME: Force unwrap
                });
            });

        frame.render_widget(canvas, rect);
    }
}

struct CharacterPreviewShape {
    rendered_character: RenderedCharacter,
}

impl Shape for CharacterPreviewShape {
    fn draw(&self, painter: &mut Painter) {
        let bitmap = &self.rendered_character.bitmap;

        for y in 0..bitmap.len() {
            for x in 0..bitmap[y].len() {
                match bitmap[y][x] {
                    p if p == 0 => {}
                    _ => painter.paint(x, y, Color::Reset),
                };
            }
        }
    }
}

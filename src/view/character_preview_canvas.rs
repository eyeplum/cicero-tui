use std::cmp::min;

use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders};

use crate::preview::{CharacterPreview, RenderSize, RenderedCharacter};
use crate::view::main_view::TerminalFrame;

const BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL: usize = 2;
const BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL: usize = 4;

const RENDER_PADDING_IN_CELLS: usize = 4;

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
        let canvas_pixel_width = (rect.width as usize - RENDER_PADDING_IN_CELLS)
            * BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL;
        let canvas_pixel_height = (rect.height as usize - RENDER_PADDING_IN_CELLS)
            * BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL;

        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .paint(|ctx| {
                let render_pixel_length = min(canvas_pixel_width, canvas_pixel_height);
                let render_pixel_size = RenderSize::new(render_pixel_length, render_pixel_length);
                let rendered_character = self
                    .character_preview
                    .preview_for(chr, &render_pixel_size)
                    .unwrap(); // FIXME: Force unwrap

                let canvas_pixel_size = RenderSize::new(canvas_pixel_width, canvas_pixel_height);

                let shape = CharacterPreviewShape::new(rendered_character, canvas_pixel_size);
                ctx.draw(&shape);
            });

        frame.render_widget(canvas, rect);
    }
}

struct CharacterPreviewShape {
    rendered_character: RenderedCharacter,
    x_padding: usize,
    y_padding: usize,
}

impl CharacterPreviewShape {
    fn new(rendered_character: RenderedCharacter, canvas_pixel_size: RenderSize) -> Self {
        // TODO: Calculate padding according to glyph metrics
        let x_padding =
            (canvas_pixel_size.width - rendered_character.original_glyph_size.width) / 2;
        let y_padding =
            (canvas_pixel_size.height - rendered_character.original_glyph_size.height) / 2;

        CharacterPreviewShape {
            rendered_character,
            x_padding,
            y_padding,
        }
    }
}

impl Shape for CharacterPreviewShape {
    fn draw(&self, painter: &mut Painter) {
        let bitmap = &self.rendered_character.bitmap;

        for y in 0..bitmap.len() {
            for x in 0..bitmap[y].len() {
                match bitmap[y][x] {
                    p if p == 0 => {}
                    _ => painter.paint(x + self.x_padding, y + self.y_padding, Color::Reset),
                };
            }
        }
    }
}

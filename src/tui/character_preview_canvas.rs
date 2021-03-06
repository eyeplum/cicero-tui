// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

use std::cmp::min;

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders, Paragraph, Text};

use super::main_view::TerminalFrame;
use crate::preview::{CharacterPreview, RenderSize, RenderedCharacter, Result};
use crate::settings::Settings;

const BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL: u16 = 2;
const BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL: u16 = 4;

const RENDER_PADDING_IN_CELLS: u16 = 4;

pub struct CharacterPreviewCanvas {
    character_preview: CharacterPreview,
}

impl CharacterPreviewCanvas {
    pub fn try_new(
        chr: char,
        selected_font_path: Option<&String>,
        settings: &Settings,
    ) -> Result<Self> {
        let character_preview = CharacterPreview::new(chr, selected_font_path, settings)?;
        Ok(CharacterPreviewCanvas { character_preview })
    }

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Min(RENDER_PADDING_IN_CELLS),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(rect);

        self.draw_character_preview(frame, chunks[0]);
        self.draw_font_selection(frame, chunks[1]);
        self.draw_borders(frame, rect);
    }

    pub fn get_current_preview_font(&self) -> Option<String> {
        self.character_preview.get_current_font_path()
    }

    pub fn previous_preview_font(&mut self) {
        let _ = self.character_preview.select_previous_font();
    }

    pub fn next_preview_font(&mut self) {
        let _ = self.character_preview.select_next_font();
    }

    fn draw_character_preview(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        if rect.width < RENDER_PADDING_IN_CELLS || rect.height < RENDER_PADDING_IN_CELLS {
            return;
        }

        let canvas = Canvas::default().paint(|ctx| {
            let canvas_pixel_width =
                (rect.width - RENDER_PADDING_IN_CELLS) * BRAILLE_PATTERN_DOTS_PER_CELL_HORIZONTAL;
            let canvas_pixel_height =
                (rect.height - RENDER_PADDING_IN_CELLS) * BRAILLE_PATTERN_DOTS_PER_CELL_VERTICAL;
            let canvas_pixel_size =
                RenderSize::new(canvas_pixel_width as usize, canvas_pixel_height as usize);

            let render_pixel_size = {
                let render_pixel_length = min(canvas_pixel_width, canvas_pixel_height);
                RenderSize::new(render_pixel_length as usize, render_pixel_length as usize)
            };

            match self.character_preview.render(render_pixel_size) {
                Ok(rendered_character) => {
                    let glyph_size = rendered_character.glyph_size;
                    let x_padding = if glyph_size.width < canvas_pixel_size.width {
                        (canvas_pixel_size.width - glyph_size.width) / 2
                    } else {
                        0
                    };
                    let y_padding = if glyph_size.height < canvas_pixel_size.height {
                        (canvas_pixel_size.height - glyph_size.height) / 2
                    } else {
                        0
                    };
                    ctx.draw(&CharacterPreviewShape {
                        rendered_character: &rendered_character,
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
            }
        });

        frame.render_widget(canvas, rect);
    }

    fn draw_font_selection(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(15),
                    Constraint::Min(1),
                    Constraint::Length(15),
                ]
                .as_ref(),
            )
            .direction(Direction::Horizontal)
            .split(rect);

        if self.character_preview.has_previous_font() {
            let help_item = [Text::raw("[\u{2190}]: Prev. Font")];
            let help_text = Paragraph::new(help_item.iter())
                .style(Style::default().fg(Color::LightGreen))
                .alignment(Alignment::Left);
            frame.render_widget(help_text, chunks[0]);
        }
        {
            let font_name = self.character_preview.get_current_font_display_name();
            let help_item = [Text::raw(font_name)];
            let help_text = Paragraph::new(help_item.iter())
                .style(Style::default())
                .alignment(Alignment::Center);
            frame.render_widget(help_text, chunks[1]);
        }
        if self.character_preview.has_next_font() {
            let help_item = [Text::raw("[\u{2192}]: Next Font")];
            let help_text = Paragraph::new(help_item.iter())
                .style(Style::default().fg(Color::LightGreen))
                .alignment(Alignment::Right);
            frame.render_widget(help_text, chunks[2]);
        }
    }

    fn draw_borders(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let block = Block::default().title("Preview").borders(Borders::ALL);
        frame.render_widget(block, rect);
    }
}

struct CharacterPreviewShape<'a> {
    rendered_character: &'a RenderedCharacter,
    x_padding: usize,
    y_padding: usize,
}

impl Shape for CharacterPreviewShape<'_> {
    fn draw(&self, painter: &mut Painter) {
        for (y, row) in self.rendered_character.bitmap.iter().enumerate() {
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

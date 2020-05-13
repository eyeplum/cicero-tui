use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Painter, Shape};
use tui::widgets::{Block, Borders, Paragraph, Text};
use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{Age, GeneralCategory, Name};

use crate::preview::{Bitmap, CharacterPreview};
use crate::ucd::Plane;
use crate::view::code_point_description;
use crate::view::main_view::TerminalFrame;

pub struct CharacterDetailView {
    pub chr: char,
}

impl CharacterDetailView {
    pub fn new(chr: char) -> Self {
        CharacterDetailView { chr }
    }

    const NOT_AVAILABLE_DISPLAY_TEXT: &'static str = "N/A";

    pub fn draw(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(7)].as_ref())
            .direction(Direction::Vertical)
            .split(rect);

        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .x_bounds([0.0, 128.0])
            .y_bounds([0.0, 128.0])
            .paint(|ctx| {
                let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"; // FIXME: Remove hard coded font path
                let character_preview = CharacterPreview::new(font_path).unwrap();
                ctx.draw(&CharacterPreviewShape {
                    bitmap: character_preview.preview_for(self.chr).unwrap(),
                });
            });

        frame.render_widget(canvas, chunks[0]);

        let code_point_description = code_point_description(self.chr);

        let name_description = match Name::of(self.chr) {
            Some(name) => name.to_string(),
            None => "".to_owned(),
        };

        let age_description = match Age::of(self.chr) {
            Some(age) => format!("Unicode {}", age.actual().to_string()),
            None => CharacterDetailView::NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
        };

        let gc = GeneralCategory::of(self.chr);
        let gc_description = format!("{}({})", gc.human_name(), gc.abbr_name());

        let plane_name = Plane::of(self.chr).name;

        let block_name = match unic::ucd::Block::of(self.chr) {
            Some(block) => block.name.to_owned(),
            None => CharacterDetailView::NOT_AVAILABLE_DISPLAY_TEXT.to_owned(),
        };

        let text = [
            Text::styled("General\n\n", Style::default().modifier(Modifier::BOLD)),
            Text::raw(format!("Code Point: {}\n", code_point_description)),
            Text::raw(format!("Name: {}\n", name_description)),
            Text::raw(format!("Age: {}\n", age_description)),
            Text::raw(format!("Plane: {}\n", plane_name)),
            Text::raw(format!("Block: {}\n", block_name)),
            Text::raw(format!("General Category: {}\n", gc_description)),
        ];
        let paragraph = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(&code_point_description),
            )
            .style(Style::default())
            .alignment(Alignment::Center)
            .wrap(true);

        frame.render_widget(paragraph, chunks[1]);
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

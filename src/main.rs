use std::io::stdout;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders, Widget},
    Terminal,
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let stdout = stdout();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    terminal.draw(|mut f| {
        let frame_size = f.size();
        let size = Rect::new(0, 0, frame_size.width / 2, frame_size.height / 2);
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })?;

    disable_raw_mode()
}

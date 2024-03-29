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

use std::io::Stdout;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::Result;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text};
use tui::Frame;
use unic::ucd::UNICODE_VERSION;

use super::character_detail_view::CharacterDetailView;
use super::renderer::ApplicationTerminal;
use super::stateful_graphemes::StatefulGraphemes;
use super::ApplicationState;

pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub struct MainView {
    user_input: String,
    graphemes: StatefulGraphemes,
    character_detail_view: Option<CharacterDetailView>,
}

const PAGE_CONTROL_STEP_SIZE: usize = 10;

impl MainView {
    pub fn new(user_input: String) -> Self {
        let graphemes = StatefulGraphemes::new(&user_input);
        MainView {
            user_input,
            graphemes,
            character_detail_view: None,
        }
    }

    pub fn update(
        &mut self,
        terminal: &mut ApplicationTerminal,
        app_state: &mut ApplicationState,
    ) -> Result<()> {
        terminal.draw(|mut frame| {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Vertical)
                .split(frame.size());

            self.draw_user_input(&mut frame, chunks[0]);

            if self.character_detail_view.is_some() {
                let grapheme_list_chunks = Layout::default()
                    .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                    .direction(Direction::Horizontal)
                    .split(chunks[1]);

                self.draw_graphemes_list(&mut frame, grapheme_list_chunks[0]);
                self.character_detail_view
                    .as_mut()
                    .unwrap()
                    .draw(&mut frame, grapheme_list_chunks[1]);
            } else {
                self.draw_graphemes_list(&mut frame, chunks[1])
            }

            self.draw_status_bar(&mut frame, chunks[2]);
        })?;

        if let Event::Key(event) = read()? {
            self.handle_key_event(event, app_state);
        }

        Ok(())
    }

    fn draw_user_input(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let user_input_items = [Text::raw(&self.user_input)];
        let user_input_paragraph = Paragraph::new(user_input_items.iter())
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(user_input_paragraph, rect);
    }

    fn draw_graphemes_list(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let graphemes_list = List::new(
            self.graphemes
                .rows
                .iter()
                .map(|row| Text::raw(row.to_string())),
        )
        .block(Block::default().borders(Borders::ALL).title("Graphemes"))
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .modifier(Modifier::BOLD),
        )
        .highlight_symbol(">");

        frame.render_stateful_widget(graphemes_list, rect, &mut self.graphemes.state);
    }

    fn draw_status_bar(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let status_bar_chunks = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .direction(Direction::Horizontal)
            .split(rect);

        let help_item = if self.character_detail_view.is_some() {
            [Text::raw("[ESC]: Hide Detail | [C-D][C-U]: Scroll Detail")]
        } else {
            [Text::raw("[ESC]: Quit")]
        };
        let help_text =
            Paragraph::new(help_item.iter()).style(Style::default().fg(Color::LightGreen));
        frame.render_widget(help_text, status_bar_chunks[0]);

        let unicode_version_item = [Text::raw(format!("Unicode Version {}", UNICODE_VERSION))];
        let unicode_version_text = Paragraph::new(unicode_version_item.iter())
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Right);
        frame.render_widget(unicode_version_text, status_bar_chunks[1]);
    }

    fn handle_key_event(&mut self, event: KeyEvent, app_state: &mut ApplicationState) {
        match event.code {
            KeyCode::Esc => {
                if self.character_detail_view.is_some() {
                    self.character_detail_view = None;
                } else {
                    app_state.keep_running = false;
                }
            }
            KeyCode::Up => {
                self.graphemes.select_previous();
                if self.character_detail_view.is_some() {
                    self.update_showing_detail(app_state);
                }
            }
            KeyCode::PageUp => {
                self.graphemes.select_previous_n(PAGE_CONTROL_STEP_SIZE);
                if self.character_detail_view.is_some() {
                    self.update_showing_detail(app_state);
                }
            }
            KeyCode::Down => {
                self.graphemes.select_next();
                if self.character_detail_view.is_some() {
                    self.update_showing_detail(app_state);
                }
            }
            KeyCode::PageDown => {
                self.graphemes.select_next_n(PAGE_CONTROL_STEP_SIZE);
                if self.character_detail_view.is_some() {
                    self.update_showing_detail(app_state);
                }
            }
            KeyCode::Left => {
                if let Some(character_detail_view) = &mut self.character_detail_view {
                    character_detail_view.previous_preview_font();
                    app_state.selected_font_path =
                        character_detail_view.get_current_preview_font_path()
                }
            }
            KeyCode::Right => {
                if let Some(character_detail_view) = &mut self.character_detail_view {
                    character_detail_view.next_preview_font();
                    app_state.selected_font_path =
                        character_detail_view.get_current_preview_font_path()
                }
            }
            KeyCode::Enter => self.update_showing_detail(app_state),
            KeyCode::Char(c) => self.handle_character_input(c, event.modifiers),
            KeyCode::Backspace => {
                self.user_input.pop();
                self.graphemes = StatefulGraphemes::new(&self.user_input);
            }
            _ => {}
        };
    }

    fn handle_character_input(&mut self, chr: char, modifiers: KeyModifiers) {
        if chr == 'u'
            && modifiers.contains(KeyModifiers::CONTROL)
            && self.character_detail_view.is_some()
        {
            self.character_detail_view.as_mut().unwrap().scroll_up();
            return;
        }

        if chr == 'd'
            && modifiers.contains(KeyModifiers::CONTROL)
            && self.character_detail_view.is_some()
        {
            self.character_detail_view.as_mut().unwrap().scroll_down();
            return;
        }

        self.user_input.push(chr);
        self.graphemes = StatefulGraphemes::new(&self.user_input);
    }

    fn update_showing_detail(&mut self, app_state: &ApplicationState) {
        if let Some(selected_row_index) = self.graphemes.state.selected() {
            if let Some(chr) = self.graphemes.rows[selected_row_index].code_point {
                self.character_detail_view = Some(CharacterDetailView::new(
                    chr,
                    &app_state.selected_font_path,
                    &app_state.settings,
                ));
            }
        }
    }
}

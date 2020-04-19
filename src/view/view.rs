use std::io::Stdout;

use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::Result;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text};
use tui::Frame;
use unic::char::property::EnumeratedCharProperty;
use unic::ucd::{Age, GeneralCategory, Name};

use crate::renderer::ApplicationTerminal;
use crate::view::code_point_description;
use crate::view::stateful_graphemes::StatefulGraphemes;
use crate::ApplicationState;

type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub struct View {
    user_input: String,
    graphemes: StatefulGraphemes,
    showing_detail: Option<char>,
}

impl View {
    pub fn new() -> Self {
        View {
            user_input: String::default(),
            graphemes: StatefulGraphemes::default(),
            showing_detail: None,
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

            match self.showing_detail {
                None => self.draw_graphemes_list(&mut frame, chunks[1]),
                Some(chr) => {
                    let grapheme_list_chunks = Layout::default()
                        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                        .direction(Direction::Horizontal)
                        .split(chunks[1]);

                    self.draw_graphemes_list(&mut frame, grapheme_list_chunks[0]);
                    self.draw_character_detail(chr, &mut frame, grapheme_list_chunks[1]);
                }
            }

            self.draw_help_text(&mut frame, chunks[2]);
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
        let grapheme_items = self
            .graphemes
            .rows
            .iter()
            .map(|row| Text::raw(row.to_string()));
        let graphemes_list = List::new(grapheme_items.into_iter())
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

    fn draw_character_detail(&mut self, chr: char, frame: &mut TerminalFrame, rect: Rect) {
        let code_point_description = code_point_description(chr);

        let name_description = match Name::of(chr) {
            Some(name) => name.to_string(),
            None => "".to_owned(),
        };

        let age_description = match Age::of(chr) {
            Some(age) => format!("Unicode {}", age.actual().to_string()),
            None => "N/A".to_owned(),
        };

        let gc = GeneralCategory::of(chr);
        let gc_description = format!("{}({})", gc.human_name(), gc.abbr_name());

        let text = [
            Text::raw(format!("Code Point: {}\n", code_point_description)),
            Text::raw(format!("Name: {}\n", name_description)),
            Text::raw(format!("Age: {}\n", age_description)),
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

        frame.render_widget(paragraph, rect);
    }

    fn draw_help_text(&mut self, frame: &mut TerminalFrame, rect: Rect) {
        let help_item;
        if self.showing_detail.is_some() {
            help_item = [Text::raw("esc: quit | q: hide detail")];
        } else {
            help_item = [Text::raw("esc: quit")];
        }

        let help_text =
            Paragraph::new(help_item.iter()).style(Style::default().fg(Color::LightGreen));

        frame.render_widget(help_text, rect);
    }

    fn handle_key_event(&mut self, event: KeyEvent, app_state: &mut ApplicationState) {
        match event.code {
            KeyCode::Esc => app_state.keep_running = false,
            KeyCode::Up => {
                self.graphemes.select_previous();
                if self.showing_detail.is_some() {
                    self.update_showing_detail();
                }
            }
            KeyCode::Down => {
                self.graphemes.select_next();
                if self.showing_detail.is_some() {
                    self.update_showing_detail();
                }
            }
            KeyCode::Enter => self.update_showing_detail(),
            KeyCode::Char(c) => {
                if self.showing_detail.is_some() {
                    if c == 'q' {
                        self.showing_detail = None;
                    }
                    return;
                }

                self.user_input.push(c);
                self.graphemes = StatefulGraphemes::new(&self.user_input);
            }
            KeyCode::Backspace => {
                if self.showing_detail.is_some() {
                    return;
                }

                self.user_input.pop();
                self.graphemes = StatefulGraphemes::new(&self.user_input);
            }
            _ => {}
        };
    }

    fn update_showing_detail(&mut self) {
        if let Some(selected_row_index) = self.graphemes.state.selected() {
            if let Some(chr) = self.graphemes.rows[selected_row_index].code_point {
                self.showing_detail = Some(chr);
            }
        }
    }
}

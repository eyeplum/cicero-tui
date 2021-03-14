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

//!
//! This module implements the Terminal User Interface of Cicero
//!

mod character_detail_view;
mod character_preview_canvas;
mod character_property_view;
mod main_view;
mod renderer;
mod stateful_graphemes;

pub use main_view::MainView;
pub use renderer::run;

use std::path::PathBuf;

use crate::settings::{get_settings, Settings};

#[derive(Debug)]
pub struct ApplicationState {
    pub keep_running: bool,
    pub selected_font_path: Option<PathBuf>,
    pub settings: Settings,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            keep_running: true,
            selected_font_path: None,
            settings: get_settings(),
        }
    }
}

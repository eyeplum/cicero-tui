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

use std::collections::HashSet;
use std::fmt;

use tui::widgets::ListState;
use unic::segment::Graphemes;
use unic::ucd::name::Name;

use crate::ucd::code_point_description;

#[derive(Default)]
pub struct GraphemeRow {
    pub code_point: Option<char>,
}

impl GraphemeRow {
    pub fn new(chr: char) -> Self {
        GraphemeRow {
            code_point: Some(chr),
        }
    }
}

impl fmt::Display for GraphemeRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code_point {
            None => write!(f, ""),
            Some(chr) => {
                let code_point_str = code_point_description(chr);
                let name = match Name::of(chr) {
                    None => "".to_owned(),
                    Some(name) => name.to_string(),
                };
                write!(f, "{}  {}  {}", code_point_str, chr, name)
            }
        }
    }
}

#[derive(Default)]
pub struct StatefulGraphemes {
    pub state: ListState,
    pub rows: Vec<GraphemeRow>,

    grapheme_start_row_indices: HashSet<usize>,
    grapheme_end_row_indices: HashSet<usize>,
}

impl StatefulGraphemes {
    pub fn new(s: &str) -> Self {
        let graphemes: Vec<Vec<char>> = Graphemes::new(s)
            .map(|grapheme| grapheme.chars().collect())
            .collect();

        let mut state = ListState::default();
        if !graphemes.is_empty() {
            state.select(Some(0));
        }

        let mut rows = vec![];

        let mut grapheme_start_row_indices = HashSet::new();
        grapheme_start_row_indices.reserve(graphemes.len());

        let mut grapheme_end_row_indices = HashSet::new();
        grapheme_end_row_indices.reserve(graphemes.len());

        for (i, grapheme) in graphemes.iter().enumerate() {
            for (j, chr) in grapheme.iter().enumerate() {
                if j == 0 {
                    grapheme_start_row_indices.insert(rows.len());
                }

                if j + 1 == grapheme.len() {
                    grapheme_end_row_indices.insert(rows.len());
                }

                rows.push(GraphemeRow::new(*chr));
            }

            if i + 1 < graphemes.len() {
                rows.push(GraphemeRow::default());
            }
        }

        StatefulGraphemes {
            state,
            rows,
            grapheme_start_row_indices,
            grapheme_end_row_indices,
        }
    }

    pub fn select_next(&mut self) {
        match self.state.selected() {
            None => {
                if self.rows.is_empty() {
                    return;
                }
                self.state.select(Some(0));
            }
            Some(selected) => {
                if selected + 1 >= self.rows.len() {
                    return;
                }

                let mut next = selected + 1;
                {
                    if self.grapheme_end_row_indices.contains(&selected) {
                        next += 1;
                    }
                }
                self.state.select(Some(next));
            }
        };
    }

    pub fn select_next_n(&mut self, n: usize) {
        for _ in 0..n {
            self.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        match self.state.selected() {
            None => {
                if self.rows.is_empty() {
                    return;
                }
                self.state.select(Some(self.rows.len() - 1));
            }
            Some(selected) => {
                if selected == 0 {
                    return;
                }

                let mut previous = selected - 1;
                {
                    if self.grapheme_start_row_indices.contains(&selected) {
                        previous -= 1;
                    }
                }
                self.state.select(Some(previous));
            }
        }
    }

    pub fn select_previous_n(&mut self, n: usize) {
        for _ in 0..n {
            self.select_previous();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "ಠ_ರೃ ab";

    #[test]
    fn test_new_stateful_graphemes() {
        let graphemes = StatefulGraphemes::new(TEST_STR);

        assert_eq!(graphemes.rows.len(), 12);

        {
            let mut expected_grapheme_start_row_indices = HashSet::new();
            expected_grapheme_start_row_indices.insert(0);
            expected_grapheme_start_row_indices.insert(2);
            expected_grapheme_start_row_indices.insert(4);
            expected_grapheme_start_row_indices.insert(7);
            expected_grapheme_start_row_indices.insert(9);
            expected_grapheme_start_row_indices.insert(11);
            assert_eq!(
                graphemes.grapheme_start_row_indices,
                expected_grapheme_start_row_indices
            );
        }

        {
            let mut expected_grapheme_end_row_indices = HashSet::new();
            expected_grapheme_end_row_indices.insert(0);
            expected_grapheme_end_row_indices.insert(2);
            expected_grapheme_end_row_indices.insert(5);
            expected_grapheme_end_row_indices.insert(7);
            expected_grapheme_end_row_indices.insert(9);
            expected_grapheme_end_row_indices.insert(11);
            assert_eq!(
                graphemes.grapheme_end_row_indices,
                expected_grapheme_end_row_indices
            );
        }

        assert!(graphemes.state.selected().is_some());
        assert_eq!(graphemes.state.selected().unwrap(), 0);
    }

    #[test]
    fn test_select_next() {
        let mut graphemes = StatefulGraphemes::new(TEST_STR);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 2);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 4);
        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 5);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 7);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 9);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 11);

        graphemes.select_next();
        assert_eq!(graphemes.state.selected().unwrap(), 11);
    }

    #[test]
    fn test_select_next_n() {
        let mut graphemes = StatefulGraphemes::new(TEST_STR);

        graphemes.select_next_n(2);
        assert_eq!(graphemes.state.selected().unwrap(), 4);

        graphemes.select_next_n(42);
        assert_eq!(graphemes.state.selected().unwrap(), 11);
    }

    #[test]
    fn test_select_previous() {
        let mut graphemes = StatefulGraphemes::new(TEST_STR);
        graphemes.state.select(Some(11));

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 9);

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 7);

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 5);
        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 4);

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 2);

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 0);

        graphemes.select_previous();
        assert_eq!(graphemes.state.selected().unwrap(), 0);
    }

    #[test]
    fn test_select_previous_n() {
        let mut graphemes = StatefulGraphemes::new(TEST_STR);
        graphemes.state.select(Some(11));

        graphemes.select_previous_n(3);
        assert_eq!(graphemes.state.selected().unwrap(), 5);

        graphemes.select_previous_n(42);
        assert_eq!(graphemes.state.selected().unwrap(), 0);
    }
}

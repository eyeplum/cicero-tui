use std::collections::HashSet;

use tui::widgets::ListState;
use unic::segment::Graphemes;
use unic::ucd::name::Name;

#[derive(Default)]
pub struct StatefulGraphemes {
    pub state: ListState,
    pub rows: Vec<String>,

    grapheme_start_row_indices: HashSet<usize>,
    grapheme_end_row_indices: HashSet<usize>,
}

impl StatefulGraphemes {
    pub fn new(s: &str) -> Self {
        let graphemes: Vec<Vec<char>> = Graphemes::new(s)
            .map(|grapheme| grapheme.chars().map(|chr| chr).collect())
            .collect();

        let mut state = ListState::default();
        if graphemes.len() > 0 {
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

                rows.push(StatefulGraphemes::row_for(*chr));
            }

            if i + 1 < graphemes.len() {
                rows.push("".to_owned());
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

    const CODE_POINT_STR_PADDING: usize = 8;

    fn row_for(chr: char) -> String {
        let mut code_point_str = format!("U+{:04X}", chr as u32);
        if code_point_str.len() < StatefulGraphemes::CODE_POINT_STR_PADDING {
            code_point_str = format!("{}{}", " ".repeat(8 - code_point_str.len()), code_point_str);
        }

        let name = match Name::of(chr) {
            None => "".to_owned(),
            Some(name) => name.to_string(),
        };

        format!("{}  {}  {}", code_point_str, chr, name)
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
}

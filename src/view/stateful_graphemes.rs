use tui::widgets::ListState;
use unic::segment::Graphemes;
use unic::ucd::name::Name;

type GraphemeCluster = Vec<char>;

#[derive(Default)]
pub struct StatefulGraphemes {
    pub state: ListState,
    pub rows: Vec<String>,

    graphemes: Vec<GraphemeCluster>,
}

impl StatefulGraphemes {
    pub fn new(s: &str) -> Self {
        let graphemes: Vec<GraphemeCluster> = Graphemes::new(s)
            .map(|grapheme| grapheme.chars().map(|chr| chr).collect())
            .collect();

        let mut state = ListState::default();
        if graphemes.len() > 0 {
            state.select(Some(0));
        }

        let mut rows = vec![];
        for (i, grapheme) in graphemes.iter().enumerate() {
            for chr in grapheme {
                let mut code_point_str = format!("U+{:04X}", *chr as u32);
                if code_point_str.len() < 8 {
                    code_point_str =
                        format!("{}{}", " ".repeat(8 - code_point_str.len()), code_point_str);
                }
                let name = match Name::of(*chr) {
                    None => "".to_owned(),
                    Some(name) => name.to_string(),
                };

                rows.push(format!("{}  {}  {}", code_point_str, chr, name));
            }

            if i + 1 < graphemes.len() {
                rows.push("".to_owned());
            }
        }

        StatefulGraphemes {
            state,
            rows,
            graphemes,
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
                    let mut end_of_graphemes = vec![];
                    end_of_graphemes.reserve(self.graphemes.len());
                    for (i, grapheme) in self.graphemes.iter().enumerate() {
                        if i == 0 {
                            end_of_graphemes.push(grapheme.len() - 1);
                        } else {
                            end_of_graphemes
                                .push(end_of_graphemes.last().unwrap() + 1 + grapheme.len());
                        }
                    }

                    if end_of_graphemes.contains(&selected) {
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
                    let mut start_of_graphemes = vec![];
                    start_of_graphemes.reserve(self.graphemes.len());
                    for (i, _grapheme) in self.graphemes.iter().enumerate() {
                        if i == 0 {
                            start_of_graphemes.push(0);
                        } else {
                            start_of_graphemes.push(
                                start_of_graphemes.last().unwrap()
                                    + &self.graphemes[i - 1].len()
                                    + 1,
                            );
                        }
                    }

                    if start_of_graphemes.contains(&selected) {
                        previous -= 1;
                    }
                }
                self.state.select(Some(previous));
            }
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

        assert_eq!(graphemes.graphemes.len(), 6);
        assert_eq!(graphemes.rows.len(), 12);

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

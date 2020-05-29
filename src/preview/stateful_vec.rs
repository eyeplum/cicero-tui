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

#[derive(Debug)]
pub struct StatefulVec<T> {
    storage: Vec<T>,
    current: Option<usize>,
}

impl<T> StatefulVec<T> {
    pub fn new(storage: Vec<T>, current: Option<usize>) -> Self {
        let current = match current {
            Some(current) => {
                if current + 1 > storage.len() {
                    None
                } else {
                    Some(current)
                }
            }
            None => None,
        };

        StatefulVec { storage, current }
    }

    pub fn has_previous(&self) -> bool {
        match self.current {
            Some(current) => current > 0,
            None => !self.storage.is_empty(),
        }
    }

    pub fn select_previous(&mut self) {
        if !self.has_previous() {
            return;
        }
        assert!(!self.storage.is_empty());

        self.current = match self.current {
            Some(current) => Some(current - 1),
            None => Some(self.storage.len() - 1),
        }
    }

    pub fn has_next(&self) -> bool {
        match self.current {
            Some(current) => current + 1 < self.storage.len(),
            None => !self.storage.is_empty(),
        }
    }

    pub fn select_next(&mut self) {
        if !self.has_next() {
            return;
        }
        assert!(!self.storage.is_empty());

        self.current = match self.current {
            Some(current) => Some(current + 1),
            None => Some(0),
        }
    }

    pub fn current_item(&self) -> Option<&T> {
        match self.current {
            Some(current) => Some(&self.storage[current]),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        {
            let stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            assert!(stateful_vec.storage.is_empty());
            assert!(stateful_vec.current.is_none());
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(2));
            assert_eq!(stateful_vec.storage, vec![1, 2, 3, 4, 5]);
            assert_eq!(stateful_vec.current, Some(2));
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(100));
            assert_eq!(stateful_vec.storage, vec![1, 2, 3, 4, 5]);
            assert_eq!(stateful_vec.current, None);
        }
    }

    #[test]
    fn test_has_previous() {
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(1));
            assert!(stateful_vec.has_previous());
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(0));
            assert!(!stateful_vec.has_previous());
        }
        {
            let stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            assert!(!stateful_vec.has_previous());
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            assert!(stateful_vec.has_previous());
        }
    }

    #[test]
    fn test_previous() {
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(1));
            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current, Some(0));
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(0));
            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current, Some(0));
        }
        {
            let mut stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current, None);
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current, Some(4));
        }
    }

    #[test]
    fn test_has_next() {
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(1));
            assert!(stateful_vec.has_next());
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(4));
            assert!(!stateful_vec.has_next());
        }
        {
            let stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            assert!(!stateful_vec.has_next());
        }
        {
            let stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            assert!(stateful_vec.has_next());
        }
    }

    #[test]
    fn test_next() {
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(1));
            stateful_vec.select_next();
            assert_eq!(stateful_vec.current, Some(2));
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], Some(4));
            stateful_vec.select_next();
            assert_eq!(stateful_vec.current, Some(4));
        }
        {
            let mut stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            stateful_vec.select_next();
            assert_eq!(stateful_vec.current, None);
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            stateful_vec.select_next();
            assert_eq!(stateful_vec.current, Some(0));
        }
    }

    #[test]
    fn test_current_item() {
        {
            let stateful_vec = StatefulVec::new(Vec::<u8>::default(), None);
            assert_eq!(stateful_vec.current_item(), None);
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            assert_eq!(stateful_vec.current_item(), None);

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&5));

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&4));

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&3));

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&2));

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&1));

            stateful_vec.select_previous();
            assert_eq!(stateful_vec.current_item(), Some(&1));
        }
        {
            let mut stateful_vec = StatefulVec::new(vec![1, 2, 3, 4, 5], None);
            assert_eq!(stateful_vec.current_item(), None);

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&1));

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&2));

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&3));

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&4));

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&5));

            stateful_vec.select_next();
            assert_eq!(stateful_vec.current_item(), Some(&5));
        }
    }
}

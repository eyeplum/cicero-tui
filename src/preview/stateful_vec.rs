#[derive(Debug)]
pub struct StatefulVec<T> {
    storage: Vec<T>,
    current: Option<usize>,
}

impl<T> StatefulVec<T> {
    pub fn new(storage: Vec<T>, current: usize) -> Self {
        if current + 1 > storage.len() {
            StatefulVec {
                storage,
                current: None,
            }
        } else {
            StatefulVec {
                storage,
                current: Some(current),
            }
        }
    }

    pub fn has_previous(&self) -> bool {
        match self.current {
            Some(current) => current > 0,
            None => self.storage.is_empty(),
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
            None => self.storage.is_empty(),
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
    // TODO: Unit tests
}

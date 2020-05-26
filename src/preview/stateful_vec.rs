const STATEFUL_VEC_CURRENT_INVALID: usize = usize::max_value();

#[derive(Debug)]
pub struct StatefulVec<T> {
    storage: Vec<T>,
    current: usize,
}

impl<T> StatefulVec<T> {
    pub fn new(storage: Vec<T>, current: usize) -> Self {
        if current + 1 > storage.len() {
            StatefulVec {
                storage,
                current: STATEFUL_VEC_CURRENT_INVALID,
            }
        } else {
            StatefulVec { storage, current }
        }
    }

    pub fn has_previous(&self) -> bool {
        !self.storage.is_empty() && self.current != STATEFUL_VEC_CURRENT_INVALID && self.current > 0
    }

    pub fn select_previous(&mut self) {
        if !self.has_previous() {
            return;
        }

        self.current -= 1;
    }

    pub fn has_next(&self) -> bool {
        !self.storage.is_empty()
            && self.current != STATEFUL_VEC_CURRENT_INVALID
            && (self.current + 1 < self.storage.len())
    }

    pub fn select_next(&mut self) {
        if !self.has_next() {
            return;
        }

        self.current += 1;
    }

    pub fn current_item(&self) -> &T {
        &self.storage[self.current]
    }
}

// TODO: Unit tests

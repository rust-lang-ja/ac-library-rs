#[derive(Default)]
pub(crate) struct SimpleQueue<T> {
    payload: Vec<T>,
    pos: usize,
}

impl<T> SimpleQueue<T> {
    pub(crate) fn reserve(&mut self, n: usize) {
        if n > self.payload.len() {
            self.payload.reserve(n - self.payload.len());
        }
    }

    pub(crate) fn size(&self) -> usize {
        self.payload.len() - self.pos
    }

    pub(crate) fn empty(&self) -> bool {
        self.pos == self.payload.len()
    }

    pub(crate) fn push(&mut self, t: T) {
        self.payload.push(t);
    }

    // Do we need mutable version?
    pub(crate) fn front(&self) -> &T {
        &self.payload[self.pos]
    }

    pub(crate) fn clear(&mut self) {
        self.payload.clear();
        self.pos = 0;
    }

    pub(crate) fn pop(&mut self) -> Option<&T> {
        if self.pos < self.payload.len() {
            self.pos += 1;
            Some(&self.payload[self.pos - 1])
        } else {
            None
        }
    }
}

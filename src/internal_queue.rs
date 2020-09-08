struct SimpleQueue<T> {
    payload: Vec<T>,
    pos: usize,
}

impl<T> SimpleQueue<T> {
    fn reserve(&mut self, n: usize) {
        if n > self.payload.len() {
            self.payload.reserve(n - self.payload.len());
        }
    }

    fn size(&self) -> usize {
        self.payload.len() - self.pos
    }

    fn empty(&self) -> bool {
        self.pos == self.payload.len()
    }

    fn push(&mut self, t: T) {
        self.payload.push(t);
    }

    // Do we need mutable version?
    fn front(&self) -> &T {
        &self.payload[self.pos]
    }

    fn clear(&mut self) {
        self.payload.clear();
        self.pos = 0;
    }

    fn pop(&mut self) {
        self.pos += 1;
    }
}

struct SimpleQueue<T> {
    payload: Vec<T>,
    pos: usize,
}

impl<T> SimpleQueue<T>
where
    T: Copy,
{
    fn reserve(&mut self, n: i32) {
        let n = n as usize;
        if n > self.payload.len() {
            self.payload.reserve(n - self.payload.len());
        }
    }

    fn size(&self) -> i32 {
        (self.payload.len() - self.pos) as i32
    }

    fn empty(&self) -> bool {
        self.pos == self.payload.len()
    }

    fn push(&mut self, t: &T) {
        self.payload.push(*t);
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

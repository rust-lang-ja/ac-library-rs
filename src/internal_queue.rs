#![allow(dead_code)]

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
    pub(crate) fn front(&self) -> Option<&T> {
        if self.pos < self.payload.len() {
            Some(&self.payload[self.pos])
        } else {
            None
        }
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

#[cfg(test)]
mod test {
    use super::SimpleQueue;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_simple_queue() {
        let mut queue = SimpleQueue::default();

        assert_eq!(queue.size(), 0);
        assert!(queue.empty());
        assert!(queue.front().is_none());
        assert!(queue.pop().is_none());

        queue.push(123);

        assert_eq!(queue.size(), 1);
        assert!(!queue.empty());
        assert_eq!(queue.front(), Some(&123));

        queue.push(456);

        assert_eq!(queue.size(), 2);
        assert!(!queue.empty());
        assert_eq!(queue.front(), Some(&123));

        assert_eq!(queue.pop(), Some(&123));
        assert_eq!(queue.size(), 1);
        assert!(!queue.empty());
        assert_eq!(queue.front(), Some(&456));

        queue.push(789);
        queue.push(789);
        queue.push(456);
        queue.push(456);

        assert_eq!(queue.size(), 5);
        assert!(!queue.empty());
        assert_eq!(queue.front(), Some(&456));

        assert_eq!(queue.pop(), Some(&456));
        assert_eq!(queue.size(), 4);
        assert!(!queue.empty());
        assert_eq!(queue.front(), Some(&789));

        queue.clear();

        assert_eq!(queue.size(), 0);
        assert!(queue.empty());
        assert!(queue.front().is_none());
        assert!(queue.pop().is_none());
    }
}

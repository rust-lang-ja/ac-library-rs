// Reference: https://en.wikipedia.org/wiki/Fenwick_tree
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
        FenwickTree {
            n: n,
            ary: vec![e.clone(); n],
            e: e,
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(r) - self.accum(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fenwick_tree_works() {
        let mut bit = FenwickTree::new(5, 0i64);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0, 5), 15);
        assert_eq!(bit.sum(0, 4), 10);
        assert_eq!(bit.sum(1, 3), 5);
    }
}

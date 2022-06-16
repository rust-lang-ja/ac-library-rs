use std::iter::repeat_with;

use crate::num_traits::Zero;

// Reference: https://en.wikipedia.org/wiki/Fenwick_tree
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
}

impl<T: for<'a> std::ops::AddAssign<&'a T> + Zero> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            ary: repeat_with(T::zero).take(n).collect(),
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = T::zero();
        while idx > 0 {
            sum += &self.ary[idx - 1];
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add<U>(&mut self, mut idx: usize, val: U)
    where
        T: for<'a> std::ops::AddAssign<&'a U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += &val;
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
        let mut bit = FenwickTree::<i64>::new(5);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0, 5), 15);
        assert_eq!(bit.sum(0, 4), 10);
        assert_eq!(bit.sum(1, 3), 5);
    }
}

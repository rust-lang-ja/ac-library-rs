use std::{
    iter::FromIterator,
    ops::{AddAssign, Bound, RangeBounds},
};

use crate::num_traits::Zero;

// Reference: https://en.wikipedia.org/wiki/Fenwick_tree
#[derive(Clone)]
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
}

impl<T: Clone + AddAssign<T> + Zero> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            ary: vec![T::zero(); n],
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = T::zero();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum<R>(&self, range: R) -> T
    where
        T: std::ops::Sub<Output = T>,
        R: RangeBounds<usize>,
    {
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => return self.accum(r),
        };
        self.accum(r) - self.accum(l)
    }
}
impl<T: Clone + AddAssign<T>> From<Vec<T>> for FenwickTree<T> {
    fn from(mut ary: Vec<T>) -> Self {
        for i in 1..=ary.len() {
            let j = i + (i & i.wrapping_neg());
            if j <= ary.len() {
                let add = ary[i - 1].clone();
                ary[j - 1] += add;
            }
        }
        Self { n: ary.len(), ary }
    }
}
impl<T: Clone + AddAssign<T>> FromIterator<T> for FenwickTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound::*;

    #[test]
    fn fenwick_tree_works() {
        let mut bit = FenwickTree::<i64>::new(5);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0..5), 15);
        assert_eq!(bit.sum(0..4), 10);
        assert_eq!(bit.sum(1..3), 5);

        assert_eq!(bit.sum(..), 15);
        assert_eq!(bit.sum(..2), 3);
        assert_eq!(bit.sum(..=2), 6);
        assert_eq!(bit.sum(1..), 14);
        assert_eq!(bit.sum(1..=3), 9);
        assert_eq!(bit.sum((Excluded(0), Included(2))), 5);
    }

    #[test]
    fn from_iter_works() {
        let tree = FenwickTree::from_iter(vec![1, 2, 3, 4, 5].iter().map(|x| x * 2));
        let internal = vec![2, 4, 6, 8, 10];
        for j in 0..=internal.len() {
            for i in 0..=j {
                assert_eq!(tree.sum(i..j), internal[i..j].iter().sum::<i32>());
            }
        }
    }
}

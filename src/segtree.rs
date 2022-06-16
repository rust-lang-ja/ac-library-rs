use crate::internal_bit::ceil_pow2;
use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
use std::cmp::{max, min, Ordering};
use std::convert::Infallible;
use std::iter::{empty, repeat_with, FromIterator};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

// TODO Should I split monoid-related traits to another module?
pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Max<S>
where
    S: Copy + Ord + BoundedBelow,
{
    type S = S;
    fn identity() -> Self::S {
        S::min_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        max(*a, *b)
    }
}

pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Min<S>
where
    S: Copy + Ord + BoundedAbove,
{
    type S = S;
    fn identity() -> Self::S {
        S::max_value()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        min(*a, *b)
    }
}

pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Additive<S>
where
    S: Copy + Add<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for Multiplicative<S>
where
    S: Copy + Mul<Output = S> + One,
{
    type S = S;
    fn identity() -> Self::S {
        S::one()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a * *b
    }
}

impl<M: Monoid> Default for Segtree<M> {
    fn default() -> Self {
        Segtree::new(0)
    }
}
impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Segtree<M> {
        vec![M::identity(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(v: Vec<M::S>) -> Self {
        Self::from_vec(v, 0)
    }
}
impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let n = iter.size_hint().0;
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = Vec::with_capacity(size * 2);
        d.extend(repeat_with(M::identity).take(size).chain(iter));
        Self::from_vec(d, size)
    }
}
impl<M: Monoid> Segtree<M> {
    /// Creates a segtree from elements `d[offset..]`.
    fn from_vec(mut d: Vec<M::S>, offset: usize) -> Self {
        assert!(offset <= d.len());
        let n = d.len() - offset;
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        match offset.cmp(&size) {
            Ordering::Less => {
                d.splice(0..0, repeat_with(M::identity).take(size - offset));
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                d.splice(size..offset, empty());
            }
        };
        d.resize_with(size * 2, M::identity);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        // `ret.d[0]` is uninitialized and has an unknown value.
        // This is ok as it is unused (as of writing).
        ret
    }

    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> M::S {
        assert!(p < self.n);
        self.d[p + self.size].clone()
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
        assert!(l <= r && r <= self.n);
        let mut sml = M::identity();
        let mut smr = M::identity();
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> M::S {
        self.d[1].clone()
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, &self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(&sm, &self.d[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(&sm, &self.d[l]);
            l += 1;
            // while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(&self.d[r], &sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(&self.d[r], &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(&self.d[r], &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
    }
}

// Maybe we can use this someday
// ```
// for i in 0..=self.log {
//     for j in 0..1 << i {
//         print!("{}\t", self.d[(1 << i) + j]);
//     }
//     println!();
// }
// ```

pub struct Segtree<M>
where
    M: Monoid,
{
    // variable name is _n in original library
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::S>,
}

#[cfg(test)]
mod tests {
    use crate::segtree::{Additive, Max};
    use crate::Segtree;

    #[test]
    fn test_max_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let segtree: Segtree<Max<_>> = base.clone().into();
        check_segtree(&base, &segtree);

        let mut segtree = Segtree::<Max<_>>::new(n);
        let mut internal = vec![i32::min_value(); n];
        for i in 0..n {
            segtree.set(i, base[i]);
            internal[i] = base[i];
            check_segtree(&internal, &segtree);
        }

        segtree.set(6, 5);
        internal[6] = 5;
        check_segtree(&internal, &segtree);

        segtree.set(6, 0);
        internal[6] = 0;
        check_segtree(&internal, &segtree);
    }

    #[test]
    fn test_from_iter() {
        let it = || (1..7).map(|x| x * 4 % 11);
        let base = it().collect::<Vec<_>>();
        let segtree: Segtree<Max<_>> = it().collect();
        check_segtree(&base, &segtree);
    }

    //noinspection DuplicatedCode
    fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }
        for i in 0..=n {
            for j in i..=n {
                assert_eq!(
                    segtree.prod(i, j),
                    base[i..j].iter().max().copied().unwrap_or(i32::min_value())
                );
            }
        }
        assert_eq!(
            segtree.all_prod(),
            base.iter().max().copied().unwrap_or(i32::min_value())
        );
        for k in 0..=10 {
            let f = |&x: &i32| x < k;
            for i in 0..=n {
                assert_eq!(
                    Some(segtree.max_right(i, f)),
                    (i..=n)
                        .filter(|&j| f(&base[i..j]
                            .iter()
                            .max()
                            .copied()
                            .unwrap_or(i32::min_value())))
                        .max()
                );
            }
            for j in 0..=n {
                assert_eq!(
                    Some(segtree.min_left(j, f)),
                    (0..=j)
                        .filter(|&i| f(&base[i..j]
                            .iter()
                            .max()
                            .copied()
                            .unwrap_or(i32::min_value())))
                        .min()
                );
            }
        }
    }

    #[test]
    fn test_from_vec() {
        let v = vec![1, 2, 4];
        let ans_124 = vec![7, 3, 4, 1, 2, 4, 0];
        let tree = Segtree::<Additive<_>>::from_vec(v, 0);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![1, 2, 4, 8];
        let tree = Segtree::<Additive<_>>::from_vec(v, 0);
        assert_eq!(&tree.d[1..], &vec![15, 3, 12, 1, 2, 4, 8][..]);

        let v = vec![1, 2, 4, 8, 16];
        let tree = Segtree::<Additive<_>>::from_vec(v, 0);
        assert_eq!(
            &tree.d[1..],
            &vec![31, 15, 16, 3, 12, 16, 0, 1, 2, 4, 8, 16, 0, 0, 0][..]
        );

        let v = vec![314, 159, 265, 1, 2, 4];
        let tree = Segtree::<Additive<_>>::from_vec(v, 3);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![314, 159, 265, 897, 1, 2, 4];
        let tree = Segtree::<Additive<_>>::from_vec(v, 4);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![314, 159, 265, 897, 932, 1, 2, 4];
        let tree = Segtree::<Additive<_>>::from_vec(v, 5);
        assert_eq!(&tree.d[1..], &ans_124[..]);
    }
}

use crate::internal_bit::ceil_pow2;
use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
use std::cmp::{max, min};
use std::convert::Infallible;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Add, BitAnd, BitOr, BitXor, Bound, Mul, Not, RangeBounds};

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

pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseOr<S>
where
    S: Copy + BitOr<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a | *b
    }
}

pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseAnd<S>
where
    S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        !S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a & *b
    }
}

pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseXor<S>
where
    S: Copy + BitXor<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
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
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..][..n].clone_from_slice(&v);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        v.into()
    }
}
impl<M: Monoid> Segtree<M> {
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

    pub fn get_slice(&self) -> &[M::S] {
        &self.d[self.size..][..self.n]
    }

    pub fn prod<R>(&self, range: R) -> M::S
    where
        R: RangeBounds<usize>,
    {
        // Trivial optimization
        if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
            return self.all_prod();
        }

        let mut r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let mut l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            // TODO: There are another way of optimizing [0..r)
            Bound::Unbounded => 0,
        };

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

#[derive(Clone)]
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
    use crate::segtree::Max;
    use crate::Segtree;
    use std::ops::{Bound::*, RangeBounds};

    #[test]
    fn test_max_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let segtree: Segtree<Max<_>> = base.clone().into();
        check_segtree(&base, &segtree);

        let mut segtree = Segtree::<Max<_>>::new(n);
        let mut internal = vec![i32::MIN; n];
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
    fn test_segtree_fromiter() {
        let v = [1, 4, 1, 4, 2, 1, 3, 5, 6];
        let base = v
            .iter()
            .copied()
            .filter(|&x| x % 2 == 0)
            .collect::<Vec<_>>();
        let segtree: Segtree<Max<_>> = v.iter().copied().filter(|&x| x % 2 == 0).collect();
        check_segtree(&base, &segtree);
    }

    //noinspection DuplicatedCode
    fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }

        check(base, segtree, ..);
        for i in 0..=n {
            check(base, segtree, ..i);
            check(base, segtree, i..);
            if i < n {
                check(base, segtree, ..=i);
            }
            for j in i..=n {
                check(base, segtree, i..j);
                if j < n {
                    check(base, segtree, i..=j);
                    check(base, segtree, (Excluded(i), Included(j)));
                }
            }
        }
        assert_eq!(
            segtree.all_prod(),
            base.iter().max().copied().unwrap_or(i32::MAX)
        );
        for k in 0..=10 {
            let f = |&x: &i32| x < k;
            for i in 0..=n {
                assert_eq!(
                    Some(segtree.max_right(i, f)),
                    (i..=n)
                        .filter(|&j| f(&base[i..j].iter().max().copied().unwrap_or(i32::MIN)))
                        .max()
                );
            }
            for j in 0..=n {
                assert_eq!(
                    Some(segtree.min_left(j, f)),
                    (0..=j)
                        .filter(|&i| f(&base[i..j].iter().max().copied().unwrap_or(i32::MIN)))
                        .min()
                );
            }
        }
    }

    fn check(base: &[i32], segtree: &Segtree<Max<i32>>, range: impl RangeBounds<usize>) {
        let expected = base
            .iter()
            .enumerate()
            .filter_map(|(i, a)| Some(a).filter(|_| range.contains(&i)))
            .max()
            .copied()
            .unwrap_or(i32::MIN);
        assert_eq!(segtree.prod(range), expected);
    }
}

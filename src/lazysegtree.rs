use crate::internal_bit::ceil_pow2;
use crate::Monoid;

pub trait MapMonoid {
    type M: Monoid;
    type F: Clone;
    // type S = <Self::M as Monoid>::S;
    fn identity_element() -> <Self::M as Monoid>::S {
        Self::M::identity()
    }
    fn binary_operation(
        a: &<Self::M as Monoid>::S,
        b: &<Self::M as Monoid>::S,
    ) -> <Self::M as Monoid>::S {
        Self::M::binary_operation(a, b)
    }
    fn identity_map() -> Self::F;
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

impl<F: MapMonoid> Default for LazySegtree<F> {
    fn default() -> Self {
        Self::new(0)
    }
}
impl<F: MapMonoid> LazySegtree<F> {
    pub fn new(n: usize) -> Self {
        vec![F::identity_element(); n].into()
    }
}
impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegtree<F> {
    fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
        Self::from_vec(v, 0)
    }
}
impl<F: MapMonoid> FromIterator<<F::M as Monoid>::S> for LazySegtree<F> {
    fn from_iter<T: IntoIterator<Item = <F::M as Monoid>::S>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let n = iter.size_hint().0;
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = Vec::with_capacity(size * 2);
        d.extend(repeat_with(F::identity_element).take(size).chain(iter));
        Self::from_vec(d, size)
    }
}

impl<F: MapMonoid> LazySegtree<F> {
    /// Creates a segtree from elements `d[offset..]`.
    fn from_vec(mut d: Vec<<F::M as Monoid>::S>, offset: usize) -> Self {
        assert!(offset <= d.len());
        let n = d.len() - offset;
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        match offset.cmp(&size) {
            Ordering::Less => {
                d.splice(0..0, repeat_with(F::identity_element).take(size - offset));
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                d.splice(size..offset, empty());
            }
        };
        d.resize_with(size * 2, F::identity_element);
        let lz = vec![F::identity_map(); size];
        let mut ret = LazySegtree {
            n,
            size,
            log,
            d,
            lz,
        };
        for i in (1..size).rev() {
            ret.update(i);
        }
        // `ret.d[0]` is uninitialized and has an unknown value.
        // This is ok as it is unused (as of writing).
        ret
    }

    pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p].clone()
    }

    pub fn prod<R>(&mut self, range: R) -> <F::M as Monoid>::S
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
        if l == r {
            return F::identity_element();
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }

        let mut sml = F::identity_element();
        let mut smr = F::identity_element();
        while l < r {
            if l & 1 != 0 {
                sml = F::binary_operation(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = F::binary_operation(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        F::binary_operation(&sml, &smr)
    }

    pub fn all_prod(&self) -> <F::M as Monoid>::S {
        self.d[1].clone()
    }

    pub fn apply(&mut self, mut p: usize, f: F::F) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = F::mapping(&f, &self.d[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
    pub fn apply_range<R>(&mut self, range: R, f: F::F)
    where
        R: RangeBounds<usize>,
    {
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
        if l == r {
            return;
        }

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        {
            let l2 = l;
            let r2 = r;
            while l < r {
                if l & 1 != 0 {
                    self.all_apply(l, f.clone());
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.all_apply(r, f.clone());
                }
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(<F::M as Monoid>::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(g(F::identity_element()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = F::identity_element();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(F::binary_operation(&sm, &self.d[l])) {
                while l < self.size {
                    self.push(l);
                    l *= 2;
                    let res = F::binary_operation(&sm, &self.d[l]);
                    if g(res.clone()) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = F::binary_operation(&sm, &self.d[l]);
            l += 1;
            //while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where
        G: Fn(<F::M as Monoid>::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(g(F::identity_element()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = F::identity_element();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !g(F::binary_operation(&self.d[r], &sm)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::binary_operation(&self.d[r], &sm);
                    if g(res.clone()) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = F::binary_operation(&self.d[r], &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }
}

pub struct LazySegtree<F>
where
    F: MapMonoid,
{
    n: usize,
    size: usize,
    log: usize,
    d: Vec<<F::M as Monoid>::S>,
    lz: Vec<F::F>,
}
impl<F> LazySegtree<F>
where
    F: MapMonoid,
{
    fn update(&mut self, k: usize) {
        self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
    }
    fn all_apply(&mut self, k: usize, f: F::F) {
        self.d[k] = F::mapping(&f, &self.d[k]);
        if k < self.size {
            self.lz[k] = F::composition(&f, &self.lz[k]);
        }
    }
    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k].clone());
        self.all_apply(2 * k + 1, self.lz[k].clone());
        self.lz[k] = F::identity_map();
    }
}

// TODO is it useful?
use std::{
    cmp::Ordering,
    fmt::{Debug, Error, Formatter, Write},
    iter::{empty, repeat_with, FromIterator},
    ops::{Bound, RangeBounds},
};
impl<F> Debug for LazySegtree<F>
where
    F: MapMonoid,
    F::F: Debug,
    <F::M as Monoid>::S: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.log {
            for j in 0..1 << i {
                f.write_fmt(format_args!(
                    "{:?}[{:?}]\t",
                    self.d[(1 << i) + j],
                    self.lz[(1 << i) + j]
                ))?;
            }
            f.write_char('\n')?;
        }
        for i in 0..self.size {
            f.write_fmt(format_args!("{:?}\t", self.d[self.size + i]))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Bound::*, RangeBounds};

    use crate::{Additive, LazySegtree, MapMonoid, Max};

    struct MaxAdd;
    impl MapMonoid for MaxAdd {
        type M = Max<i32>;
        type F = i32;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &i32, &x: &i32) -> i32 {
            f + x
        }

        fn composition(&f: &i32, &g: &i32) -> i32 {
            f + g
        }
    }

    #[test]
    fn test_max_add_lazy_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let mut segtree: LazySegtree<MaxAdd> = base.clone().into();
        check_segtree(&base, &mut segtree);

        let mut segtree = LazySegtree::<MaxAdd>::new(n);
        let mut internal = vec![i32::min_value(); n];
        for i in 0..n {
            segtree.set(i, base[i]);
            internal[i] = base[i];
            check_segtree(&internal, &mut segtree);
        }

        segtree.set(6, 5);
        internal[6] = 5;
        check_segtree(&internal, &mut segtree);

        segtree.apply(5, 1);
        internal[5] += 1;
        check_segtree(&internal, &mut segtree);

        segtree.set(6, 0);
        internal[6] = 0;
        check_segtree(&internal, &mut segtree);

        segtree.apply_range(3..8, 2);
        internal[3..8].iter_mut().for_each(|e| *e += 2);
        check_segtree(&internal, &mut segtree);

        segtree.apply_range(2..=5, 7);
        internal[2..=5].iter_mut().for_each(|e| *e += 7);
        check_segtree(&internal, &mut segtree);
    }

    #[test]
    fn test_from_iter() {
        let it = || (1..7).map(|x| x * 4 % 11);
        let base = it().collect::<Vec<_>>();
        let mut segtree: LazySegtree<MaxAdd> = it().collect();
        check_segtree(&base, &mut segtree);
    }

    //noinspection DuplicatedCode
    fn check_segtree(base: &[i32], segtree: &mut LazySegtree<MaxAdd>) {
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
            base.iter().max().copied().unwrap_or(i32::min_value())
        );
        for k in 0..=10 {
            let f = |x| x < k;
            for i in 0..=n {
                assert_eq!(
                    Some(segtree.max_right(i, f)),
                    (i..=n)
                        .filter(|&j| f(base[i..j]
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
                        .filter(|&i| f(base[i..j]
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
        struct IdAdditive;
        impl MapMonoid for IdAdditive {
            type M = Additive<u32>;
            type F = ();
            fn identity_map() {}
            fn mapping(_: &(), &x: &u32) -> u32 {
                x
            }
            fn composition(_: &(), _: &()) {}
        }

        let v = vec![1, 2, 4];
        let ans_124 = vec![7, 3, 4, 1, 2, 4, 0];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 0);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![1, 2, 4, 8];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 0);
        assert_eq!(&tree.d[1..], &vec![15, 3, 12, 1, 2, 4, 8][..]);

        let v = vec![1, 2, 4, 8, 16];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 0);
        assert_eq!(
            &tree.d[1..],
            &vec![31, 15, 16, 3, 12, 16, 0, 1, 2, 4, 8, 16, 0, 0, 0][..]
        );

        let v = vec![314, 159, 265, 1, 2, 4];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 3);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![314, 159, 265, 897, 1, 2, 4];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 4);
        assert_eq!(&tree.d[1..], &ans_124[..]);

        let v = vec![314, 159, 265, 897, 932, 1, 2, 4];
        let tree = LazySegtree::<IdAdditive>::from_vec(v, 5);
        assert_eq!(&tree.d[1..], &ans_124[..]);
    }

    fn check(base: &[i32], segtree: &mut LazySegtree<MaxAdd>, range: impl RangeBounds<usize>) {
        let expected = base
            .iter()
            .enumerate()
            .filter_map(|(i, a)| Some(a).filter(|_| range.contains(&i)))
            .max()
            .copied()
            .unwrap_or(i32::min_value());
        assert_eq!(segtree.prod(range), expected);
    }
}

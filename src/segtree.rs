use crate::internal_bit::ceil_pow2;

pub trait Monoid {
    type S: Copy;
    const IDENTITY: Self::S;
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S;
}

impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Segtree<M> {
        vec![M::IDENTITY; n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![M::IDENTITY; n];
        d[size..(n + size)].clone_from_slice(&v[..n]);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..n).rev() {
            ret.update(i);
        }
        ret
    }
}

impl<M: Monoid> Segtree<M> {
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p <= self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> M::S {
        assert!(p <= self.n);
        self.d[p + self.size]
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
        assert!(l <= r && r <= self.n);
        let mut sml = M::IDENTITY;
        let mut smr = M::IDENTITY;
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(sml, self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(self.d[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(sml, smr)
    }

    pub fn all_prod(&self) -> M::S {
        self.d[1]
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(M::IDENTITY));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::IDENTITY;
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(M::binary_operation(sm, self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(sm, self.d[l]);
                    if f(res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(sm, self.d[l]);
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
        F: Fn(M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(M::IDENTITY));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::IDENTITY;
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(M::binary_operation(self.d[r], sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(self.d[r], sm);
                    if f(res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(self.d[r], sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(self.d[2 * k], self.d[2 * k + 1]);
    }
}

#[derive(Default)]
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

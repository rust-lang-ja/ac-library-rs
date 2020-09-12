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
        a: <Self::M as Monoid>::S,
        b: <Self::M as Monoid>::S,
    ) -> <Self::M as Monoid>::S {
        Self::M::binary_operation(a, b)
    }
    fn identity_map() -> Self::F;
    fn mapping(f: Self::F, x: <Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
    fn composition(f: Self::F, g: Self::F) -> Self::F;
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
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![F::identity_element(); 2 * size];
        let lz = vec![F::identity_map(); size];
        d[size..(size + n)].clone_from_slice(&v);
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
        ret
    }
}

impl<F: MapMonoid> LazySegtree<F> {
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

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> <F::M as Monoid>::S {
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
                sml = F::binary_operation(sml, self.d[l].clone());
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = F::binary_operation(self.d[r].clone(), smr);
            }
            l >>= 1;
            r >>= 1;
        }

        F::binary_operation(sml, smr)
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
        self.d[p] = F::mapping(f, self.d[p].clone());
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F::F) {
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
            if !g(F::binary_operation(sm.clone(), self.d[l].clone())) {
                while l < self.size {
                    self.push(l);
                    l *= 2;
                    let res = F::binary_operation(sm.clone(), self.d[l].clone());
                    if g(res.clone()) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = F::binary_operation(sm, self.d[l].clone());
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
            if !g(F::binary_operation(self.d[r].clone(), sm.clone())) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::binary_operation(self.d[r].clone(), sm.clone());
                    if g(res.clone()) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = F::binary_operation(self.d[r].clone(), sm);
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
        self.d[k] = F::binary_operation(self.d[2 * k].clone(), self.d[2 * k + 1].clone());
    }
    fn all_apply(&mut self, k: usize, f: F::F) {
        self.d[k] = F::mapping(f.clone(), self.d[k].clone());
        if k < self.size {
            self.lz[k] = F::composition(f, self.lz[k].clone());
        }
    }
    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k].clone());
        self.all_apply(2 * k + 1, self.lz[k].clone());
        self.lz[k] = F::identity_map();
    }
}

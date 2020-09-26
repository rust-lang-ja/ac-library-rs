macro_rules! modulus {
    ($($name:ident),*) => {
        $(
            #[derive(Copy, Clone, Eq, PartialEq)]
            enum $name {}

            impl Modulus for $name {
                const VALUE: u32 = $name as _;
                const HINT_VALUE_IS_PRIME: bool = true;

                fn butterfly_cache() -> &'static ::std::thread::LocalKey<::std::cell::RefCell<::std::option::Option<self::modint::ButterflyCache<Self>>>> {
                    thread_local! {
                        static BUTTERFLY_CACHE: ::std::cell::RefCell<::std::option::Option<self::modint::ButterflyCache<$name>>> = ::std::default::Default::default();
                    }
                    &BUTTERFLY_CACHE
                }
            }
        )*
    };
}

use super::{
    internal_bit, internal_math,
    modint::{self, ButterflyCache, Modulus, RemEuclidU32, StaticModInt},
};
use std::{
    cmp,
    convert::{TryFrom, TryInto as _},
    fmt,
};

#[allow(clippy::many_single_char_names)]
pub fn convolution<M>(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>>
where
    M: Modulus,
{
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let (n, m) = (a.len(), b.len());

    if cmp::min(n, m) <= 60 {
        let (n, m, a, b) = if n < m { (m, n, b, a) } else { (n, m, a, b) };
        let mut ans = vec![StaticModInt::new(0); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                ans[i + j] += a[i] * b[j];
            }
        }
        return ans;
    }

    let (mut a, mut b) = (a.to_owned(), b.to_owned());
    let z = 1 << internal_bit::ceil_pow2((n + m - 1) as _);
    a.resize(z, StaticModInt::raw(0));
    butterfly(&mut a);
    b.resize(z, StaticModInt::raw(0));
    butterfly(&mut b);
    for (a, b) in a.iter_mut().zip(&b) {
        *a *= b;
    }
    butterfly_inv(&mut a);
    a.resize(n + m - 1, StaticModInt::raw(0));
    let iz = StaticModInt::new(z).inv();
    for a in &mut a {
        *a *= iz;
    }
    a
}

pub fn convolution_raw<T, M>(a: &[T], b: &[T]) -> Vec<T>
where
    T: RemEuclidU32 + TryFrom<u32> + Clone,
    T::Error: fmt::Debug,
    M: Modulus,
{
    let a = a.iter().cloned().map(Into::into).collect::<Vec<_>>();
    let b = b.iter().cloned().map(Into::into).collect::<Vec<_>>();
    convolution::<M>(&a, &b)
        .into_iter()
        .map(|z| {
            z.val()
                .try_into()
                .expect("the numeric type is smaller than the modulus")
        })
        .collect()
}

#[allow(clippy::many_single_char_names)]
pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    const M1: u64 = 754_974_721; // 2^24
    const M2: u64 = 167_772_161; // 2^25
    const M3: u64 = 469_762_049; // 2^26
    const M2M3: u64 = M2 * M3;
    const M1M3: u64 = M1 * M3;
    const M1M2: u64 = M1 * M2;
    const M1M2M3: u64 = M1M2.wrapping_mul(M3);

    modulus!(M1, M2, M3);

    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let (_, i1) = internal_math::inv_gcd(M2M3 as _, M1 as _);
    let (_, i2) = internal_math::inv_gcd(M1M3 as _, M2 as _);
    let (_, i3) = internal_math::inv_gcd(M1M2 as _, M3 as _);

    let c1 = convolution_raw::<i64, M1>(a, b);
    let c2 = convolution_raw::<i64, M2>(a, b);
    let c3 = convolution_raw::<i64, M3>(a, b);

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((c1, c2), c3)| {
            const OFFSET: &[u64] = &[0, 0, M1M2M3, 2 * M1M2M3, 3 * M1M2M3];

            let mut x = [(c1, i1, M1, M2M3), (c2, i2, M2, M1M3), (c3, i3, M3, M1M2)]
                .iter()
                .map(|&(c, i, m1, m2)| c.wrapping_mul(i).rem_euclid(m1 as _).wrapping_mul(m2 as _))
                .fold(0, i64::wrapping_add);

            // B = 2^63, -B <= x, r(real value) < B
            // (x, x - M, x - 2M, or x - 3M) = r (mod 2B)
            // r = c1[i] (mod MOD1)
            // focus on MOD1
            // r = x, x - M', x - 2M', x - 3M' (M' = M % 2^64) (mod 2B)
            // r = x,
            //     x - M' + (0 or 2B),
            //     x - 2M' + (0, 2B or 4B),
            //     x - 3M' + (0, 2B, 4B or 6B) (without mod!)
            // (r - x) = 0, (0)
            //           - M' + (0 or 2B), (1)
            //           -2M' + (0 or 2B or 4B), (2)
            //           -3M' + (0 or 2B or 4B or 6B) (3) (mod MOD1)
            // we checked that
            //   ((1) mod MOD1) mod 5 = 2
            //   ((2) mod MOD1) mod 5 = 3
            //   ((3) mod MOD1) mod 5 = 4
            let mut diff = c1 - internal_math::safe_mod(x, M1 as _);
            if diff < 0 {
                diff += M1 as i64;
            }
            x = x.wrapping_sub(OFFSET[diff.rem_euclid(5) as usize] as _);
            x
        })
        .collect()
}

#[allow(clippy::many_single_char_names)]
fn butterfly<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = internal_bit::ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_e, .. } = cache.get_or_insert_with(prepare);
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * now;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                now *= sum_e[(!s).trailing_zeros() as usize];
            }
        }
    });
}

#[allow(clippy::many_single_char_names)]
fn butterfly_inv<M: Modulus>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = internal_bit::ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { sum_ie, .. } = cache.get_or_insert_with(prepare);
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inow = StaticModInt::<M>::new(1);
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = StaticModInt::new(M::VALUE + l.val() - r.val()) * inow;
                }
                inow *= sum_ie[(!s).trailing_zeros() as usize];
            }
        }
    });
}

fn prepare<M: Modulus>() -> ButterflyCache<M> {
    let g = StaticModInt::<M>::raw(internal_math::primitive_root(M::VALUE as i32) as u32);
    let mut es = [StaticModInt::<M>::raw(0); 30]; // es[i]^(2^(2+i)) == 1
    let mut ies = [StaticModInt::<M>::raw(0); 30];
    let cnt2 = (M::VALUE - 1).trailing_zeros() as usize;
    let mut e = g.pow(((M::VALUE - 1) >> cnt2).into());
    let mut ie = e.inv();
    for i in (2..=cnt2).rev() {
        es[i - 2] = e;
        ies[i - 2] = ie;
        e *= e;
        ie *= ie;
    }
    let sum_e = es
        .iter()
        .scan(StaticModInt::new(1), |acc, e| {
            *acc *= e;
            Some(*acc)
        })
        .collect();
    let sum_ie = ies
        .iter()
        .scan(StaticModInt::new(1), |acc, ie| {
            *acc *= ie;
            Some(*acc)
        })
        .collect();
    ButterflyCache { sum_e, sum_ie }
}

#[cfg(test)]
mod tests {
    use super::super::modint::{self, Mod998244353, Modulus, RemEuclidU32, StaticModInt};
    use rand::{rngs::ThreadRng, Rng as _};
    use std::{
        convert::{TryFrom, TryInto as _},
        fmt,
    };

    //https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L51-L71
    #[test]
    fn empty() {
        assert!(super::convolution_raw::<i32, Mod998244353>(&[], &[]).is_empty());
        assert!(super::convolution_raw::<i32, Mod998244353>(&[], &[1, 2]).is_empty());
        assert!(super::convolution_raw::<i32, Mod998244353>(&[1, 2], &[]).is_empty());
        assert!(super::convolution_raw::<i32, Mod998244353>(&[1], &[]).is_empty());
        assert!(super::convolution_raw::<i64, Mod998244353>(&[], &[]).is_empty());
        assert!(super::convolution_raw::<i64, Mod998244353>(&[], &[1, 2]).is_empty());
        assert!(super::convolution::<Mod998244353>(&[], &[]).is_empty());
        assert!(super::convolution::<Mod998244353>(&[], &[1.into(), 2.into()]).is_empty());
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L73-L85
    #[test]
    fn mid() {
        const N: usize = 1234;
        const M: usize = 2345;

        let mut rng = rand::thread_rng();
        let mut gen_values = |n| gen_values::<Mod998244353>(&mut rng, n);
        let (a, b) = (gen_values(N), gen_values(M));
        assert_eq!(conv_naive(&a, &b), super::convolution(&a, &b));
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L87-L118
    #[test]
    fn simple_s_mod() {
        const M1: u32 = 998_244_353;
        const M2: u32 = 924_844_033;

        modulus!(M1, M2);

        fn test<M: Modulus>(rng: &mut ThreadRng) {
            let mut gen_values = |n| gen_values::<Mod998244353>(rng, n);
            for (n, m) in (1..20).flat_map(|i| (1..20).map(move |j| (i, j))) {
                let (a, b) = (gen_values(n), gen_values(m));
                assert_eq!(conv_naive(&a, &b), super::convolution(&a, &b));
            }
        }

        let mut rng = rand::thread_rng();
        test::<M1>(&mut rng);
        test::<M2>(&mut rng);
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L120-L150
    #[test]
    fn simple_int() {
        simple_raw::<i32>();
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L152-L182
    #[test]
    fn simple_uint() {
        simple_raw::<u32>();
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L184-L214
    #[test]
    fn simple_ll() {
        simple_raw::<i64>();
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L216-L246
    #[test]
    fn simple_ull() {
        simple_raw::<u64>();
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L249-L279
    #[test]
    fn simple_int128() {
        simple_raw::<i128>();
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L281-L311
    #[test]
    fn simple_uint128() {
        simple_raw::<u128>();
    }

    fn simple_raw<T>()
    where
        T: TryFrom<u32> + Copy + RemEuclidU32,
        T::Error: fmt::Debug,
    {
        const M1: u32 = 998_244_353;
        const M2: u32 = 924_844_033;

        modulus!(M1, M2);

        fn test<T, M>(rng: &mut ThreadRng)
        where
            T: TryFrom<u32> + Copy + RemEuclidU32,
            T::Error: fmt::Debug,
            M: Modulus,
        {
            let mut gen_raw_values = |n| gen_raw_values::<u32, Mod998244353>(rng, n);
            for (n, m) in (1..20).flat_map(|i| (1..20).map(move |j| (i, j))) {
                let (a, b) = (gen_raw_values(n), gen_raw_values(m));
                assert_eq!(
                    conv_raw_naive::<_, M>(&a, &b),
                    super::convolution_raw::<_, M>(&a, &b),
                );
            }
        }

        let mut rng = rand::thread_rng();
        test::<T, M1>(&mut rng);
        test::<T, M2>(&mut rng);
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L315-L329
    #[test]
    fn conv_ll() {
        let mut rng = rand::thread_rng();
        for (n, m) in (1..20).flat_map(|i| (1..20).map(move |j| (i, j))) {
            let mut gen =
                |n: usize| -> Vec<_> { (0..n).map(|_| rng.gen_range(-500_000, 500_000)).collect() };
            let (a, b) = (gen(n), gen(m));
            assert_eq!(conv_i64_naive(&a, &b), super::convolution_i64(&a, &b));
        }
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L331-L356
    #[test]
    fn conv_ll_bound() {
        const M1: u64 = 754_974_721; // 2^24
        const M2: u64 = 167_772_161; // 2^25
        const M3: u64 = 469_762_049; // 2^26
        const M2M3: u64 = M2 * M3;
        const M1M3: u64 = M1 * M3;
        const M1M2: u64 = M1 * M2;

        modulus!(M1, M2, M3);

        for i in -1000..=1000 {
            let a = vec![0u64.wrapping_sub(M1M2 + M1M3 + M2M3) as i64 + i];
            let b = vec![1];
            assert_eq!(a, super::convolution_i64(&a, &b));
        }

        for i in 0..1000 {
            let a = vec![i64::min_value() + i];
            let b = vec![1];
            assert_eq!(a, super::convolution_i64(&a, &b));
        }

        for i in 0..1000 {
            let a = vec![i64::max_value() - i];
            let b = vec![1];
            assert_eq!(a, super::convolution_i64(&a, &b));
        }
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L358-L371
    #[test]
    fn conv_641() {
        const M: u32 = 641;
        modulus!(M);

        let mut rng = rand::thread_rng();
        let mut gen_values = |n| gen_values::<M>(&mut rng, n);
        let (a, b) = (gen_values(64), gen_values(65));
        assert_eq!(conv_naive(&a, &b), super::convolution(&a, &b));
    }

    // https://github.com/atcoder/ac-library/blob/8250de484ae0ab597391db58040a602e0dc1a419/test/unittest/convolution_test.cpp#L373-L386
    #[test]
    fn conv_18433() {
        const M: u32 = 18433;
        modulus!(M);

        let mut rng = rand::thread_rng();
        let mut gen_values = |n| gen_values::<M>(&mut rng, n);
        let (a, b) = (gen_values(1024), gen_values(1025));
        assert_eq!(conv_naive(&a, &b), super::convolution(&a, &b));
    }

    #[allow(clippy::many_single_char_names)]
    fn conv_naive<M: Modulus>(
        a: &[StaticModInt<M>],
        b: &[StaticModInt<M>],
    ) -> Vec<StaticModInt<M>> {
        let (n, m) = (a.len(), b.len());
        let mut c = vec![StaticModInt::raw(0); n + m - 1];
        for (i, j) in (0..n).flat_map(|i| (0..m).map(move |j| (i, j))) {
            c[i + j] += a[i] * b[j];
        }
        c
    }

    fn conv_raw_naive<T, M>(a: &[T], b: &[T]) -> Vec<T>
    where
        T: TryFrom<u32> + Copy + RemEuclidU32,
        T::Error: fmt::Debug,
        M: Modulus,
    {
        conv_naive::<M>(
            &a.iter().copied().map(Into::into).collect::<Vec<_>>(),
            &b.iter().copied().map(Into::into).collect::<Vec<_>>(),
        )
        .into_iter()
        .map(|x| x.val().try_into().unwrap())
        .collect()
    }

    #[allow(clippy::many_single_char_names)]
    fn conv_i64_naive(a: &[i64], b: &[i64]) -> Vec<i64> {
        let (n, m) = (a.len(), b.len());
        let mut c = vec![0; n + m - 1];
        for (i, j) in (0..n).flat_map(|i| (0..m).map(move |j| (i, j))) {
            c[i + j] += a[i] * b[j];
        }
        c
    }

    fn gen_values<M: Modulus>(rng: &mut ThreadRng, n: usize) -> Vec<StaticModInt<M>> {
        (0..n).map(|_| rng.gen_range(0, M::VALUE).into()).collect()
    }

    fn gen_raw_values<T, M>(rng: &mut ThreadRng, n: usize) -> Vec<T>
    where
        T: TryFrom<u32>,
        T::Error: fmt::Debug,
        M: Modulus,
    {
        (0..n)
            .map(|_| rng.gen_range(0, M::VALUE).try_into().unwrap())
            .collect()
    }
}

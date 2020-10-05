// remove this after dependencies has been added
#![allow(dead_code)]
use std::mem::swap;

/// # Arguments
/// * `m` `1 <= m`
///
/// # Returns
/// x mod m
/* const */
pub(crate) fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

/// Fast modular by barrett reduction
/// Reference: https://en.wikipedia.org/wiki/Barrett_reduction
/// NOTE: reconsider after Ice Lake
pub(crate) struct Barrett {
    pub(crate) _m: u32,
    pub(crate) im: u64,
}

impl Barrett {
    /// # Arguments
    /// * `m` `1 <= m`
    /// (Note: `m <= 2^31` should also hold, which is undocumented in the original library.
    /// See the [pull reqeust commment](https://github.com/rust-lang-ja/ac-library-rs/pull/3#discussion_r484661007)
    /// for more details.)
    pub(crate) fn new(m: u32) -> Barrett {
        Barrett {
            _m: m,
            im: (-1i64 as u64 / m as u64).wrapping_add(1),
        }
    }

    /// # Returns
    /// `m`
    pub(crate) fn umod(&self) -> u32 {
        self._m
    }

    /// # Parameters
    /// * `a` `0 <= a < m`
    /// * `b` `0 <= b < m`
    ///
    /// # Returns
    /// a * b % m
    #[allow(clippy::many_single_char_names)]
    pub(crate) fn mul(&self, a: u32, b: u32) -> u32 {
        mul_mod(a, b, self._m, self.im)
    }
}

/// Calculates `a * b % m`.
///
/// * `a` `0 <= a < m`
/// * `b` `0 <= b < m`
/// * `m` `1 <= m <= 2^31`
/// * `im` = ceil(2^64 / `m`)
#[allow(clippy::many_single_char_names)]
pub(crate) fn mul_mod(a: u32, b: u32, m: u32, im: u64) -> u32 {
    // [1] m = 1
    // a = b = im = 0, so okay

    // [2] m >= 2
    // im = ceil(2^64 / m)
    // -> im * m = 2^64 + r (0 <= r < m)
    // let z = a*b = c*m + d (0 <= c, d < m)
    // a*b * im = (c*m + d) * im = c*(im*m) + d*im = c*2^64 + c*r + d*im
    // c*r + d*im < m * m + m * im < m * m + 2^64 + m <= 2^64 + m * (m + 1) < 2^64 * 2
    // ((ab * im) >> 64) == c or c + 1
    let mut z = a as u64;
    z *= b as u64;
    let x = (((z as u128) * (im as u128)) >> 64) as u64;
    let mut v = z.wrapping_sub(x.wrapping_mul(m as u64)) as u32;
    if m <= v {
        v = v.wrapping_add(m);
    }
    v
}

/// # Parameters
/// * `n` `0 <= n`
/// * `m` `1 <= m`
///
/// # Returns
/// `(x ** n) % m`
/* const */
#[allow(clippy::many_single_char_names)]
pub(crate) fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
    if m == 1 {
        return 0;
    }
    let _m = m as u32;
    let mut r: u64 = 1;
    let mut y: u64 = safe_mod(x, m as i64) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (_m as u64);
        }
        y = (y * y) % (_m as u64);
        n >>= 1;
    }
    r as i64
}

/// Reference:
/// M. Forisek and J. Jancina,
/// Fast Primality Testing for Integers That Fit into a Machine Word
///
/// # Parameters
/// * `n` `0 <= n`
/* const */
pub(crate) fn is_prime(n: i32) -> bool {
    let n = n as i64;
    match n {
        _ if n <= 1 => return false,
        2 | 7 | 61 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &a in &[2, 7, 61] {
        let mut t = d;
        let mut y = pow_mod(a, t, n as i32);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = y * y % n;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

// omitted
// template <int n> constexpr bool is_prime = is_prime_constexpr(n);

/// # Parameters
/// * `b` `1 <= b`
///
/// # Returns
/// (g, x) s.t. g = gcd(a, b), xa = g (mod b), 0 <= x < b/g
/* const */
#[allow(clippy::many_single_char_names)]
pub(crate) fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }

    // Contracts:
    // [1] s - m0 * a = 0 (mod b)
    // [2] t - m1 * a = 0 (mod b)
    // [3] s * |m1| + t * |m0| <= b
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

        // [3]:
        // (s - t * u) * |m1| + t * |m0 - m1 * u|
        // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
        // = s * |m1| + t * |m0| <= b

        swap(&mut s, &mut t);
        swap(&mut m0, &mut m1);
    }
    // by [3]: |m0| <= b/g
    // by g != b: |m0| < b/g
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

/// Compile time (currently not) primitive root
/// @param m must be prime
/// @return primitive root (and minimum in now)
/* const */
pub(crate) fn primitive_root(m: i32) -> i32 {
    match m {
        2 => return 1,
        167_772_161 => return 3,
        469_762_049 => return 3,
        754_974_721 => return 11,
        998_244_353 => return 3,
        _ => {}
    }

    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    for i in (3..std::i32::MAX).step_by(2) {
        if i as i64 * i as i64 > x as i64 {
            break;
        }
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }
    let mut g = 2;
    loop {
        if (0..cnt).all(|i| pow_mod(g, ((m - 1) / divs[i]) as i64, m) != 1) {
            break g as i32;
        }
        g += 1;
    }
}
// omitted
// template <int m> constexpr int primitive_root = primitive_root_constexpr(m);

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal)]
    #![allow(clippy::cognitive_complexity)]
    use crate::internal_math::{inv_gcd, is_prime, pow_mod, primitive_root, safe_mod, Barrett};
    use std::collections::HashSet;

    #[test]
    fn test_safe_mod() {
        assert_eq!(safe_mod(0, 3), 0);
        assert_eq!(safe_mod(1, 3), 1);
        assert_eq!(safe_mod(2, 3), 2);
        assert_eq!(safe_mod(3, 3), 0);
        assert_eq!(safe_mod(4, 3), 1);
        assert_eq!(safe_mod(5, 3), 2);
        assert_eq!(safe_mod(73, 11), 7);
        assert_eq!(safe_mod(2306249155046129918, 6620319213327), 1374210749525);

        assert_eq!(safe_mod(-1, 3), 2);
        assert_eq!(safe_mod(-2, 3), 1);
        assert_eq!(safe_mod(-3, 3), 0);
        assert_eq!(safe_mod(-4, 3), 2);
        assert_eq!(safe_mod(-5, 3), 1);
        assert_eq!(safe_mod(-7170500492396019511, 777567337), 333221848);
    }

    #[test]
    fn test_barrett() {
        let b = Barrett::new(7);
        assert_eq!(b.umod(), 7);
        assert_eq!(b.mul(2, 3), 6);
        assert_eq!(b.mul(4, 6), 3);
        assert_eq!(b.mul(5, 0), 0);

        let b = Barrett::new(998244353);
        assert_eq!(b.umod(), 998244353);
        assert_eq!(b.mul(2, 3), 6);
        assert_eq!(b.mul(3141592, 653589), 919583920);
        assert_eq!(b.mul(323846264, 338327950), 568012980);

        // make `z - x * self._m as u64` overflow.
        // Thanks @koba-e964 (at https://github.com/rust-lang-ja/ac-library-rs/pull/3#discussion_r484932161)
        let b = Barrett::new(2147483647);
        assert_eq!(b.umod(), 2147483647);
        assert_eq!(b.mul(1073741824, 2147483645), 2147483646);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(0, 0, 1), 0);
        assert_eq!(pow_mod(0, 0, 3), 1);
        assert_eq!(pow_mod(0, 0, 723), 1);
        assert_eq!(pow_mod(0, 0, 998244353), 1);
        assert_eq!(pow_mod(0, 0, i32::max_value()), 1);

        assert_eq!(pow_mod(0, 1, 1), 0);
        assert_eq!(pow_mod(0, 1, 3), 0);
        assert_eq!(pow_mod(0, 1, 723), 0);
        assert_eq!(pow_mod(0, 1, 998244353), 0);
        assert_eq!(pow_mod(0, 1, i32::max_value()), 0);

        assert_eq!(pow_mod(0, i64::max_value(), 1), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 3), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 723), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 998244353), 0);
        assert_eq!(pow_mod(0, i64::max_value(), i32::max_value()), 0);

        assert_eq!(pow_mod(1, 0, 1), 0);
        assert_eq!(pow_mod(1, 0, 3), 1);
        assert_eq!(pow_mod(1, 0, 723), 1);
        assert_eq!(pow_mod(1, 0, 998244353), 1);
        assert_eq!(pow_mod(1, 0, i32::max_value()), 1);

        assert_eq!(pow_mod(1, 1, 1), 0);
        assert_eq!(pow_mod(1, 1, 3), 1);
        assert_eq!(pow_mod(1, 1, 723), 1);
        assert_eq!(pow_mod(1, 1, 998244353), 1);
        assert_eq!(pow_mod(1, 1, i32::max_value()), 1);

        assert_eq!(pow_mod(1, i64::max_value(), 1), 0);
        assert_eq!(pow_mod(1, i64::max_value(), 3), 1);
        assert_eq!(pow_mod(1, i64::max_value(), 723), 1);
        assert_eq!(pow_mod(1, i64::max_value(), 998244353), 1);
        assert_eq!(pow_mod(1, i64::max_value(), i32::max_value()), 1);

        assert_eq!(pow_mod(i64::max_value(), 0, 1), 0);
        assert_eq!(pow_mod(i64::max_value(), 0, 3), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, 723), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, 998244353), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, i32::max_value()), 1);

        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 1), 0);
        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 3), 1);
        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 723), 640);
        assert_eq!(
            pow_mod(i64::max_value(), i64::max_value(), 998244353),
            683296792
        );
        assert_eq!(
            pow_mod(i64::max_value(), i64::max_value(), i32::max_value()),
            1
        );

        assert_eq!(pow_mod(2, 3, 1_000_000_007), 8);
        assert_eq!(pow_mod(5, 7, 1_000_000_007), 78125);
        assert_eq!(pow_mod(123, 456, 1_000_000_007), 565291922);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));

        // assert!(is_prime(57));
        assert!(!is_prime(57));
        assert!(!is_prime(58));
        assert!(is_prime(59));
        assert!(!is_prime(60));
        assert!(is_prime(61));
        assert!(!is_prime(62));

        assert!(!is_prime(701928443));
        assert!(is_prime(998244353));
        assert!(!is_prime(1_000_000_000));
        assert!(is_prime(1_000_000_007));

        assert!(is_prime(i32::max_value()));
    }

    #[test]
    fn test_is_prime_sieve() {
        let n = 1_000_000;
        let mut prime = vec![true; n];
        prime[0] = false;
        prime[1] = false;
        for i in 0..n {
            assert_eq!(prime[i], is_prime(i as i32));
            if prime[i] {
                for j in (2 * i..n).step_by(i) {
                    prime[j] = false;
                }
            }
        }
    }

    #[test]
    fn test_inv_gcd() {
        for &(a, b, g) in &[
            (0, 1, 1),
            (0, 4, 4),
            (0, 7, 7),
            (2, 3, 1),
            (-2, 3, 1),
            (4, 6, 2),
            (-4, 6, 2),
            (13, 23, 1),
            (57, 81, 3),
            (12345, 67890, 15),
            (-3141592 * 6535, 3141592 * 8979, 3141592),
            (i64::max_value(), i64::max_value(), i64::max_value()),
            (i64::min_value(), i64::max_value(), 1),
        ] {
            let (g_, x) = inv_gcd(a, b);
            assert_eq!(g, g_);
            let b_ = b as i128;
            assert_eq!(((x as i128 * a as i128) % b_ + b_) % b_, g as i128 % b_);
        }
    }

    #[test]
    fn test_primitive_root() {
        for &p in &[
            2,
            3,
            5,
            7,
            233,
            200003,
            998244353,
            1_000_000_007,
            i32::max_value(),
        ] {
            assert!(is_prime(p));
            let g = primitive_root(p);
            if p != 2 {
                assert_ne!(g, 1);
            }

            let q = p - 1;
            for i in (2..i32::max_value()).take_while(|i| i * i <= q) {
                if q % i != 0 {
                    break;
                }
                for &r in &[i, q / i] {
                    assert_ne!(pow_mod(g as i64, r as i64, p), 1);
                }
            }
            assert_eq!(pow_mod(g as i64, q as i64, p), 1);

            if p < 1_000_000 {
                assert_eq!(
                    (0..p - 1)
                        .scan(1, |i, _| {
                            *i = *i * g % p;
                            Some(*i)
                        })
                        .collect::<HashSet<_>>()
                        .len() as i32,
                    p - 1
                );
            }
        }
    }
}

//! Number-theoretic algorithms.

use crate::internal_math;

use std::mem::swap;

/// Returns $x^n \bmod m$.
///
/// # Constraints
///
/// - $0 \leq n$
/// - $1 \leq m$
///
/// # Panics
///
/// Panics if the above constraints are not satisfied.
///
/// # Complexity
///
/// - $O(\log n)$
///
/// # Example
///
/// ```
/// use ac_library::math;
///
/// assert_eq!(math::pow_mod(2, 10000, 7), 2);
/// ```
#[allow(clippy::many_single_char_names)]
pub fn pow_mod(x: i64, mut n: i64, m: u32) -> u32 {
    assert!(0 <= n && 1 <= m && m <= 2u32.pow(31));
    if m == 1 {
        return 0;
    }
    let bt = internal_math::Barrett::new(m);
    let mut r = 1;
    let mut y = internal_math::safe_mod(x, m as i64) as u32;
    while n != 0 {
        if n & 1 != 0 {
            r = bt.mul(r, y);
        }
        y = bt.mul(y, y);
        n >>= 1;
    }
    r
}

/// Returns an integer $y \in [0, m)$ such that $xy \equiv 1 \pmod m$.
///
/// # Constraints
///
/// - $\gcd(x, m) = 1$
/// - $1 \leq m$
///
/// # Panics
///
/// Panics if the above constraints are not satisfied.
///
/// # Complexity
///
/// - $O(\log m)$
///
/// # Example
///
/// ```
/// use ac_library::math;
///
/// assert_eq!(math::inv_mod(3, 7), 5);
/// ```
pub fn inv_mod(x: i64, m: i64) -> i64 {
    assert!(1 <= m);
    let z = internal_math::inv_gcd(x, m);
    assert!(z.0 == 1);
    z.1
}

/// Performs CRT (Chinese Remainder Theorem).
///
/// Given two sequences $r, m$ of length $n$, this function solves the modular equation system
///
/// \\[
///   x \equiv r_i \pmod{m_i}, \forall i \in \\{0, 1, \cdots, n - 1\\}
/// \\]
///
/// If there is no solution, it returns $(0, 0)$.
///
/// Otherwise, all of the solutions can be written as the form $x \equiv y \pmod z$, using integer $y, z\\ (0 \leq y < z = \text{lcm}(m))$.
/// It returns this $(y, z)$.
///
/// If $n = 0$, it returns $(0, 1)$.
///
/// # Constraints
///
/// - $|r| = |m|$
/// - $1 \leq m_{\forall i}$
/// - $\text{lcm}(m)$ is in `i64`
///
/// # Panics
///
/// Panics if the above constraints are not satisfied.
///
/// # Complexity
///
/// - $O(n \log \text{lcm}(m))$
///
/// # Example
///
/// ```
/// use ac_library::math;
///
/// let r = [2, 3, 2];
/// let m = [3, 5, 7];
/// assert_eq!(math::crt(&r, &m), (23, 105));
/// ```
pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    assert_eq!(r.len(), m.len());
    // Contracts: 0 <= r0 < m0
    let (mut r0, mut m0) = (0, 1);
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        assert!(1 <= mi);
        ri = internal_math::safe_mod(ri, mi);
        if m0 < mi {
            swap(&mut r0, &mut ri);
            swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return (0, 0);
            }
            continue;
        }
        // assume: m0 > mi, lcm(m0, mi) >= 2 * max(m0, mi)

        // (r0, m0), (ri, mi) -> (r2, m2 = lcm(m0, m1));
        // r2 % m0 = r0
        // r2 % mi = ri
        // -> (r0 + x*m0) % mi = ri
        // -> x*u0*g = ri-r0 (mod u1*g) (u0*g = m0, u1*g = mi)
        // -> x = (ri - r0) / g * inv(u0) (mod u1)

        // im = inv(u0) (mod u1) (0 <= im < u1)
        let (g, im) = internal_math::inv_gcd(m0, mi);
        let u1 = mi / g;
        // |ri - r0| < (m0 + mi) <= lcm(m0, mi)
        if (ri - r0) % g != 0 {
            return (0, 0);
        }
        // u1 * u1 <= mi * mi / g / g <= m0 * mi / g = lcm(m0, mi)
        let x = (ri - r0) / g % u1 * im % u1;

        // |r0| + |m0 * x|
        // < m0 + m0 * (u1 - 1)
        // = m0 + m0 * mi / g - m0
        // = lcm(m0, mi)
        r0 += x * m0;
        m0 *= u1; // -> lcm(m0, mi)
        if r0 < 0 {
            r0 += m0
        };
    }

    (r0, m0)
}

/// Returns $\sum_{i = 0}^{n - 1} \lfloor \frac{a \times i + b}{m} \rfloor$.
///
/// # Constraints
///
/// - $0 \leq n \leq 10^9$
/// - $1 \leq m \leq 10^9$
/// - $0 \leq a, b \leq m$
///
/// # Panics
///
/// Panics if the above constraints are not satisfied and overflow or division by zero occurred.
///
/// # Complexity
///
/// - $O(\log(n + m + a + b))$
///
/// # Example
///
/// ```
/// use ac_library::math;
///
/// assert_eq!(math::floor_sum(6, 5, 4, 3), 13);
/// ```
pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        return ans;
    }
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    ans
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal)]
    #![allow(clippy::cognitive_complexity)]
    use super::*;
    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(0, 0, 1), 0);
        assert_eq!(pow_mod(0, 0, 3), 1);
        assert_eq!(pow_mod(0, 0, 723), 1);
        assert_eq!(pow_mod(0, 0, 998244353), 1);
        assert_eq!(pow_mod(0, 0, 2u32.pow(31)), 1);

        assert_eq!(pow_mod(0, 1, 1), 0);
        assert_eq!(pow_mod(0, 1, 3), 0);
        assert_eq!(pow_mod(0, 1, 723), 0);
        assert_eq!(pow_mod(0, 1, 998244353), 0);
        assert_eq!(pow_mod(0, 1, 2u32.pow(31)), 0);

        assert_eq!(pow_mod(0, i64::max_value(), 1), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 3), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 723), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 998244353), 0);
        assert_eq!(pow_mod(0, i64::max_value(), 2u32.pow(31)), 0);

        assert_eq!(pow_mod(1, 0, 1), 0);
        assert_eq!(pow_mod(1, 0, 3), 1);
        assert_eq!(pow_mod(1, 0, 723), 1);
        assert_eq!(pow_mod(1, 0, 998244353), 1);
        assert_eq!(pow_mod(1, 0, 2u32.pow(31)), 1);

        assert_eq!(pow_mod(1, 1, 1), 0);
        assert_eq!(pow_mod(1, 1, 3), 1);
        assert_eq!(pow_mod(1, 1, 723), 1);
        assert_eq!(pow_mod(1, 1, 998244353), 1);
        assert_eq!(pow_mod(1, 1, 2u32.pow(31)), 1);

        assert_eq!(pow_mod(1, i64::max_value(), 1), 0);
        assert_eq!(pow_mod(1, i64::max_value(), 3), 1);
        assert_eq!(pow_mod(1, i64::max_value(), 723), 1);
        assert_eq!(pow_mod(1, i64::max_value(), 998244353), 1);
        assert_eq!(pow_mod(1, i64::max_value(), 2u32.pow(31)), 1);

        assert_eq!(pow_mod(i64::max_value(), 0, 1), 0);
        assert_eq!(pow_mod(i64::max_value(), 0, 3), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, 723), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, 998244353), 1);
        assert_eq!(pow_mod(i64::max_value(), 0, 2u32.pow(31)), 1);

        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 1), 0);
        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 3), 1);
        assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 723), 640);
        assert_eq!(
            pow_mod(i64::max_value(), i64::max_value(), 998244353),
            683296792
        );
        assert_eq!(
            pow_mod(i64::max_value(), i64::max_value(), 2u32.pow(31)),
            2147483647
        );

        assert_eq!(pow_mod(2, 3, 1_000_000_007), 8);
        assert_eq!(pow_mod(5, 7, 1_000_000_007), 78125);
        assert_eq!(pow_mod(123, 456, 1_000_000_007), 565291922);
    }

    #[test]
    #[should_panic]
    fn test_inv_mod_1() {
        inv_mod(271828, 0);
    }

    #[test]
    #[should_panic]
    fn test_inv_mod_2() {
        inv_mod(3141592, 1000000008);
    }

    #[test]
    fn test_crt() {
        let a = [44, 23, 13];
        let b = [13, 50, 22];
        assert_eq!(crt(&a, &b), (1773, 7150));
        let a = [12345, 67890, 99999];
        let b = [13, 444321, 95318];
        assert_eq!(crt(&a, &b), (103333581255, 550573258014));
        let a = [0, 3, 4];
        let b = [1, 9, 5];
        assert_eq!(crt(&a, &b), (39, 45));
    }

    #[test]
    fn test_floor_sum() {
        assert_eq!(floor_sum(0, 1, 0, 0), 0);
        assert_eq!(floor_sum(1_000_000_000, 1, 1, 1), 500_000_000_500_000_000);
        assert_eq!(
            floor_sum(1_000_000_000, 1_000_000_000, 999_999_999, 999_999_999),
            499_999_999_500_000_000
        );
        assert_eq!(floor_sum(332955, 5590132, 2231, 999423), 22014575);
    }
}

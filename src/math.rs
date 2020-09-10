use crate::internal_math;

use std::mem::swap;

#[allow(clippy::many_single_char_names)]
pub fn pow_mod(x: i64, mut n: i64, m: u32) -> u32 {
    assert!(0 <= n && 1 <= m);

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

pub fn inv_mod(x: i64, m: i64) -> i64 {
    assert!(1 <= m);
    let z = internal_math::inv_gcd(x, m);
    assert!(z.0 == 1);
    z.1
}

pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    assert!(r.len() == m.len());
    // Contracts: 0 <= r0 < m0
    let (mut r0, mut m0) = (0, 1);
    for (ri, mi) in r.iter().zip(m.iter()) {
        assert!(1 < *mi);
        let mut r1 = internal_math::safe_mod(*ri, *mi);
        let mut m1 = *mi;
        if m0 < m1 {
            swap(&mut r0, &mut r1);
            swap(&mut m0, &mut m1);
        }
        if m0 % m1 == 0 {
            if r0 % m1 != r1 {
                return (0, 0);
            }
            continue;
        }
        // assume: m0 > m1, lcm(m0, m1) >= 2 * max(m0, m1)

        // (r0, m0), (r1, m1) -> (r2, m2 = lcm(m0, m1));
        // r2 % m0 = r0
        // r2 % m1 = r1
        // -> (r0 + x*m0) % m1 = r1
        // -> x*u0*g % (u1*g) = (r1 - r0) (u0*g = m0, u1*g = m1)
        // -> x = (r1 - r0) / g * inv(u0) (mod u1)

        // im = inv(u0) (mod u1) (0 <= im < u1)
        let (g, im) = internal_math::inv_gcd(m0, m1);
        let u1 = m1 / g;
        // |r1 - r0| < (m0 + m1) <= lcm(m0, m1)
        if (r1 - r0) % g != 0 {
            return (0, 0);
        }
        // u1 * u1 <= m1 * m1 / g / g <= m0 * m1 / g = lcm(m0, m1)
        let x = (r1 - r0) / g % u1 * im % u1;

        // |r0| + |m0 * x|
        // < m0 + m0 * (u1 - 1)
        // = m0 + m0 * m1 / g - m0
        // = lcm(m0, m1)
        r0 += x * m0;
        m0 *= u1; // -> lcm(m0, m1)
        if r0 < 0 {
            r0 += m0
        };
    }

    (r0, m0)
}

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

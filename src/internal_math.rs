use std::mem::swap;

mod atcoder {

    mod internal {

        /// # Arguments
        /// * `m` `1 <= m`
        ///
        /// # Returns
        /// x mod m
        /* const */
        fn safe_mod(mut x: i64, m: i64) -> i64 {
            x %= m;
            if x < 0 {
                x += m;
            }
            x
        }

        /// Fast moduler by barrett reduction
        /// Reference: https://en.wikipedia.org/wiki/Barrett_reduction
        /// NOTE: reconsider after Ice Lake
        struct Barrett {
            _m: u32,
            im: u64,
        }

        impl Barrett {
            /// # Arguments
            /// * `m` `1 <= m`
            fn new(m: u32) -> Barrett {
                Barrett {
                    _m: m,
                    im: (-1i64 as u64) / (m as u64) + 1,
                }
            }

            /// # Returns
            /// `m`
            fn umod(&self) -> u32 {
                self._m
            }

            /// # Parameters
            /// * `a` `0 <= a < m`
            /// * `b` `0 <= b < m`
            ///
            /// # Returns
            /// a * b % m
            fn mul(&self, a: u32, b: u32) -> u32 {
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
                let x = (((z as u128) * (self.im as u128)) >> 64) as u64;
                let mut v = (z - x * self._m as u64) as u32;
                if self._m <= v {
                    v += self._m;
                }
                v
            }
        }

        /// # Parameters
        /// * `n` `0 <= n`
        /// * `m` `1 <= m`
        ///
        /// # Returns
        /// `(x ** n) % m`
        /* const */
        fn pow_mod_constexpr(x: i64, mut n: i64, m: i32) -> i64 {
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
        fn is_prime_constexpr(n: i32) -> bool {
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
            for a in [2, 7, 61].iter().copied() {
                let mut t = d;
                let mut y = pow_mod_constexpr(a, t, n as i32);
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
        fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
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
        fn primitive_root_constexpr(m: i32) -> i32 {
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
                if (i as i64) * (i as i64) <= (x as i64) {
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
                if (0..cnt).any(|i| pow_mod_constexpr(g, ((m - 1) / divs[i]) as i64, m) == 1) {
                    break g as i32;
                }
                g += 1;
            }

            // omitted
            // template <int m> constexpr int primitive_root = primitive_root_constexpr(m);
        }
    }
}

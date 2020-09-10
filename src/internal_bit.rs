// Skipped:
//
// - `bsf` = `__builtin_ctz`: should equivalent to `{integer}::trailing_zeros`

#[allow(dead_code)]
pub(crate) fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ceil_pow2() {
        // https://github.com/atcoder/ac-library/blob/2088c8e2431c3f4d29a2cfabc6529fe0a0586c48/test/unittest/bit_test.cpp
        assert_eq!(0, super::ceil_pow2(0));
        assert_eq!(0, super::ceil_pow2(1));
        assert_eq!(1, super::ceil_pow2(2));
        assert_eq!(2, super::ceil_pow2(3));
        assert_eq!(2, super::ceil_pow2(4));
        assert_eq!(3, super::ceil_pow2(5));
        assert_eq!(3, super::ceil_pow2(6));
        assert_eq!(3, super::ceil_pow2(7));
        assert_eq!(3, super::ceil_pow2(8));
        assert_eq!(4, super::ceil_pow2(9));
        assert_eq!(30, super::ceil_pow2(1 << 30));
        assert_eq!(31, super::ceil_pow2((1 << 30) + 1));

        assert_eq!(32, super::ceil_pow2(u32::max_value()));
    }
}

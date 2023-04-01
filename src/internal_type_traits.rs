use std::{
    fmt,
    iter::{self, Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

// Skipped:
//
// - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
// - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
// - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

/// Corresponds to `std::is_integral` in C++.
// We will remove unnecessary bounds later.
//
// Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
// same way as the original ACL.
pub trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
    + Zero
    + One
    + BoundedBelow
    + BoundedAbove
{
}

/// Class that has additive identity element
pub trait Zero: Sum {
    /// The additive identity element
    #[inline]
    fn zero() -> Self {
        iter::empty().sum()
    }
}

impl<T: Sum> Zero for T {}

/// Class that has multiplicative identity element
pub trait One: Product {
    /// The multiplicative identity element
    #[inline]
    fn one() -> Self {
        iter::empty().product()
    }
}

impl<T: Product> One for T {}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::{One, Zero};

    #[test]
    fn zero() {
        assert_eq!(0, i8::zero());
        assert_eq!(0, i16::zero());
        assert_eq!(0, i32::zero());
        assert_eq!(0, i64::zero());
        assert_eq!(0, i128::zero());
        assert_eq!(0, isize::zero());
        assert_eq!(0, u8::zero());
        assert_eq!(0, u16::zero());
        assert_eq!(0, u32::zero());
        assert_eq!(0, u64::zero());
        assert_eq!(0, u128::zero());
        assert_eq!(0, usize::zero());
    }

    #[test]
    fn one() {
        assert_eq!(1, i8::one());
        assert_eq!(1, i16::one());
        assert_eq!(1, i32::one());
        assert_eq!(1, i64::one());
        assert_eq!(1, i128::one());
        assert_eq!(1, isize::one());
        assert_eq!(1, u8::one());
        assert_eq!(1, u16::one());
        assert_eq!(1, u32::one());
        assert_eq!(1, u64::one());
        assert_eq!(1, u128::one());
        assert_eq!(1, usize::one());
    }
}

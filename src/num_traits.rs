/// A type that has an additive identity element.
pub trait Zero {
    /// The additive identity element.
    fn zero() -> Self;
}

/// A type that has a multiplicative identity element.
pub trait One {
    /// The multiplicative identity element.
    fn one() -> Self;
}

macro_rules! impl_zero {
    ($zero: literal, $one: literal: $($t: ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    $zero
                }
            }

            impl One for $t {
                fn one() -> Self {
                    $one
                }
            }
        )*
    };
}
impl_zero!(0, 1: usize, u8, u16, u32, u64, u128);
impl_zero!(0, 1: isize, i8, i16, i32, i64, i128);
impl_zero!(0.0, 1.0: f32, f64);

impl<T: Zero> Zero for core::num::Wrapping<T> {
    fn zero() -> Self {
        Self(T::zero())
    }
}
impl<T: One> One for core::num::Wrapping<T> {
    fn one() -> Self {
        Self(T::one())
    }
}

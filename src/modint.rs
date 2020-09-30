//! Structs that treat the modular arithmetic.
//!
//! # Major changes from the original ACL
//!
//! - Converted the struct names to PascalCase.
//! - Renamed `mod` → `modulus`.
//! - Moduli are `u32`, not `i32`.
//! - `Id`s are `usize`, not `i32`.
//! - The default `Id` is `0`, not `-1`.
//! - The type of the argument of `pow` is `u64`, not `i64`.
//! - Modints implement `FromStr` and `Display`. Modints in the original ACL don't have `operator<<` or `operator>>`.

use crate::internal_math;
use std::{
    cell::RefCell,
    convert::{Infallible, TryInto as _},
    fmt,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    thread::LocalKey,
};

pub type ModInt1000000007 = StaticModInt<Mod1000000007>;
pub type ModInt998244353 = StaticModInt<Mod998244353>;
pub type ModInt = DynamicModInt<DefaultId>;

/// Corresponds to `atcoder::static_modint` in the original ACL.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StaticModInt<M> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M: Modulus> StaticModInt<M> {
    /// Corresponds to `atcoder::static_modint::mod` in the original ACL.
    #[inline(always)]
    pub fn modulus() -> u32 {
        M::VALUE
    }

    /// Creates a new `StaticModInt`.
    #[inline]
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(M::VALUE))
    }

    /// Corresponds to `atcoder::static_modint::raw` in the original ACL.
    #[inline]
    pub fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    /// Corresponds to `atcoder::static_modint::val` in the original ACL.
    #[inline]
    pub fn val(self) -> u32 {
        self.val
    }

    /// Corresponds to `atcoder::static_modint::pow` in the original ACL.
    #[inline]
    pub fn pow(self, n: u64) -> Self {
        <Self as ModIntBase>::pow(self, n)
    }

    /// Corresponds to `atcoder::static_modint::inv` in the original ACL.
    ///
    /// # Panics
    ///
    /// Panics if the multiplicative inverse does not exist.
    #[inline]
    pub fn inv(self) -> Self {
        if M::HINT_VALUE_IS_PRIME {
            if self.val() == 0 {
                panic!("attempt to divide by zero");
            }
            debug_assert!(
                internal_math::is_prime(M::VALUE.try_into().unwrap()),
                "{} is not a prime number",
                M::VALUE,
            );
            self.pow((M::VALUE - 2).into())
        } else {
            Self::inv_for_non_prime_modulus(self)
        }
    }
}

impl<M: Modulus> ModIntBase for StaticModInt<M> {
    #[inline(always)]
    fn modulus() -> u32 {
        Self::modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self::raw(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.val()
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }
}

pub trait Modulus: 'static + Copy + Eq {
    const VALUE: u32;
    const HINT_VALUE_IS_PRIME: bool;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod1000000007 {}

impl Modulus for Mod1000000007 {
    const VALUE: u32 = 1_000_000_007;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod1000000007>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const VALUE: u32 = 998_244_353;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod998244353>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}

pub struct ButterflyCache<M> {
    pub(crate) sum_e: Vec<StaticModInt<M>>,
    pub(crate) sum_ie: Vec<StaticModInt<M>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DynamicModInt<I> {
    val: u32,
    phantom: PhantomData<fn() -> I>,
}

impl<I: Id> DynamicModInt<I> {
    #[inline]
    pub fn modulus() -> u32 {
        I::companion_barrett().with(|bt| bt.borrow().umod())
    }

    #[inline]
    pub fn set_modulus(modulus: u32) {
        if modulus == 0 {
            panic!("the modulus must not be 0");
        }
        I::companion_barrett().with(|bt| *bt.borrow_mut() = Barrett::new(modulus))
    }

    #[inline]
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        <Self as ModIntBase>::new(val)
    }

    #[inline]
    pub fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn val(self) -> u32 {
        self.val
    }

    #[inline]
    pub fn pow(self, n: u64) -> Self {
        <Self as ModIntBase>::pow(self, n)
    }

    #[inline]
    pub fn inv(self) -> Self {
        Self::inv_for_non_prime_modulus(self)
    }
}

impl<I: Id> ModIntBase for DynamicModInt<I> {
    #[inline]
    fn modulus() -> u32 {
        Self::modulus()
    }

    #[inline]
    fn raw(val: u32) -> Self {
        Self::raw(val)
    }

    #[inline]
    fn val(self) -> u32 {
        self.val()
    }

    #[inline]
    fn inv(self) -> Self {
        self.inv()
    }
}

pub trait Id: 'static + Copy + Eq {
    // TODO: Make `internal_math::Barret` `Copy`.
    fn companion_barrett() -> &'static LocalKey<RefCell<Barrett>>;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DefaultId {}

impl Id for DefaultId {
    fn companion_barrett() -> &'static LocalKey<RefCell<Barrett>> {
        thread_local! {
            static BARRETT: RefCell<Barrett> = RefCell::default();
        }
        &BARRETT
    }
}

pub struct Barrett(internal_math::Barrett);

impl Barrett {
    #[inline]
    pub fn new(m: u32) -> Self {
        Self(internal_math::Barrett::new(m))
    }

    #[inline]
    fn umod(&self) -> u32 {
        self.0.umod()
    }

    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        self.0.mul(a, b)
    }
}

impl Default for Barrett {
    #[inline]
    fn default() -> Self {
        Self(internal_math::Barrett::new(998_244_353))
    }
}

pub trait ModIntBase:
    Default
    + FromStr
    + From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + Copy
    + Eq
    + Hash
    + fmt::Display
    + fmt::Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn modulus() -> u32;
    fn raw(val: u32) -> Self;
    fn val(self) -> u32;
    fn inv(self) -> Self;

    #[inline]
    fn new<T: RemEuclidU32>(val: T) -> Self {
        Self::raw(val.rem_euclid_u32(Self::modulus()))
    }

    #[inline]
    fn pow(self, mut n: u64) -> Self {
        let mut x = self;
        let mut r = Self::raw(1);
        while n > 0 {
            if n & 1 == 1 {
                r *= x;
            }
            x *= x;
            n >>= 1;
        }
        r
    }
}

pub trait RemEuclidU32 {
    fn rem_euclid_u32(self, modulus: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_small_signed {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self as i64).rem_euclid(i64::from(modulus)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_signed!(i8, i16, i32, i64, isize);

impl RemEuclidU32 for i128 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self.rem_euclid(i128::from(modulus)) as _
    }
}

macro_rules! impl_rem_euclid_u32_for_small_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    self as u32 % modulus
                }
            }
        )*
    }
}

macro_rules! impl_rem_euclid_u32_for_large_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self % (modulus as $ty)) as _
                }
            }
        )*
    }
}

impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);
impl_rem_euclid_u32_for_large_unsigned!(u64, u128);

#[cfg(target_pointer_width = "32")]
impl_rem_euclid_u32_for_small_unsigned!(usize);

#[cfg(target_pointer_width = "64")]
impl_rem_euclid_u32_for_large_unsigned!(usize);

trait InternalImplementations: ModIntBase {
    #[inline]
    fn inv_for_non_prime_modulus(this: Self) -> Self {
        let (gcd, x) = internal_math::inv_gcd(this.val().into(), Self::modulus().into());
        if gcd != 1 {
            panic!("the multiplicative inverse does not exist");
        }
        Self::new(x)
    }

    #[inline]
    fn default_impl() -> Self {
        Self::raw(0)
    }

    #[inline]
    fn from_str_impl(s: &str) -> Result<Self, Infallible> {
        Ok(s.parse::<i64>()
            .map(Self::new)
            .unwrap_or_else(|_| todo!("parsing as an arbitrary precision integer?")))
    }

    #[inline]
    fn hash_impl(this: &Self, state: &mut impl Hasher) {
        this.val().hash(state)
    }

    #[inline]
    fn display_impl(this: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&this.val(), f)
    }

    #[inline]
    fn debug_impl(this: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&this.val(), f)
    }

    #[inline]
    fn neg_impl(this: Self) -> Self {
        Self::sub_impl(Self::raw(0), this)
    }

    #[inline]
    fn add_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val() + rhs.val();
        if val >= modulus {
            val -= modulus;
        }
        Self::raw(val)
    }

    #[inline]
    fn sub_impl(lhs: Self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        let mut val = lhs.val().wrapping_sub(rhs.val());
        if val >= modulus {
            val = val.wrapping_add(modulus)
        }
        Self::raw(val)
    }

    fn mul_impl(lhs: Self, rhs: Self) -> Self;

    #[inline]
    fn div_impl(lhs: Self, rhs: Self) -> Self {
        Self::mul_impl(lhs, rhs.inv())
    }
}

impl<M: Modulus> InternalImplementations for StaticModInt<M> {
    #[inline]
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        Self::raw((u64::from(lhs.val()) * u64::from(rhs.val()) % u64::from(M::VALUE)) as u32)
    }
}

impl<I: Id> InternalImplementations for DynamicModInt<I> {
    #[inline]
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        I::companion_barrett().with(|bt| Self::raw(bt.borrow().mul(lhs.val, rhs.val)))
    }
}

macro_rules! impl_basic_traits {
    () => {};
    (impl <$generic_param:ident : $generic_param_bound:tt> _ for $self:ty; $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Default for $self {
            #[inline]
            fn default() -> Self {
                Self::default_impl()
            }
        }

        impl <$generic_param: $generic_param_bound> FromStr for $self {
            type Err = Infallible;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Infallible> {
                Self::from_str_impl(s)
            }
        }

        impl<$generic_param: $generic_param_bound, V: RemEuclidU32> From<V> for $self {
            #[inline]
            fn from(from: V) -> Self {
                Self::new(from)
            }
        }

        #[allow(clippy::derive_hash_xor_eq)]
        impl<$generic_param: $generic_param_bound> Hash for $self {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Self::hash_impl(self, state)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Display for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::display_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Debug for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::debug_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                Self::neg_impl(self)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for &'_ $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                <$self>::neg_impl(*self)
            }
        }

        impl_basic_traits!($($rest)*);
    };
}

impl_basic_traits! {
    impl <M: Modulus> _ for StaticModInt<M> ;
    impl <I: Id     > _ for DynamicModInt<I>;
}

macro_rules! impl_bin_ops {
    () => {};
    (for<$generic_param:ident : $generic_param_bound:tt> <$lhs_ty:ty> ~ <$rhs_ty:ty> -> $output:ty { { $lhs_body:expr } ~ { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Add<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs_ty) -> $output {
                <$output>::add_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Sub<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs_ty) -> $output {
                <$output>::sub_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Mul<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs_ty) -> $output {
                <$output>::mul_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Div<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn div(self, rhs: $rhs_ty) -> $output {
                <$output>::div_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl_bin_ops!($($rest)*);
    };
}

macro_rules! impl_assign_ops {
    () => {};
    (for<$generic_param:ident : $generic_param_bound:tt> <$lhs_ty:ty> ~= <$rhs_ty:ty> { _ ~= { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> AddAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs_ty) {
                *self = *self + apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> SubAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs_ty) {
                *self = *self - apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> MulAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs_ty) {
                *self = *self * apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> DivAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn div_assign(&mut self, rhs: $rhs_ty) {
                *self = *self / apply($rhs_body, rhs);
            }
        }

        impl_assign_ops!($($rest)*);
    };
}

#[inline]
fn apply<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
    f(x)
}

impl_bin_ops! {
    for<M: Modulus> <StaticModInt<M>     > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |x| x  } ~ { |x| x  } }
    for<M: Modulus> <StaticModInt<M>     > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |x| x  } ~ { |&x| x } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |&x| x } ~ { |x| x  } }
    for<M: Modulus> <&'_ StaticModInt<M> > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |&x| x } ~ { |&x| x } }
    for<I: Id     > <DynamicModInt<I>    > ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |x| x  } ~ { |x| x  } }
    for<I: Id     > <DynamicModInt<I>    > ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |x| x  } ~ { |&x| x } }
    for<I: Id     > <&'_ DynamicModInt<I>> ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |&x| x } ~ { |x| x  } }
    for<I: Id     > <&'_ DynamicModInt<I>> ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |&x| x } ~ { |&x| x } }
}

impl_assign_ops! {
    for<M: Modulus> <StaticModInt<M> > ~= <StaticModInt<M>     > { _ ~= { |x| x  } }
    for<M: Modulus> <StaticModInt<M> > ~= <&'_ StaticModInt<M> > { _ ~= { |&x| x } }
    for<I: Id     > <DynamicModInt<I>> ~= <DynamicModInt<I>    > { _ ~= { |x| x  } }
    for<I: Id     > <DynamicModInt<I>> ~= <&'_ DynamicModInt<I>> { _ ~= { |&x| x } }
}

macro_rules! impl_folding {
    () => {};
    (impl<$generic_param:ident : $generic_param_bound:tt> $trait:ident<_> for $self:ty { fn $method:ident(_) -> _ { _($unit:expr, $op:expr) } } $($rest:tt)*) => {
        impl<$generic_param: $generic_param_bound> $trait<Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl<'a, $generic_param: $generic_param_bound> $trait<&'a Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = &'a Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl_folding!($($rest)*);
    };
}

impl_folding! {
    impl<M: Modulus> Sum<_>     for StaticModInt<M>  { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
    impl<M: Modulus> Product<_> for StaticModInt<M>  { fn product(_) -> _ { _(Self::raw(1), Mul::mul) } }
    impl<I: Id     > Sum<_>     for DynamicModInt<I> { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
    impl<I: Id     > Product<_> for DynamicModInt<I> { fn product(_) -> _ { _(Self::raw(1), Mul::mul) } }
}

#[cfg(test)]
mod tests {
    use crate::modint::ModInt1000000007;

    #[test]
    fn static_modint_new() {
        assert_eq!(0, ModInt1000000007::new(0u32).val);
        assert_eq!(1, ModInt1000000007::new(1u32).val);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008u32).val);

        assert_eq!(0, ModInt1000000007::new(0u64).val);
        assert_eq!(1, ModInt1000000007::new(1u64).val);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008u64).val);

        assert_eq!(0, ModInt1000000007::new(0usize).val);
        assert_eq!(1, ModInt1000000007::new(1usize).val);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008usize).val);

        assert_eq!(0, ModInt1000000007::new(0i64).val);
        assert_eq!(1, ModInt1000000007::new(1i64).val);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008i64).val);
        assert_eq!(1_000_000_006, ModInt1000000007::new(-1i64).val);
    }

    #[test]
    fn static_modint_add() {
        fn add(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) + ModInt1000000007::new(rhs)).val
        }

        assert_eq!(2, add(1, 1));
        assert_eq!(1, add(1_000_000_006, 2));
    }

    #[test]
    fn static_modint_sub() {
        fn sub(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) - ModInt1000000007::new(rhs)).val
        }

        assert_eq!(1, sub(2, 1));
        assert_eq!(1_000_000_006, sub(0, 1));
    }

    #[test]
    fn static_modint_mul() {
        fn mul(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) * ModInt1000000007::new(rhs)).val
        }

        assert_eq!(1, mul(1, 1));
        assert_eq!(4, mul(2, 2));
        assert_eq!(999_999_937, mul(100_000, 100_000));
    }

    #[test]
    fn static_modint_prime_div() {
        fn div(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) / ModInt1000000007::new(rhs)).val
        }

        assert_eq!(0, div(0, 1));
        assert_eq!(1, div(1, 1));
        assert_eq!(1, div(2, 2));
        assert_eq!(23_809_524, div(1, 42));
    }

    #[test]
    fn static_modint_sum() {
        fn sum(values: &[i64]) -> ModInt1000000007 {
            values.iter().copied().map(ModInt1000000007::new).sum()
        }

        assert_eq!(ModInt1000000007::new(-3), sum(&[-1, 2, -3, 4, -5]));
    }

    #[test]
    fn static_modint_product() {
        fn product(values: &[i64]) -> ModInt1000000007 {
            values.iter().copied().map(ModInt1000000007::new).product()
        }

        assert_eq!(ModInt1000000007::new(-120), product(&[-1, 2, -3, 4, -5]));
    }
}

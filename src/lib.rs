#[cfg(feature = "convolution")]
pub mod convolution;
#[cfg(feature = "dsu")]
pub mod dsu;
#[cfg(feature = "fenwicktree")]
pub mod fenwicktree;
#[cfg(feature = "lazysegtree")]
pub mod lazysegtree;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "maxflow")]
pub mod maxflow;
#[cfg(feature = "mincostflow")]
pub mod mincostflow;
#[cfg(feature = "modint")]
pub mod modint;
#[cfg(feature = "scc")]
pub mod scc;
#[cfg(feature = "segtree")]
pub mod segtree;
#[cfg(feature = "string")]
pub mod string;
#[cfg(feature = "twosat")]
pub mod twosat;

#[cfg(any(feature = "convolution", feature = "lazysegtree", feature = "segtree"))]
pub(crate) mod internal_bit;
#[cfg(any(feature = "math", feature = "modint"))]
pub(crate) mod internal_math;
#[cfg(feature = "maxflow")]
pub(crate) mod internal_queue;
#[cfg(any(feature = "scc", feature = "twosat"))]
pub(crate) mod internal_scc;
#[cfg(any(feature = "maxflow", feature = "mincostflow", feature = "segtree"))]
pub(crate) mod internal_type_traits;

#[cfg(feature = "convolution")]
pub use convolution::{convolution, convolution_i64};
#[cfg(feature = "dsu")]
pub use dsu::Dsu;
#[cfg(feature = "fenwicktree")]
pub use fenwicktree::FenwickTree;
#[cfg(feature = "lazysegtree")]
pub use lazysegtree::{LazySegtree, MapMonoid};
#[cfg(feature = "math")]
pub use math::{crt, floor_sum, inv_mod, pow_mod};
#[cfg(feature = "maxflow")]
pub use maxflow::{Edge, MfGraph};
#[cfg(feature = "mincostflow")]
pub use mincostflow::MinCostFlowGraph;
#[cfg(feature = "modint")]
pub use modint::{
    Barrett, ButterflyCache, DefaultId, DynamicModInt, Id, Mod1000000007, Mod998244353, ModInt,
    ModInt1000000007, ModInt998244353, Modulus, RemEuclidU32, StaticModInt,
};
#[cfg(feature = "scc")]
pub use scc::SccGraph;
#[cfg(feature = "segtree")]
pub use segtree::{Additive, Max, Min, Monoid, Multiplicative, Segtree};
#[cfg(feature = "string")]
pub use string::{
    lcp_array, lcp_array_arbitrary, suffix_array, suffix_array_arbitrary, suffix_array_manual,
    z_algorithm, z_algorithm_arbitrary,
};
#[cfg(feature = "twosat")]
pub use twosat::TwoSat;

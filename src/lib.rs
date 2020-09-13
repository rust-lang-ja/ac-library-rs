mod convolution;
mod dsu;
mod fenwicktree;
mod lazysegtree;
mod math;
mod maxflow;
mod mincostflow;
mod modint;
mod scc;
mod segtree;
#[allow(clippy::many_single_char_names)]
mod string;
mod twosat;

pub(crate) mod internal_bit;
pub(crate) mod internal_math;
pub(crate) mod internal_queue;
pub(crate) mod internal_scc;
pub(crate) mod internal_type_traits;

pub use dsu::Dsu;
pub use fenwicktree::FenwickTree;
pub use lazysegtree::{LazySegtree, MapMonoid};
pub use math::{crt, floor_sum, inv_mod, pow_mod};
pub use maxflow::{Edge, MfGraph};
pub use mincostflow::MinCostFlowGraph;
pub use modint::{
    Barrett, DefaultId, DynamicModInt, Id, Mod1000000007, Mod998244353, ModInt, ModInt1000000007,
    ModInt998244353, Modulus, RemEuclidU32, StaticModInt,
};
pub use scc::SccGraph;
pub use segtree::{Max, Monoid, Segtree};
pub use string::{
    lcp_array, lcp_array_arbitrary, suffix_array, suffix_array_arbitrary, suffix_array_manual,
    z_algorithm, z_algorithm_arbitrary,
};

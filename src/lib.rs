#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_convolution as convolution;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_dsu as dsu;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_fenwicktree as fenwicktree;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_lazysegtree as lazysegtree;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_math as math;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_maxflow as maxflow;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_mincostflow as mincostflow;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_modint as modint;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_scc as scc;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_segtree as segtree;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_string as string;
#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
pub extern crate __acl_twosat as twosat;

// Crates like `num` re-export sub crates like this, but currently `cargo-simple-bundler` does not support inline modules.
//pub mod twosat {
//    #[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
//    extern crate __acl_twosat as twosat;
//    pub use self::twosat::*;
//}

pub use convolution::{convolution, convolution_i64};
pub use dsu::Dsu;
pub use fenwicktree::FenwickTree;
pub use lazysegtree::{LazySegtree, MapMonoid};
pub use math::{crt, floor_sum, inv_mod, pow_mod};
pub use maxflow::{Edge, MfGraph};
pub use mincostflow::MinCostFlowGraph;
pub use modint::{
    Barrett, ButterflyCache, DefaultId, DynamicModInt, Id, Mod1000000007, Mod998244353, ModInt,
    ModInt1000000007, ModInt998244353, Modulus, RemEuclidU32, StaticModInt,
};
pub use scc::SccGraph;
pub use segtree::{Additive, Max, Min, Monoid, Multiplicative, Segtree};
pub use string::{
    lcp_array, lcp_array_arbitrary, suffix_array, suffix_array_arbitrary, suffix_array_manual,
    z_algorithm, z_algorithm_arbitrary,
};
pub use twosat::TwoSat;

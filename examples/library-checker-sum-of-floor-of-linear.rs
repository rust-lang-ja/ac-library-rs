#[macro_use]
extern crate proconio as _;
#[macro_use]
extern crate proconio_derive as _;

use ac_library::math;

#[fastout]
fn main() {
    input! {
        nmabs: [(i64, i64, i64, i64)],
    }

    for (n, m, a, b) in nmabs {
        println!("{}", math::floor_sum(n, m, a, b));
    }
}

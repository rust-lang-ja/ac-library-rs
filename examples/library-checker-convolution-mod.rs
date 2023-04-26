#[macro_use]
extern crate proconio as _;

use ac_library::{convolution, modint::ModInt998244353 as Mint};
use std::fmt;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Mint; n],
        b: [Mint; m],
    }

    print_oneline(convolution::convolution(&a, &b));
}

fn print_oneline<I: IntoIterator<Item = T>, T: fmt::Display>(values: I) {
    println!(
        "{}",
        values
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

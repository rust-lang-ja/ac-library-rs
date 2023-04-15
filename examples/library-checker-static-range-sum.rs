#[macro_use]
extern crate proconio as _;
#[macro_use]
extern crate proconio_derive as _;

use ac_library::fenwicktree::FenwickTree;

#[allow(clippy::needless_collect)]
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        r#as: [u64; n],
        lrs: [(usize, usize); q],
    }

    let mut fenwick = FenwickTree::new(n, 0);
    for (i, a) in r#as.into_iter().enumerate() {
        fenwick.add(i, a);
    }
    for (l, r) in lrs {
        println!("{}", fenwick.sum(l..r));
    }
}

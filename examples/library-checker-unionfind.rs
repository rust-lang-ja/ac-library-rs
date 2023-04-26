#[macro_use]
extern crate proconio as _;

use ac_library::dsu::Dsu;

fn main() {
    input! {
        n: usize,
        queries: [(u8, usize, usize)],
    }

    let mut dsu = Dsu::new(n);
    for (kind, u, v) in queries {
        match kind {
            0 => {
                dsu.merge(u, v);
            }
            1 => println!("{}", u8::from(dsu.same(u, v))),
            _ => unreachable!(),
        }
    }
}

use ac_library::{Max, Segtree};
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let q: usize = input.next().unwrap().parse().unwrap();
    let mut segtree = Segtree::<Max<i32>>::new(n + 1);
    for i in 1..=n {
        segtree.set(i, input.next().unwrap().parse().unwrap());
    }
    for _ in 0..q {
        match input.next().unwrap().parse().unwrap() {
            1 => {
                let x = input.next().unwrap().parse().unwrap();
                let v = input.next().unwrap().parse().unwrap();
                segtree.set(x, v);
            }
            2 => {
                let l: usize = input.next().unwrap().parse().unwrap();
                let r: usize = input.next().unwrap().parse().unwrap();
                println!("{}", segtree.prod(l..=r));
            }
            3 => {
                let x = input.next().unwrap().parse().unwrap();
                let v = input.next().unwrap().parse().unwrap();
                println!("{}", segtree.max_right(x, |a| a < &v))
            }
            _ => {}
        }
    }
}

use ac_library_rs::FenwickTree;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let q = input.next().unwrap().parse().unwrap();
    let mut tree = FenwickTree::<u64>::new(n, 0);
    for i in 0..n {
        let a: u64 = input.next().unwrap().parse().unwrap();
        tree.add(i, a);
    }
    for _ in 0..q {
        match input.next().unwrap().parse().unwrap() {
            0 => {
                let p = input.next().unwrap().parse().unwrap();
                let x: u64 = input.next().unwrap().parse().unwrap();
                tree.add(p, x);
            }
            1 => {
                let l = input.next().unwrap().parse().unwrap();
                let r = input.next().unwrap().parse().unwrap();
                println!("{}", tree.sum(l, r));
            }
            _ => {}
        }
    }
}

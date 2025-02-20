// Check Problem Statement via https://atcoder.jp/contests/practice2/tasks/practice2_c
use ac_library_rs::floor_sum;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    for _ in 0..input.next().unwrap().parse().unwrap() {
        let n = input.next().unwrap().parse().unwrap();
        let m = input.next().unwrap().parse().unwrap();
        let a = input.next().unwrap().parse().unwrap();
        let b = input.next().unwrap().parse().unwrap();
        println!("{}", floor_sum(n, m, a, b));
    }
}

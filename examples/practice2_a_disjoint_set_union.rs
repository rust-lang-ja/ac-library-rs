use ac_library_rs::Dsu;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let mut dsu = Dsu::new(n);
    for _ in 0..input.next().unwrap().parse().unwrap() {
        let t = input.next().unwrap().parse().unwrap();
        let u = input.next().unwrap().parse().unwrap();
        let v = input.next().unwrap().parse().unwrap();
        match t {
            0 => {
                dsu.merge(u, v);
            }
            1 => println!("{}", dsu.same(u, v) as i32),
            _ => {}
        }
    }
}

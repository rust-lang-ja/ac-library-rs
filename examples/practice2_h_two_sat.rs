use ac_library_rs::TwoSat;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let d = input.next().unwrap().parse().unwrap();
    let xs = (0..2 * n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    let mut sat = TwoSat::new(2*n);
    for i in 0..2 * n {
        sat.add_clause(i, i % 2 == 0, i ^ 1, i % 2 == 0);
    }
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs[..i].iter().enumerate() {
            if (x - y).abs() < d {
                sat.add_clause(i, false, j, false);
            }
        }
    }
    if sat.satisfiable() {
        println!("Yes");
        let ans = sat.answer();
        for i in 0..n {
            println!("{}", xs[2 * i + ans[2 * i + 1] as usize]);
        }
    } else {
        println!("No");
    }
}

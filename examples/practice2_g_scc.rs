// Check Problem Statement via https://atcoder.jp/contests/practice2/tasks/practice2_g
use ac_library_rs::SccGraph;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let m = input.next().unwrap().parse().unwrap();
    let mut graph = SccGraph::new(n);
    for _ in 0..m {
        let a = input.next().unwrap().parse().unwrap();
        let b = input.next().unwrap().parse().unwrap();
        graph.add_edge(a, b);
    }
    let scc = graph.scc();
    println!("{}", scc.len());
    for cc in scc {
        print!("{}", cc.len());
        for v in cc {
            print!(" {}", v);
        }
        println!();
    }
}

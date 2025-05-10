// Check Problem Statement via https://atcoder.jp/contests/practice2/tasks/practice2_e
use ac_library_rs::MinCostFlowGraph;
use std::io::Read;

const MAX: i64 = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let k = input.next().unwrap().parse().unwrap();
    let a: Vec<Vec<i64>> = (0..n)
        .map(|_| input.by_ref().take(n).map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut graph = MinCostFlowGraph::new(102);
    for i in 0..n {
        for j in 0..n {
            graph.add_edge(i, 50 + j, 1, MAX - a[i][j]);
        }
    }
    for i in 0..n {
        graph.add_edge(100, i, k, 0);
        graph.add_edge(50 + i, 101, k, 0);
    }
    graph.add_edge(100, 101, n as i64 * k, MAX);

    let (max_flow, min_cost) = graph.flow(100, 101, n as i64 * k);
    println!("{}", max_flow * MAX - min_cost);

    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| match graph.get_edge(i * n + j).flow {
                    1 => 'X',
                    _ => '.',
                })
                .collect()
        })
        .for_each(|s: String| println!("{}", s));
}

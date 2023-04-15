use ac_library::MfGraph;
use std::io::Read;

#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
fn main() {
    const N: usize = 128;

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();
    let mut s = vec![vec![false; N]; N];
    for i in 1..=n {
        for (j, c) in (1..=m).zip(input.next().unwrap().chars()) {
            s[i][j] = c == '.';
        }
    }

    let dy = [1, 0, -1, 0];
    let dx = [0, 1, 0, -1];
    let c = "v>^<".chars().collect::<Vec<_>>();
    let mut graph = MfGraph::<i32>::new(128 * 128);

    for i in 1..=n {
        for j in 1..=m {
            if (i + j) % 2 == 0 {
                graph.add_edge(0, i * N + j, 1);
                for (dy, dx) in dy.iter().zip(dx.iter()) {
                    let k = (i as i32 + dy) as usize;
                    let l = (j as i32 + dx) as usize;
                    if s[i][j] && s[k][l] {
                        graph.add_edge(i * N + j, k * N + l, 1);
                    }
                }
            } else {
                graph.add_edge(i * N + j, 1, 1);
            }
        }
    }
    println!("{}", graph.flow(0, 1));
    let mut res: Vec<Vec<_>> = (1..=n)
        .map(|i| (1..=m).map(|j| if s[i][j] { '.' } else { '#' }).collect())
        .collect();
    for e in graph.edges() {
        let i = e.from / N;
        let j = e.from % N;
        let k = e.to / N;
        let l = e.to % N;
        if e.flow == 1 {
            for h in 0..4 {
                if i as i32 + dy[h] == k as i32 && j as i32 + dx[h] == l as i32 {
                    res[i - 1][j - 1] = c[h];
                    res[k - 1][l - 1] = c[h ^ 2];
                }
            }
        }
    }
    for s in res {
        println!("{}", s.iter().collect::<String>());
    }
}

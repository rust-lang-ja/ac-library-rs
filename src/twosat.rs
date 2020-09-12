use crate::internal_scc;

pub struct TwoSat {
    n: usize,
    scc: internal_scc::SccGraph,
    answer: Vec<bool>,
}
impl TwoSat {
    pub fn new(n: usize) -> Self {
        TwoSat {
            n,
            answer: vec![false; n],
            scc: internal_scc::SccGraph::new(2 * n),
        }
    }
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n && j < self.n);
        self.scc.add_edge(2 * i + !f as usize, 2 * j + g as usize);
        self.scc.add_edge(2 * j + !g as usize, 2 * i + f as usize);
    }
    pub fn satisfiable(&mut self) -> bool {
        let id = self.scc.scc_ids().1;
        for i in 0..self.n {
            if id[2 * i] == id[2 * i + 1] {
                return false;
            }
            self.answer[i] = id[2 * i] < id[2 * i + 1];
        }
        true
    }
    pub fn answer(&self) -> &[bool] {
        &self.answer
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::many_single_char_names)]
    use super::*;
    #[test]
    fn solve_alpc_h_sample1() {
        // https://atcoder.jp/contests/practice2/tasks/practice2_h

        let (n, d) = (3, 2);
        let x = [1, 2, 0i32];
        let y = [4, 5, 6];

        let mut t = TwoSat::new(n);

        for i in 0..n {
            for j in i + 1..n {
                if (x[i] - x[j]).abs() < d {
                    t.add_clause(i, false, j, false);
                }
                if (x[i] - y[j]).abs() < d {
                    t.add_clause(i, false, j, true);
                }
                if (y[i] - x[j]).abs() < d {
                    t.add_clause(i, true, j, false);
                }
                if (y[i] - y[j]).abs() < d {
                    t.add_clause(i, true, j, true);
                }
            }
        }
        assert!(t.satisfiable());
        let answer = t.answer();
        let mut res = vec![];
        for (i, &v) in answer.iter().enumerate() {
            if v {
                res.push(x[i])
            } else {
                res.push(y[i]);
            }
        }
        assert_eq!(res, [4, 2, 0]);
    }

    #[test]
    fn solve_alpc_h_sample2() {
        // https://atcoder.jp/contests/practice2/tasks/practice2_h

        let (n, d) = (3, 3);
        let x = [1, 2, 0i32];
        let y = [4, 5, 6];

        let mut t = TwoSat::new(n);

        for i in 0..n {
            for j in i + 1..n {
                if (x[i] - x[j]).abs() < d {
                    t.add_clause(i, false, j, false);
                }
                if (x[i] - y[j]).abs() < d {
                    t.add_clause(i, false, j, true);
                }
                if (y[i] - x[j]).abs() < d {
                    t.add_clause(i, true, j, false);
                }
                if (y[i] - y[j]).abs() < d {
                    t.add_clause(i, true, j, true);
                }
            }
        }
        assert!(!t.satisfiable());
    }
}

//! A 2-SAT Solver.
use crate::internal_scc;

/// A 2-SAT Solver.
///
/// For variables $x_0, x_1, \ldots, x_{N - 1}$ and clauses with from
///
/// \\[
///   (x_i = f) \lor (x_j = g)
/// \\]
///
/// it decides whether there is a truth assignment that satisfies all clauses.
///
/// # Example
///
/// ```
/// #![allow(clippy::many_single_char_names)]
///
/// use ac_library::TwoSat;
/// use proconio::{input, marker::Bytes, source::once::OnceSource};
///
/// input! {
///     from OnceSource::from(
///         "3\n\
///          3\n\
///          a b\n\
///          !b c\n\
///          !a !a\n",
///     ),
///     n: usize,
///     pqs: [(Bytes, Bytes)],
/// }
///
/// let mut twosat = TwoSat::new(n);
///
/// for (p, q) in pqs {
///     fn parse(s: &[u8]) -> (usize, bool) {
///         match *s {
///             [c] => ((c - b'a').into(), true),
///             [b'!', c] => ((c - b'a').into(), false),
///             _ => unreachable!(),
///         }
///     }
///     let ((i, f), (j, g)) = (parse(&p), parse(&q));
///     twosat.add_clause(i, f, j, g);
/// }
///
/// assert!(twosat.satisfiable());
/// assert_eq!(twosat.answer(), [false, true, true]);
/// ```
pub struct TwoSat {
    n: usize,
    scc: internal_scc::SccGraph,
    answer: Vec<bool>,
}
impl TwoSat {
    /// Creates a new `TwoSat` of `n` variables and 0 clauses.
    ///
    /// # Constraints
    ///
    /// - $0 \leq n \leq 10^8$
    ///
    /// # Complexity
    ///
    /// - $O(n)$
    pub fn new(n: usize) -> Self {
        TwoSat {
            n,
            answer: vec![false; n],
            scc: internal_scc::SccGraph::new(2 * n),
        }
    }
    /// Adds a clause $(x_i = f) \lor (x_j = g)$.
    ///
    /// # Constraints
    ///
    /// - $0 \leq i < n$
    /// - $0 \leq j < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraints are not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(1)$ amortized
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n && j < self.n);
        self.scc.add_edge(2 * i + !f as usize, 2 * j + g as usize);
        self.scc.add_edge(2 * j + !g as usize, 2 * i + f as usize);
    }
    /// Returns whether there is a truth assignment that satisfies all clauses.
    ///
    /// # Complexity
    ///
    /// - $O(n + m)$ where $m$ is the number of added clauses
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
    /// Returns a truth assignment that satisfies all clauses **of the last call of [`satisfiable`]**.
    ///
    /// # Constraints
    ///
    /// - [`satisfiable`] is called after adding all clauses and it has returned `true`.
    ///
    /// # Complexity
    ///
    /// - $O(n)$
    ///
    /// [`satisfiable`]: #method.satisfiable
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

        //Check the min distance between flags
        res.sort_unstable();
        let mut min_distance = i32::max_value();
        for i in 1..res.len() {
            min_distance = std::cmp::min(min_distance, res[i] - res[i - 1]);
        }
        assert!(min_distance >= d);
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

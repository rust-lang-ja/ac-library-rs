//! A `SccGraph` is a directed graph that calculates strongly connected components (SCC) in $O(|V| + |E|)$.

use crate::internal_scc;

/// An `SccGraph` is a directed graph that calculates strongly connected components (SCC) in $O(|V| + |E|)$.
///
/// # Example
///
/// ```
/// use ac_library::SccGraph;
/// use proconio::{input, source::once::OnceSource};
///
/// input! {
///     from OnceSource::from(
///         "5\n\
///          5\n\
///          0 1\n\
///          1 2\n\
///          2 0\n\
///          0 3\n\
///          3 4\n",
///     ),
///     n: usize,
///     abs: [(usize, usize)],
/// }
///
/// let mut graph = SccGraph::new(n);
/// for (a, b) in abs {
///     graph.add_edge(a, b);
/// }
///
/// assert_eq!(graph.scc(), [&[0, 1, 2][..], &[3], &[4]]);
/// ```
pub struct SccGraph {
    internal: internal_scc::SccGraph,
}

impl SccGraph {
    /// Creates a new `SccGraph` with `n` edges.
    ///
    /// # Constraints
    ///
    /// - $0 \leq n \leq 10^8$
    ///
    /// # Complexity
    ///
    /// - $O(n)$
    pub fn new(n: usize) -> Self {
        SccGraph {
            internal: internal_scc::SccGraph::new(n),
        }
    }

    /// Adds a directed edge from the vertex `from` to the vertex `to`.
    ///
    /// # Constraints
    ///
    /// - $0 \leq$ `from` $< n$
    /// - $0 \leq$ `to` $< n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraints are not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(1)$ amortized
    pub fn add_edge(&mut self, from: usize, to: usize) {
        let n = self.internal.num_vertices();
        assert!(from < n);
        assert!(to < n);
        self.internal.add_edge(from, to);
    }

    /// Calculates the strongly connected components (SCC) of directed graphs in $O(|V| + |E|)$.
    ///
    /// Returns the list of the "list of the vertices" that satisfies the following.
    ///
    /// - Each vertex is in exactly one "list of the vertices".
    /// - Each "list of the vertices" corresponds to the vertex set of a strongly connected component. The order of the vertices in the list is undefined.
    /// - The list of "list of the vertices" are sorted in topological order, i.e., for two vertices $u$, $v$ in different strongly connected components, if there is a directed path from $u$ to $v$, the list containing $u$ appears earlier than the list containing $v$.
    ///
    /// # Complexity
    ///
    /// - $O(n + m)$ where $m$ is the number of added edges
    pub fn scc(&self) -> Vec<Vec<usize>> {
        self.internal.scc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scc_simple() {
        let mut graph = SccGraph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);
        let scc = graph.scc();
        assert_eq!(scc.len(), 1);
    }

    #[test]
    fn test_scc_self_loop() {
        let mut graph = SccGraph::new(2);
        graph.add_edge(0, 0);
        graph.add_edge(0, 0);
        graph.add_edge(1, 1);
        let scc = graph.scc();
        assert_eq!(scc.len(), 2);
    }

    #[test]
    fn solve_alpc_g_sample1() {
        // https://atcoder.jp/contests/practice2/tasks/practice2_g
        let n: usize = 6;
        let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

        let mut graph = SccGraph::new(n);
        for (u, v) in edges.into_iter() {
            graph.add_edge(u, v);
        }

        let scc = graph.scc();
        assert_eq!(scc, vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
    }
}

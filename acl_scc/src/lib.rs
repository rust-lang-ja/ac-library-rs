#[cfg_attr(cargo_equip, cargo_equip::use_another_lib)]
extern crate __acl_internal_scc as internal_scc;

pub struct SccGraph {
    internal: internal_scc::SccGraph,
}

impl SccGraph {
    pub fn new(n: usize) -> Self {
        SccGraph {
            internal: internal_scc::SccGraph::new(n),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let n = self.internal.num_vertices();
        assert!(from < n);
        assert!(to < n);
        self.internal.add_edge(from, to);
    }

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

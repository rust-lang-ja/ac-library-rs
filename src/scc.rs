use crate::internal_scc;

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


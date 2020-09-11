use crate::internal_scc;

pub struct TwoSAT {
    n: usize,
    scc: internal_scc::SccGraph,
    answer: Vec<bool>,
}
impl TwoSAT {
    pub fn new(n: usize) -> Self {
        TwoSAT {
            n,
            answer: vec![false; n],
            scc: internal_scc::SccGraph::new(2 * n),
        }
    }
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n && j < self.n);
        self.scc
            .add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.scc
            .add_edge(2 * j + if f { 0 } else { 1 }, 2 * i + if g { 1 } else { 0 });
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
    pub fn answer(&self) -> &Vec<bool> {
        &self.answer
    }
}

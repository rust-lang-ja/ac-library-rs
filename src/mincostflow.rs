pub struct MinCostFlowEdge<T> {
    pub from: usize,
    pub to: usize,
    pub cap: T,
    pub flow: T,
    pub cost: T,
}

pub struct MinCostFlowGraph<T> {
    pos: Vec<(usize, usize)>,
    g: Vec<Vec<_Edge<T>>>,
    cost_sum: T,
}

impl<T> MinCostFlowGraph<T>
where
    T: Number + std::ops::Neg<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            pos: vec![],
            g: (0..n).map(|_| vec![]).collect(),
            cost_sum: T::zero(),
        }
    }

    pub fn get_edge(&self, i: usize) -> MinCostFlowEdge<T> {
        assert!(i < self.pos.len());
        let e = &self.g[self.pos[i].0][self.pos[i].1];
        let re = &self.g[e.to][e.rev];
        MinCostFlowEdge {
            from: self.pos[i].0,
            to: e.to,
            cap: e.cap + re.cap,
            flow: re.cap,
            cost: e.cost,
        }
    }

    pub fn edges(&self) -> Vec<MinCostFlowEdge<T>> {
        let m = self.pos.len();
        let mut result = vec![];
        for i in 0..m {
            result.push(self.get_edge(i));
        }
        result
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        assert!(from < self.g.len());
        assert!(to < self.g.len());
        assert_ne!(from, to);
        self.pos.push((from, self.g[from].len()));
        self.cost_sum += cost;

        let rev = self.g[to].len();
        self.g[from].push(_Edge { to, rev, cap, cost });

        let rev = self.g[from].len() - 1;
        self.g[to].push(_Edge {
            to: from,
            rev,
            cap: T::zero(),
            cost: -cost,
        });

        self.pos.len() - 1
    }

    /// Returns (maximum flow, cost)
    pub fn flow(&mut self, source: usize, sink: usize, flow_limit: T) -> (T, T) {
        *self.slope(source, sink, flow_limit).last().unwrap()
    }

    pub fn slope(&mut self, source: usize, sink: usize, flow_limit: T) -> Vec<(T, T)> {
        let n = self.g.len();
        assert!(source < n);
        assert!(sink < n);
        assert_ne!(source, sink);

        let mut dual = vec![T::zero(); n];
        let mut prev_v = vec![0; n];
        let mut prev_e = vec![0; n];
        let mut flow = T::zero();
        let mut cost = T::zero();
        let mut prev_cost: Option<T> = None;
        let mut result = vec![(flow, cost)];
        while flow < flow_limit {
            if !self.dual_ref(source, sink, &mut dual, &mut prev_v, &mut prev_e) {
                break;
            }
            let mut c = flow_limit - flow;

            let mut v = sink;
            while v != source {
                c = std::cmp::min(c, self.g[prev_v[v]][prev_e[v]].cap);
                v = prev_v[v];
            }

            let mut v = sink;
            while v != source {
                self.g[prev_v[v]][prev_e[v]].cap -= c;
                let rev = self.g[prev_v[v]][prev_e[v]].rev;
                self.g[v][rev].cap += c;
                v = prev_v[v];
            }

            let d = -dual[source];
            flow += c;
            cost += d * c;
            if prev_cost == Some(d) {
                assert!(result.pop().is_some());
            }
            result.push((flow, cost));
            prev_cost = Some(cost);
        }
        result
    }

    fn dual_ref(
        &self,
        source: usize,
        sink: usize,
        dual: &mut [T],
        pv: &mut [usize],
        pe: &mut [usize],
    ) -> bool {
        let n = self.g.len();
        let mut dist = vec![self.cost_sum; n];
        let mut vis = vec![false; n];

        let mut que = std::collections::BinaryHeap::new();
        dist[source] = T::zero();
        que.push((std::cmp::Reverse(T::zero()), source));
        while let Some((_, v)) = que.pop() {
            if vis[v] {
                continue;
            }
            vis[v] = true;
            if v == sink {
                break;
            }

            for (i, e) in self.g[v].iter().enumerate() {
                if vis[e.to] || e.cap == T::zero() {
                    continue;
                }

                let cost = e.cost - dual[e.to] + dual[v];
                if dist[e.to] - dist[v] > cost {
                    dist[e.to] = dist[v] + cost;
                    pv[e.to] = v;
                    pe[e.to] = i;
                    que.push((std::cmp::Reverse(dist[e.to]), e.to));
                }
            }
        }

        if !vis[sink] {
            return false;
        }

        for v in 0..n {
            if !vis[v] {
                continue;
            }
            dual[v] -= dist[sink] - dist[v];
        }
        true
    }
}

struct _Edge<T> {
    to: usize,
    rev: usize,
    cap: T,
    cost: T,
}

// TODO migrate to common type
pub trait Number:
    Copy
    + Ord
    + std::ops::Not<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::ops::RemAssign
    + std::ops::BitOr<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitXor<Output = Self>
    + std::ops::BitOrAssign
    + std::ops::BitAndAssign
    + std::ops::BitXorAssign
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + std::ops::ShlAssign
    + std::ops::ShrAssign
    + std::fmt::Display
    + std::fmt::Debug
    + std::fmt::Binary
    + std::fmt::Octal
{
    fn zero() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Number for i64 {
        fn zero() -> Self {
            0
        }
    }

    #[test]
    fn test_min_cost_flow() {
        let mut graph = MinCostFlowGraph::new(4);
        graph.add_edge(0, 1, 2, 1);
        graph.add_edge(0, 2, 1, 2);
        graph.add_edge(1, 2, 1, 1);
        graph.add_edge(1, 3, 1, 3);
        graph.add_edge(2, 3, 2, 1);
        let (flow, cost) = graph.flow(0, 3, 2);
        assert_eq!(flow, 2);
        assert_eq!(cost, 6);
    }
}

#![allow(dead_code)]
use crate::internal_queue::SimpleQueue;
use std::cmp::min;
use std::iter;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Maybe it should be in a separate module
pub trait MfCapacity:
    Copy + Ord + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
{
    fn zero() -> Self;
    fn max_value() -> Self;
}

impl MfCapacity for i32 {
    fn zero() -> Self {
        0
    }
    fn max_value() -> Self {
        std::i32::MAX
    }
}

impl MfCapacity for i64 {
    fn zero() -> Self {
        0
    }
    fn max_value() -> Self {
        std::i64::MAX
    }
}

impl MfCapacity for u32 {
    fn zero() -> Self {
        0
    }
    fn max_value() -> Self {
        std::u32::MAX
    }
}

impl MfCapacity for u64 {
    fn zero() -> Self {
        0
    }
    fn max_value() -> Self {
        std::u64::MAX
    }
}

impl<Cap> MfGraph<Cap>
where
    Cap: MfCapacity,
{
    pub fn new(n: usize) -> MfGraph<Cap> {
        MfGraph {
            _n: n,
            pos: Vec::new(),
            g: iter::repeat_with(Vec::new).take(n).collect(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
        assert!(from < self._n);
        assert!(to < self._n);
        assert!(Cap::zero() <= cap);
        let m = self.pos.len();
        self.pos.push((from, self.g[from].len()));
        let rev = self.g[to].len() + if from == to { 1 } else { 0 };
        self.g[from].push(_Edge { to, rev, cap });
        let rev = self.g[from].len() - 1;
        self.g[to].push(_Edge {
            to: from,
            rev,
            cap: Cap::zero(),
        });
        m
    }
}

pub struct Edge<Cap> {
    pub from: usize,
    pub to: usize,
    pub cap: Cap,
    pub flow: Cap,
}

impl<Cap> MfGraph<Cap>
where
    Cap: MfCapacity,
{
    pub fn get_edge(&self, i: usize) -> Edge<Cap> {
        let m = self.pos.len();
        assert!(i < m);
        let _e = &self.g[self.pos[i].0][self.pos[i].1];
        let _re = &self.g[_e.to][_e.rev];
        Edge {
            from: self.pos[i].0,
            to: _e.to,
            cap: _e.cap + _re.cap,
            flow: _re.cap,
        }
    }
    pub fn edges(&self) -> Vec<Edge<Cap>> {
        let m = self.pos.len();
        (0..m).map(|i| self.get_edge(i)).collect()
    }
    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap) {
        let m = self.pos.len();
        assert!(i < m);
        assert!(Cap::zero() <= new_flow && new_flow <= new_cap);
        let (to, rev) = {
            let _e = &mut self.g[self.pos[i].0][self.pos[i].1];
            _e.cap = new_cap - new_flow;
            (_e.to, _e.rev)
        };
        let _re = &mut self.g[to][rev];
        _re.cap = new_flow;
    }

    pub fn flow(&mut self, s: usize, t: usize) -> Cap {
        self.flow_with_capacity(s, t, Cap::max_value())
    }
    pub fn flow_with_capacity(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        let n_ = self._n;
        assert!(s < n_);
        assert!(t < n_);

        let mut calc = FlowCalculator {
            graph: self,
            s,
            t,
            flow_limit,
            level: vec![0; n_],
            iter: vec![0; n_],
            que: SimpleQueue::default(),
        };

        let mut flow = Cap::zero();
        while flow < flow_limit {
            calc.bfs();
            if calc.level[t] == -1 {
                break;
            }
            calc.iter.iter_mut().for_each(|e| *e = 0);
            while flow < flow_limit {
                let f = calc.dfs(t, flow_limit - flow);
                if f == Cap::zero() {
                    break;
                }
                flow += f;
            }
        }
        flow
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self._n];
        let mut que = SimpleQueue::default();
        que.push(s);
        while !que.empty() {
            let &p = que.front().unwrap();
            que.pop();
            visited[p] = true;
            for e in &self.g[p] {
                if e.cap != Cap::zero() && !visited[e.to] {
                    visited[e.to] = true;
                    que.push(e.to);
                }
            }
        }
        visited
    }
}

struct FlowCalculator<'a, Cap> {
    graph: &'a mut MfGraph<Cap>,
    s: usize,
    t: usize,
    flow_limit: Cap,
    level: Vec<i32>,
    iter: Vec<usize>,
    que: SimpleQueue<usize>,
}

impl<Cap> FlowCalculator<'_, Cap>
where
    Cap: MfCapacity,
{
    fn bfs(&mut self) {
        self.level.iter_mut().for_each(|e| *e = -1);
        self.level[self.s] = 0;
        self.que.clear();
        self.que.push(self.s);
        while !self.que.empty() {
            let v = *self.que.front().unwrap();
            self.que.pop();
            for e in &self.graph.g[v] {
                if e.cap == Cap::zero() || self.level[e.to] >= 0 {
                    continue;
                }
                self.level[e.to] = self.level[v] + 1;
                if e.to == self.t {
                    return;
                }
                self.que.push(e.to);
            }
        }
    }
    fn dfs(&mut self, v: usize, up: Cap) -> Cap {
        if v == self.s {
            return up;
        }
        let mut res = Cap::zero();
        let level_v = self.level[v];
        for i in self.iter[v]..self.graph.g[v].len() {
            self.iter[v] = i;
            let &_Edge {
                to: e_to,
                rev: e_rev,
                ..
            } = &self.graph.g[v][i];
            if level_v <= self.level[e_to] || self.graph.g[e_to][e_rev].cap == Cap::zero() {
                continue;
            }
            let d = self.dfs(e_to, min(up - res, self.graph.g[e_to][e_rev].cap));
            if d <= Cap::zero() {
                continue;
            }
            self.graph.g[v][i].cap += d;
            self.graph.g[e_to][e_rev].cap -= d;
            res += d;
            if res == up {
                break;
            }
        }
        self.iter[v] = self.graph.g[v].len();
        res
    }
}

#[derive(Default)]
pub struct MfGraph<Cap> {
    _n: usize,
    pos: Vec<(usize, usize)>,
    g: Vec<Vec<_Edge<Cap>>>,
}

struct _Edge<Cap> {
    to: usize,
    rev: usize,
    cap: Cap,
}

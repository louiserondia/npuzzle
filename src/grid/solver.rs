use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

use crate::complex::Complex;

use super::Grid;

#[derive(Debug, Clone)]
pub struct State {
    grid: Grid,
    cost: i32,
    pub path: Vec<Complex<i32>>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost + self.path.len() as i32).cmp(&(other.cost + other.path.len() as i32))
        // self.cost.cmp(&other.cost)
    }
}

pub struct Solver {
    open_set: BinaryHeap<Reverse<State>>,
    pub closed_set: HashMap<Vec<i32>, State>,
    size: i32,
    target_m: HashMap<i32, Complex<i32>>,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        let mut solver = Self {
            size: grid.size,
            target_m: Grid::create_solved_grid(grid.size)
                .v
                .iter()
                .enumerate()
                .map(|(i, &v)| (i as i32, v))
                .map(|(i, v)| (v, Complex::new(i % grid.size, i / grid.size)))
                .collect(),
            open_set: BinaryHeap::new(),
            closed_set: HashMap::new(),
        };
        let state = State {
            grid,
            cost: 0,
            path: Vec::new(),
        };
        solver.open_set.push(Reverse(state));
        solver.open_set.peek_mut().unwrap().0.cost =
            solver.hcost(&solver.open_set.peek().unwrap().0.grid);
        solver
    }

    fn smart_hcost(&self, state: &State, d: Complex<i32>) -> i32 {
        let mut c = state.cost;
        c -= Complex::manhattan_dist(state.grid.zero, self.target_m[&0]);
        c -= Complex::manhattan_dist(
            state.grid.zero + d,
            self.target_m[state.grid.get_cell_ref(state.grid.zero + d)],
        );
        c += Complex::manhattan_dist(state.grid.zero + d, self.target_m[&0]);
        c += Complex::manhattan_dist(
            state.grid.zero,
            self.target_m[state.grid.get_cell_ref(state.grid.zero + d)],
        );
        c
    }

    fn hcost(&self, grid: &Grid) -> i32 {
        let mut c = 0;
        for y in 0..grid.size {
            for x in 0..grid.size {
                let p = Complex::new(x, y);
                c += Complex::manhattan_dist(p, self.target_m[grid.get_cell_ref(p)]);
            }
        }
        c
    }

    pub fn solve(&mut self) {
        let target = Grid::create_solved_grid(self.size);
        while !self.closed_set.contains_key(&target.v) {
            let s = self.open_set.pop().unwrap().0;
            // println!("{}", self.closed_set.len());
            let dirs = Grid::dirs();
            let ops = dirs.iter().filter(|d| s.grid.is_op_legal(**d));
            for op in ops {
                let mut ns = s.clone();
                ns.path.push(*op);
                ns.grid.op(*op);
                if self.closed_set.contains_key(&ns.grid.v) {
                    continue;
                }
                ns.cost = self.smart_hcost(&s, *op);
                self.open_set.push(Reverse(ns));
            }
            self.closed_set.insert(s.grid.v.clone(), s);
        }
    }
}

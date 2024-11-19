use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt,
};

use super::Grid;
use crate::complex::Complex;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct State {
    grid: Grid,
    cost: i32,
    path: Vec<Complex<i32>>,
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
    }
}

pub struct Res {
    time_complexity: usize,
    size_complexity: usize,
    sequence: Vec<Complex<i32>>,
}

pub fn print_res(res: Res, g: &Grid) {
    let mut g = g.clone();
    println!("-------------------------------");
    println!("sequence :\n");
    println!("{:?}\n", g);
    for i in res.sequence.clone() {
        g.op(i);
        println!("{:?}\n", g);
    }
    println!("complexity in time : {:?}", res.time_complexity);
    println!("complexity in size : {:?}", res.size_complexity);
    println!("total number of operations : {:?}", res.sequence.len());
    println!("-------------------------------");
}

#[derive(Clone, Copy)]
pub enum Heuristic {
    Manhattan,
    Euclidian,
    Misplaced,
}

impl Heuristic {
    fn dist(&self, z0: Complex<i32>, z1: Complex<i32>) -> i32 {
        match self {
            Self::Manhattan => (z0.x - z1.x).abs() + (z0.y - z1.y).abs(),
            Self::Euclidian => (((z0.x - z1.x).pow(2) + (z0.y - z1.y).pow(2)) as f64)
                .sqrt()
                .floor() as i32,
            Self::Misplaced => (z0 != z1) as i32,
        }
    }
}

struct Hcost {
    target_m: HashMap<i32, Complex<i32>>,
    h: Heuristic,
}

impl Hcost {
    fn new(size: i32, h: Heuristic) -> Self {
        Self {
            target_m: Grid::create_solved_grid(size)
                .v
                .iter()
                .enumerate()
                .map(|(i, &v)| (i as i32, v))
                .map(|(i, v)| (v, Complex::new(i % size, i / size)))
                .collect(),
            h,
        }
    }

    fn smart_hcost(&self, state: &State, d: Complex<i32>) -> i32 {
        let mut c = state.cost;
        c -= self.h.dist(state.grid.zero, self.target_m[&0]);
        c -= self.h.dist(
            state.grid.zero + d,
            self.target_m[state.grid.get_cell_ref(state.grid.zero + d)],
        );
        c += self.h.dist(state.grid.zero + d, self.target_m[&0]);
        c += self.h.dist(
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
                c += self.h.dist(p, self.target_m[grid.get_cell_ref(p)]);
            }
        }
        c
    }
}

fn unroll(grid: &Grid) -> Vec<i32> {
    let mut v = vec![0; grid.size.pow(2) as usize];
    let solved = Grid::create_solved_grid(grid.size);

    for (i, n) in grid.v.iter().enumerate() {
        v[((solved.v[i] - 1).rem_euclid(grid.size.pow(2))) as usize] = *n;
    }
    v
}

fn is_solvable(grid: &Grid) -> bool {
    let mut inversions = 0;
    let g = unroll(grid);

    for (i, n1) in g.iter().enumerate() {
        for n2 in g.iter().skip(i + 1) {
            if n1 > n2 && *n2 != 0 {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}

#[derive(Debug)]
pub struct UnsolvableError;

impl fmt::Display for UnsolvableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsolvable")
    }
}

impl Error for UnsolvableError {}

pub fn solve(grid: &Grid, h: Heuristic) -> Result<Res, UnsolvableError> {
    if !is_solvable(&grid) {
        return Err(UnsolvableError);
    }
    let mut res = Res {
        time_complexity: 0,
        size_complexity: 0,
        sequence: Vec::new(),
    };
    let mut open_set: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut closed_set: HashMap<Vec<i32>, State> = HashMap::new();
    let hcost = Hcost::new(grid.size, h);
    let state = State {
        grid: grid.clone(),
        cost: 0,
        path: Vec::new(),
    };
    open_set.push(Reverse(state));
    open_set.peek_mut().unwrap().0.cost = hcost.hcost(&open_set.peek().unwrap().0.grid);

    let target = Grid::create_solved_grid(grid.size);
    while !closed_set.contains_key(&target.v) {
        let s = open_set.pop().unwrap().0;
        res.time_complexity += 1;
        res.size_complexity = res.size_complexity.max(open_set.len() + closed_set.len());
        let dirs = Grid::dirs();
        let ops = dirs.iter().filter(|d| s.grid.is_op_legal(**d));
        for op in ops {
            let mut ns = s.clone();
            ns.path.push(*op);
            ns.grid.op(*op);
            if closed_set.contains_key(&ns.grid.v) {
                continue;
            }
            ns.cost = hcost.smart_hcost(&s, *op);
            open_set.push(Reverse(ns));
        }
        closed_set.insert(s.grid.v.clone(), s);
    }
    res.sequence = closed_set[&target.v].path.clone();
    Ok(res)
}

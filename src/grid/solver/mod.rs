use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
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
    h_cost: i32,
    g_cost: i32,
    last_op: Option<Complex<i32>>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.h_cost == other.h_cost
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
        match (
            self.h_cost + self.g_cost as i32,
            other.h_cost + other.g_cost as i32,
        ) {
            (c1, c2) if c1 != c2 => c1.cmp(&c2),
            _ => self.h_cost.cmp(&other.h_cost),
        }
    }
}

pub struct Res {
    time_complexity: usize,
    size_complexity: usize,
    sequence: Vec<Complex<i32>>,
    grid: Grid,
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut g = self.grid.clone();
        write!(f, "-------------------------------\n")?;
        write!(f, "sequence :\n\n")?;
        write!(f, "{:?}\n\n", g)?;
        for i in self.sequence.clone() {
            g.op(i);
            write!(f, "{:?}\n\n", g)?;
        }
        write!(f, "complexity in time : {:?}\n", self.time_complexity)?;
        write!(f, "complexity in size : {:?}\n", self.size_complexity)?;
        write!(
            f,
            "total number of operations : {:?}\n",
            self.sequence.len()
        )?;
        write!(f, "-------------------------------\n")
    }
}

#[derive(Clone, Copy)]
pub enum Heuristic {
    Manhattan,
    Euclidian,
    Misplaced,
    Zero,
}

impl Heuristic {
    fn dist(&self, z0: Complex<i32>, z1: Complex<i32>) -> i32 {
        match self {
            Self::Manhattan => (z0.x - z1.x).abs() + (z0.y - z1.y).abs(),
            Self::Euclidian => (((z0.x - z1.x).pow(2) + (z0.y - z1.y).pow(2)) as f64)
                .sqrt()
                .floor() as i32,
            Self::Misplaced => (z0 != z1) as i32,
            Self::Zero => 0,
        }
    }
}

#[derive(Clone)]
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
        let mut c = state.h_cost;
        c -= self.h.dist(
            state.grid.zero + d,
            self.target_m[state.grid.get_cell_ref(state.grid.zero + d)],
        );
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
                if *grid.get_cell_ref(p) != 0 {
                    c += self.h.dist(p, self.target_m[grid.get_cell_ref(p)]);
                }
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

pub struct UnsolvableError;

impl fmt::Display for UnsolvableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsolvable")
    }
}

impl Error for UnsolvableError {}

impl fmt::Debug for UnsolvableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

#[derive(Clone, Copy)]
pub enum Algo {
    Astar,
    IDAstar,
}

pub fn solve(grid: &Grid, h: Heuristic, algo: Algo) -> Result<Res, UnsolvableError> {
    if !is_solvable(grid) {
        return Err(UnsolvableError);
    }
    Ok(match algo {
        Algo::Astar => astar(grid, h),
        Algo::IDAstar => idastar(grid, h),
    })
}

fn astar(grid: &Grid, h: Heuristic) -> Res {
    let mut res = Res {
        time_complexity: 0,
        size_complexity: 0,
        sequence: Vec::new(),
        grid: grid.clone(),
    };
    let mut open_set: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut open_g: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut closed_set: HashMap<Vec<i32>, State> = HashMap::new();
    let hcost = Hcost::new(grid.size, h);
    {
        let state = State {
            grid: grid.clone(),
            h_cost: 0,
            g_cost: 0,
            last_op: None,
        };
        open_set.push(Reverse(state));
        open_g.insert(grid.v.clone(), 0);
    }
    open_set.peek_mut().unwrap().0.h_cost = hcost.hcost(&open_set.peek().unwrap().0.grid);

    let target = Grid::create_solved_grid(grid.size);
    while !closed_set.contains_key(&target.v) {
        let s = open_set.pop().unwrap().0;
        open_g.remove(&s.grid.v);
        res.time_complexity += 1;
        res.size_complexity = res.size_complexity.max(open_set.len() + closed_set.len());
        let dirs = Grid::dirs();
        let ops = dirs.iter().filter(|d| s.grid.is_op_legal(**d));
        for op in ops {
            let mut ns = s.clone();
            ns.grid.op(*op);
            if closed_set.contains_key(&ns.grid.v) {
                continue;
            }
            ns.g_cost += 1;
            ns.last_op = Some(*op);
            ns.h_cost = hcost.smart_hcost(&s, *op);
            if open_g.contains_key(&ns.grid.v) && open_g[&ns.grid.v] < ns.g_cost {
                continue;
            }
            open_g.insert(ns.grid.v.clone(), ns.g_cost);
            open_set.push(Reverse(ns));
        }
        closed_set.insert(s.grid.v.clone(), s);
    }
    let mut g = closed_set[&target.v].grid.clone();
    while let Some(op) = closed_set[&g.v].last_op {
        res.sequence.push(op);
        g.op(op * -1);
    }
    res.sequence.reverse();
    res
}

fn idastar(grid: &Grid, h: Heuristic) -> Res {
    struct Env {
        hcost: Hcost,
        target: Grid,
        lim: i32,
        seen: HashSet<Vec<i32>>,
        res: Res,
    }

    let hcost = Hcost::new(grid.size, h);
    let mut env = Env {
        target: Grid::create_solved_grid(grid.size),
        lim: hcost.hcost(grid),
        hcost,
        seen: HashSet::new(),
        res: Res {
            size_complexity: 0,
            time_complexity: 0,
            sequence: Vec::new(),
            grid: grid.clone(),
        },
    };
    env.seen.insert(grid.v.clone());

    enum Output {
        Found,
        Limit(Option<i32>),
    }

    fn compute(env: &mut Env, grid: &Grid, g: i32) -> Output {
        let f = g + env.hcost.hcost(grid);
        if f > env.lim {
            return Output::Limit(Some(f));
        }

        if grid.v == env.target.v {
            return Output::Found;
        }

        let mut min_lim: Option<i32> = None;
        let dirs = Grid::dirs();
        let ops = dirs.iter().filter(|d| grid.is_op_legal(**d));
        for op in ops {
            env.res.time_complexity += 1;
            let mut ngrid = grid.clone();
            ngrid.op(*op);
            if env.seen.contains(&ngrid.v) {
                continue;
            }
            env.res.sequence.push(*op);
            env.seen.insert(ngrid.v.clone());
            env.res.size_complexity = env.res.size_complexity.max(env.res.sequence.len());
            match compute(env, &ngrid, g + 1) {
                Output::Limit(Some(lim)) => min_lim = Some(min_lim.unwrap_or(lim).min(lim)),
                Output::Limit(None) => {}
                Output::Found => return Output::Found,
            }
            env.res.sequence.pop();
            env.seen.remove(&ngrid.v);
        }
        Output::Limit(min_lim)
    }

    while let Output::Limit(lim) = compute(&mut env, grid, 0) {
        env.lim = lim.unwrap();
    }
    env.res
}

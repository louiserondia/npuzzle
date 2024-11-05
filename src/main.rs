use grid::{solver::Solver, Grid};

mod complex;
mod grid;
fn main() {
    let size = 3;
    let g = Grid::create_random_grid(size, 6000);
    println!("{:?}", g);

    let mut solver = Solver::new(g);
    solver.solve();
    let res = solver.closed_set[&Grid::create_solved_grid(size).v].clone();
    println!("{:?}", res.path.len());
}

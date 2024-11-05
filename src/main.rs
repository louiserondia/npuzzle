use complex::Complex;
use grid::Grid;

mod complex;
mod grid;

fn main() {
    let g2 = Grid::create_random_grid(5, 1000);
    println!("{:?}", g2);
}

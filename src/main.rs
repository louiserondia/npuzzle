mod complex;
mod grid;


fn main() {
    let mut g = grid::Grid::create_solved_grid(15);
    println!("{:?}", g);

    let mut z = complex::Complex::new(1, 0);
    println!("{:?}", z);
}

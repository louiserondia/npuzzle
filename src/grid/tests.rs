use crate::complex::Complex;

use super::Grid;

#[test]
fn create_solved_grid() {
    let g = Grid::create_solved_grid(3);
    assert!(g.size == 3);
    assert!(g.v == vec![1, 2, 3, 8, 0, 4, 7, 6, 5]);
    assert!(g.zero == Complex::new(1, 1));
}

#[test]
fn get_cell_ref() {
    let g = Grid::create_solved_grid(3);
    assert!(*g.get_cell_ref(Complex::new(2, 0)) == 3);
    assert!(*g.get_cell_ref(Complex::new(0, 2)) == 7);
}

#[test]
fn get_cell_mut() {
    let mut g = Grid::create_solved_grid(3);
    assert!(*g.get_cell_mut(Complex::new(2, 0)) == 3);
    assert!(*g.get_cell_mut(Complex::new(0, 2)) == 7);
}

#[test]
fn op() {
    let mut g = Grid::create_solved_grid(3);
    g.op(Complex::new(0, -1));
    assert!(g.v == vec![1, 0, 3, 8, 2, 4, 7, 6, 5]);
    g.op(Complex::new(1, 0));
    assert!(g.v == vec![1, 3, 0, 8, 2, 4, 7, 6, 5]);
}

#[test]
fn is_op_legal() {
    let mut g = Grid::create_solved_grid(3);
    assert!(Grid::dirs().iter().all(|d| g.is_op_legal(*d)));
    g.op(Complex::new(0, 1));
    assert!(!g.is_op_legal(Complex::new(0, 1)));
    g.op(Complex::new(1, 0));
    assert!(!g.is_op_legal(Complex::new(1, 0)));
    assert!(g.is_op_legal(Complex::new(-1, 0)));
    assert!(g.is_op_legal(Complex::new(0, -1)));
}

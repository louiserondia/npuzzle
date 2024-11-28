use crate::{
    complex::Complex,
    grid::{
        solver::{is_solvable, solve, Algo, Heuristic},
        Grid,
    },
};

#[test]
fn solvable_3() {
    let g = Grid {
        size: 3,
        v: vec![8, 4, 2, 3, 0, 5, 6, 7, 1],
        zero: Complex::new(1, 1),
    };
    assert!(is_solvable(&g));
}

#[test]
fn solvable_4() {
    let g = Grid {
        size: 4,
        v: vec![3, 11, 13, 6, 14, 4, 0, 15, 7, 12, 1, 9, 8, 10, 2, 5],
        zero: Complex::new(2, 1),
    };
    assert!(is_solvable(&g));
}

#[test]
fn unsolvable_3() {
    let g = Grid {
        size: 3,
        v: vec![6, 4, 0, 2, 7, 3, 5, 1, 8],
        zero: Complex::new(1, 1),
    };
    assert!(!is_solvable(&g));
}

#[test]
fn unsolvable_4() {
    let g = Grid {
        size: 4,
        v: vec![4, 14, 8, 6, 5, 12, 3, 7, 9, 1, 15, 10, 13, 11, 0, 2],
        zero: Complex::new(2, 3),
    };
    assert!(!is_solvable(&g));
}

fn check_sequence(g: &Grid, sequence: &Vec<Complex<i32>>) {
    let mut g = g.clone();
    for &op in sequence.iter() {
        g.op(op);
    }
    assert!(g.v == Grid::create_solved_grid(g.size).v);
}

fn test_solve(g: &Grid, target: usize) {
    for &h in &[
        Heuristic::Manhattan,
        Heuristic::Euclidian,
        Heuristic::Misplaced,
    ] {
        for &alg in &[Algo::Astar, Algo::IDAstar] {
            let res = solve(&g, h, alg).unwrap();
            assert!(res.sequence.len() == target);
            check_sequence(g, &res.sequence);
        }
    }
}

#[test]
fn solve_3() {
    let g = Grid {
        size: 3,
        v: vec![3, 6, 1, 2, 4, 5, 8, 7, 0],
        zero: Complex::new(2, 2),
    };
    test_solve(&g, 18);
}

#[test]
fn solve_4() {
    let g = Grid {
        size: 4,
        v: vec![12, 1, 2, 4, 11, 13, 6, 5, 10, 9, 3, 0, 8, 15, 7, 14],
        zero: Complex::new(3, 2),
    };
    test_solve(&g, 22);
}

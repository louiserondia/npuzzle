use crate::complex::{self, Complex};
use rand::{seq::IteratorRandom, thread_rng};
use std::{collections::HashSet, fmt, mem::swap};

pub struct Grid {
    grid: Vec<i32>,
    size: i32,
    zero: Complex<i32>,
}

impl Grid {
    fn dirs() -> [Complex<i32>; 4] {
        [
            Complex::new(0, 1),
            Complex::new(1, 0),
            Complex::new(0, -1),
            Complex::new(-1, 0),
        ]
    }

    fn get_cell_ref(&self, p: Complex<i32>) -> &i32 {
        &self.grid[(p.y * self.size + p.x) as usize]
    }

    fn get_cell_mut(&mut self, p: Complex<i32>) -> &mut i32 {
        &mut self.grid[(p.y * self.size + p.x) as usize]
    }

    pub fn op(&mut self, d: Complex<i32>) {
        let v1 = *self.get_cell_ref(self.zero);
        let v2 = *self.get_cell_ref(self.zero + d);
        *self.get_cell_mut(self.zero) = v2;
        *self.get_cell_mut(self.zero + d) = v1;
        self.zero += d;
    }

    pub fn create_solved_grid(size: i32) -> Self {
        let mut grid = Grid {
            grid: vec![0; (size * size) as usize],
            size,
            zero: Complex::new(0, 0),
        };
        let mut hs = HashSet::new();
        let mut p = Complex::new(0, 0);
        let mut d = Complex::new(1, 0);
        for i in 1..size * size {
            *grid.get_cell_mut(p) = i;
            hs.insert(p);
            p += d;
            let np = p + d;
            if !(0..size).contains(&np.x) || !(0..size).contains(&np.y) || hs.contains(&np) {
                d *= Complex::new(0, 1);
            }
        }
        grid.zero = p;
        grid
    }

    pub fn create_random_grid(size: i32, n: i32) -> Self {
        let mut g = Self::create_solved_grid(size);
        if size == 1 {
            return g;
        }
        let mut rng = thread_rng();
        for _ in 0..n {
            let dirs = Self::dirs();
            let op = dirs
                .iter()
                .filter(|d| {
                    let p = g.zero + **d;
                    (0..size).contains(&p.x) && (0..size).contains(&p.y)
                })
                .choose(&mut rng)
                .unwrap();
            g.op(*op);
        }
        g
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "[{}]\t", self.get_cell_ref(Complex::new(x, y)))?;
            }
            if y < self.size - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

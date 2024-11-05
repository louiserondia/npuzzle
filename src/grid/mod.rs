use crate::complex;
use std::{collections::HashSet, fmt};

pub struct Grid {
    grid: Vec<u16>,
    size: usize,
}

impl Grid {
    fn get_cell(&mut self, x: usize, y: usize) -> &mut u16 {
        &mut self.grid[y * self.size + x]
    }

    pub fn create_solved_grid(size: usize) -> Self {
        let mut grid = Grid {
            grid: vec![0; size * size],
            size,
        };
        let mut hs = HashSet::new();
        let mut p: complex::Complex<i32> = complex::Complex::new(0, 0);
        let mut d = complex::Complex::new(1, 0);
        for i in 1..size * size {
            *grid.get_cell(p.x as usize, p.y as usize) = i as u16;
            hs.insert(p);
            p += d;
            let np = p + d;
            if !(0..size).contains(&(np.x as usize))
                || !(0..size).contains(&(np.y as usize))
                || hs.contains(&np)
            {
                d *= complex::Complex::new(0, 1);
            }
        }
        grid
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "[{}]\t", self.grid[y * self.size + x])?;
            }
            if y < self.size - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
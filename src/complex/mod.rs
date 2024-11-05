use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Complex<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        let new_x = self.x * rhs.x - self.y * rhs.y;
        let new_y = self.x * rhs.y + self.y * rhs.x;
        self.x = new_x;
        self.y = new_y;
    }
}

impl<T> Complex<T>
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

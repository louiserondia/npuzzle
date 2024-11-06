use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Complex<T> {
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
        (self.x, self.y) = (
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        );
    }
}

impl<T> Complex<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    pub fn manhattan_dist(z1: Self, z2: Self) -> T {
        let f = |a, b| if a > b { a - b } else { b - a };
        f(z1.x, z2.x) + f(z1.y, z2.y)
    }
}

use std::ops::{Add, Div, Neg, Sub};

#[derive(Hash, Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl <T: Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y }
    }
}

impl <T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl <T: Copy + Div<Output = T>> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}

impl <T: Eq> Eq for Point<T> { }

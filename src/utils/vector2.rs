use std::{
    cmp::{max, min, Eq, Ord},
    ops::*,
};

pub type Vec2F = Vec2Base<f32>;
pub type Vec2U = Vec2Base<usize>;
pub type Vec2I = Vec2Base<isize>;

#[derive(PartialEq, Default, Copy, Clone, Debug)]
pub struct Vec2Base<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2Base<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Eq for Vec2Base<T> where T: Eq {}

impl<T> Vec2Base<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn area(&self) -> T {
        self.x * self.y
    }

    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Vec2Base<T>
where
    T: Ord,
{
    pub fn min(lhs: Self, rhs: Self) -> Self {
        Self {
            x: min(lhs.x, rhs.x),
            y: min(lhs.y, rhs.y),
        }
    }

    pub fn max(lhs: Self, rhs: Self) -> Self {
        Self {
            x: max(lhs.x, rhs.x),
            y: max(lhs.y, rhs.y),
        }
    }
}

impl<T> Add<Self> for Vec2Base<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Self> for Vec2Base<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<Self> for Vec2Base<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div<Self> for Vec2Base<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> Neg for Vec2Base<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Vec2F {
    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }
}

impl From<Vec2I> for Vec2U {
    fn from(src: Vec2I) -> Self {
        Self {
            x: src.x as usize,
            y: src.y as usize,
        }
    }
}

impl From<Vec2U> for Vec2I {
    fn from(src: Vec2U) -> Self {
        Self {
            x: src.x as isize,
            y: src.y as isize,
        }
    }
}

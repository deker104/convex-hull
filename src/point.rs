use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Point<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    T: Sub<Output = T>
{
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl Point<i64> {
    pub fn orient(self, b: Self, c: Self) -> i8 {
        let res = (b - self).cross(c - self);

        use std::cmp::Ordering::*;
        match res.cmp(&0) {
            Less => -1,
            Equal => 0,
            Greater => 1,
        }
    }
}

impl Point<f64> {
    const EPSILON: f64 = 1e-9;

    pub fn orient(self, b: Self, c: Self) -> i8 {
        let res = (b - self).cross(c - self);
        if res > Self::EPSILON {
            1
        } else if res < -Self::EPSILON {
            -1
        } else {
            0
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    T: Sub<Output = T>,
{
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

pub enum Orientation {
    CounterClockwise,
    Clockwise,
    None,
}

pub trait Orient {
    fn orient(self, b: Self, c: Self) -> Orientation;
}

impl Orient for Point<i64> {
    fn orient(self, b: Self, c: Self) -> Orientation {
        let res = (b - self).cross(c - self);

        use Orientation::*;
        use std::cmp::Ordering::*;
        match res.cmp(&0) {
            Less => Clockwise,
            Equal => None,
            Greater => CounterClockwise,
        }
    }
}

impl Orient for Point<f64> {
    fn orient(self, b: Self, c: Self) -> Orientation {
        const EPSILON: f64 = 1e-9;

        let res = (b - self).cross(c - self);
        use Orientation::*;
        if res > EPSILON {
            CounterClockwise
        } else if res < -EPSILON {
            Clockwise
        } else {
            None
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

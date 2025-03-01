use crate::*;
use std::ops::{Add, Sub, Mul};

#[derive(Default)]
pub struct Jarvis;

impl Jarvis {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> ConvexHullSolver<T> for Jarvis
where
    T: Copy + PartialOrd + Sub<Output = T> + Add<Output = T> + Mul<Output = T>,
    Point<T>: Orient,
{
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        let points: Vec<Point<T>> = points.into_iter().collect();
        let n = points.len();

        if n < 3 {
            return vec![];
        }

        let mut hull = vec![];
        let mut left_most = 0;
        for i in 1..n {
            if points[i].x < points[left_most].x || (points[i].x == points[left_most].x && points[i].y < points[left_most].y) {
                left_most = i;
            }
        }

        let mut p = left_most;

        loop {
            hull.push(points[p]);

            let mut q = (p + 1) % n;
            for i in 0..n {
                if Point::orient(points[p], points[i], points[q]) == Orientation::CounterClockwise {
                    q = i;
                }
            }

            p = q;

            if p == left_most {
                break;
            }
        }

        hull
    }
}

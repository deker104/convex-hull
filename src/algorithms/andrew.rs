// src/algorithms/andrew.rs

use crate::*;

#[derive(Default)]
pub struct Andrew {}

impl Andrew {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> ConvexHullSolver<T> for Andrew
where
    Point<T>: PartialEq + PartialOrd + Copy + Orient,
{
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        let mut points: Vec<Point<T>> = points.into_iter().collect();
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut lower: Vec<Point<T>> = Vec::new();
        for point in points.iter() {
            while lower.len() >= 2
                && lower[lower.len() - 2].orient(lower[lower.len() - 1], *point) != Orientation::CounterClockwise
            {
                lower.pop();
            }
            lower.push(*point);
        }

        let mut upper: Vec<Point<T>> = Vec::new();
        for point in points.iter().rev() {
            while upper.len() >= 2
                && upper[upper.len() - 2].orient(upper[upper.len() - 1], *point) != Orientation::CounterClockwise
            {
                upper.pop();
            }
            upper.push(*point);
        }

        // Убираем последнюю точку с обоих массивов, так как она повторяется
        lower.pop();
        upper.pop();

        // Объединяем верхнюю и нижнюю оболочку
        lower.extend(upper);
        lower
    }
}

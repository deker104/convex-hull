use crate::*;

#[derive(Default)]
pub struct Example{}

impl Example {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> ConvexHullSolver<T> for Example {
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        points.into_iter().collect()
    }
}
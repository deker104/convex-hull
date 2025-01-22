use crate::Point;

pub type ConvexHull<T> = Vec<Point<T>>;

pub trait ConvexHullSolver<T> {
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T>;
}
use crate::*;
use std::cmp::Ordering;
use std::ops::{Sub, Add, Mul};
use num_traits::Zero; 

#[derive(Default)]
pub struct QuickHull;

impl QuickHull {
    pub fn new() -> Self {
        Self {}
    }

    fn find_hull<T>(&self, points: &mut Vec<Point<T>>, p1: Point<T>, p2: Point<T>, hull: &mut Vec<Point<T>>)
    where
        T: Copy + PartialOrd + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Zero,
    {
        if points.is_empty() {
            return;
        }

        let mut farthest_point = points[0];
        let mut max_distance = (p2 - p1).cross(farthest_point - p1);

        for &point in points.iter() {
            let distance = (p2 - p1).cross(point - p1);
            if distance > max_distance {
                max_distance = distance;
                farthest_point = point;
            }
        }

        let mut left_set = Vec::new();
        let mut right_set = Vec::new();

        for &point in points.iter() {
            if (p2 - p1).cross(point - p1) > T::zero() {
                left_set.push(point);
            } else if (farthest_point - p1).cross(point - p1) > T::zero() {
                right_set.push(point);
            }
        }

        self.find_hull(&mut left_set, p1, farthest_point, hull);
        hull.push(farthest_point);
        self.find_hull(&mut right_set, farthest_point, p2, hull);
    }
}

impl<T> ConvexHullSolver<T> for QuickHull
where
    T: Copy + PartialOrd + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Zero,
{
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        let mut points: Vec<Point<T>> = points.into_iter().collect();
        if points.len() <= 3 {
            return ConvexHull::from_iter(points);
        }

        let mut hull = Vec::new();

        let min_x = points.iter().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal)).unwrap().clone();
        let max_x = points.iter().max_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal)).unwrap().clone();

        let mut left_set = Vec::new();
        let mut right_set = Vec::new();

        for &point in points.iter() {
            if (max_x - min_x).cross(point - min_x) > T::zero() {
                left_set.push(point);
            } else if (max_x - min_x).cross(point - min_x) < T::zero() {
                right_set.push(point);
            }
        }

        self.find_hull(&mut left_set, min_x, max_x, &mut hull);
        hull.push(max_x);
        self.find_hull(&mut right_set, max_x, min_x, &mut hull);
        hull.push(min_x);

        ConvexHull::from_iter(hull)
    }
}
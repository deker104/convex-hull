use crate::*;

#[derive(Default)]
pub struct BruteForce {}

impl BruteForce {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> ConvexHullSolver<T> for BruteForce
where
    Point<T>: PartialEq + PartialOrd + Copy + Orient,
{
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        let points = {
            let mut points: Vec<Point<T>> = points.into_iter().collect();
            points.sort_by(|a, b| a.partial_cmp(b).unwrap());
            points.dedup();
            points
        };

        if points.len() <= 2 {
            return points;
        }

        let mut result = vec![points[0]];
        let mut current_index = 0;
        let mut current = points[0];
        loop {
            let next_index = (0..points.len())
                .find(|&next_index| {
                    if next_index == current_index {
                        return false;
                    }
                    let next = points[next_index];
                    (0..points.len()).all(|other| {
                        if other == current_index || other == next_index {
                            return true;
                        }
                        let other = points[other];
                        if let Orientation::CounterClockwise = current.orient(next, other) {
                            return true;
                        }
                        false
                    })
                })
                .unwrap();
            result.push(points[next_index]);
            current_index = next_index;
            current = points[next_index];
            if current_index == 0 {
                break;
            }
        }
        result
    }
}

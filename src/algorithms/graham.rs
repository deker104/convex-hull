use crate::*;

#[derive(Default)]
pub struct Graham {}

impl Graham {
    pub fn new() -> Self {
        Self {}
    }

    fn squared_distance(p1: &Point<f64>, p2: &Point<f64>) -> f64 {
        (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
    }

    fn orientation(p1: &Point<f64>, p2: &Point<f64>, p3: &Point<f64>) -> Orientation {
        let val = (p2.y - p1.y) * (p3.x - p2.x) - (p2.x - p1.x) * (p3.y - p2.y);
        if val == 0.0 {
            Orientation::None
        } else if val > 0.0 {
            Orientation::Clockwise
        } else {
            Orientation::CounterClockwise
        }
    }
}

impl ConvexHullSolver<f64> for Graham {
    fn solve(&self, points: impl IntoIterator<Item = Point<f64>>) -> ConvexHull<f64> {
        let mut points: Vec<Point<f64>> = points.into_iter().collect();
        let min_idx = points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.y.partial_cmp(&b.y)
                    .unwrap()
                    .then(a.x.partial_cmp(&b.x).unwrap())
            })
            .map(|(idx, _)| idx)
            .unwrap();
        points.swap(0, min_idx);
        let p0 = points[0];
        points[1..].sort_by(|a, b| {
            let orientation = Self::orientation(&p0, a, b);
            match orientation {
                Orientation::None => {
                    if Self::squared_distance(&p0, a) < Self::squared_distance(&p0, b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                Orientation::CounterClockwise => std::cmp::Ordering::Less,
                Orientation::Clockwise => std::cmp::Ordering::Greater,
            }
        });
        let mut stack: Vec<Point<f64>> = Vec::new();
        stack.push(points[0]);
        stack.push(points[1]);
        stack.push(points[2]);
        for point in points.iter().skip(3) {
            while stack.len() >= 2
                && Self::orientation(&stack[stack.len() - 2], &stack[stack.len() - 1], point)
                    != Orientation::CounterClockwise
            {
                stack.pop();
            }
            stack.push(*point);
        }

        stack
    }
}
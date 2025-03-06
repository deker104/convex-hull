# Convex Hull algorithms

![Build and test workflow status](https://github.com/deker104/convex-hull/actions/workflows/ci.yml/badge.svg?branch=master)

This library provides multiple algorithms to build a convex hull of a set of points on 2D plane and the tools to test and benchmark them.

## Installation

To add this library to your project just add it as a dependency to your `Cargo.toml` file.

```toml
[dependencies]
convex-hull = "0.1"
```

## Documentation

Project documantation can be accessed by running `cargo doc --open` in the cloned repository. There are several documents in the `docs` directory that contains technical documentation of the project in Russian.


## Contributing

If you want to contribute to this project, follow these steps:

1. **Fork the repository** on GitHub.
2. **Create a new branch** for your feature or bug fix.
3. **Make your changes** and commit them with a descriptive message.
4. **Push your changes** to your forked repository.
5. **Submit a pull request** to the `master` branch of the original repository.

### Adding a New Algorithm

If you'd like to add a new algorithm to the library, follow these steps:

#### 1. Create a new file for your algorithm

Navigate to the `convex-hull/src/algorithms/` directory and create a new file for your algorithm. For example, if your algorithm is called `new_algorithm`, create a file named `new_algorithm.rs` and implement your algorithm as a new module.

Example content for `new_algorithm.rs`:

```rust
use crate::*;

#[derive(Default)]
pub struct NewAlgorithm;

impl NewAlgorithm {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> ConvexHullSolver<T> for NewAlgorithm {
    fn solve(&self, points: impl IntoIterator<Item = Point<T>>) -> ConvexHull<T> {
        points.into_iter().collect() // Implement the algorithm here
    }
}
```

In this code:
- `NewAlgorithm` is a struct representing your algorithm.
- The `solve` method implements the logic for solving the convex hull problem for the points.

#### 2. Add your algorithm module to `mod.rs`

In the `convex-hull/src/algorithms/mod.rs` file, add your new module by including the following line:

```rust
pub mod new_algorithm;
```

This will make your new algorithm available for use in the library.

#### 3. Update the benchmarks

To include your algorithm in the benchmarking tests, update the `benchmarks.rs` file located in `convex-hull/benches/` (or wherever your benchmark tests are located).

In the `benchmarks.rs` file, import your new algorithm and add a benchmarking function. 

Here’s an example of how to include the new algorithm in the benchmarks:

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use convex_hull::{ConvexHullSolver, Point};
use convex_hull::algorithms::{bruteforce::BruteForce, andrew::Andrew, new_algorithm::NewAlgorithm}; // Import your new algorithm

// Generate random points
fn generate_random_points(n: usize) -> Vec<Point<f64>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Point {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
        })
        .collect()
}

// Universal benchmarking function
fn benchmark_algorithm<S: ConvexHullSolver<f64>>(c: &mut Criterion, name: &str, solver: S) {
    let input = generate_random_points(100); // 100 random points

    c.bench_function(name, |b| {
        b.iter(|| {
            solver.solve(input.clone());
        });
    });
}

// Benchmark for each algorithm
fn benchmark_bruteforce(c: &mut Criterion) {
    benchmark_algorithm(c, "bruteforce", BruteForce::new());
}

fn benchmark_andrew(c: &mut Criterion) {
    benchmark_algorithm(c, "andrew", Andrew::new());
}

fn benchmark_new_algorithm(c: &mut Criterion) {
    benchmark_algorithm(c, "new_algorithm", NewAlgorithm::new()); // Benchmark for new algorithm
}

// Benchmark group
criterion_group!(
    benches,
    benchmark_bruteforce,
    benchmark_andrew,
    benchmark_new_algorithm // Add new benchmark
);

// Main function for running benchmarks
criterion_main!(benches);
```

In this code:
- The `benchmark_new_algorithm` function benchmarks your new algorithm.
- The `criterion_group!` macro includes your new algorithm in the benchmark suite.

#### 4. Update the documentation

To include your new algorithm in the project documentation, navigate to the `convex-hull/docs/algorithms.md` file and add a section describing your new algorithm, its usage, and performance characteristics. This is important for users who will be utilizing the library and need to understand the available algorithms.

Example documentation entry:

```markdown
## New Algorithm

The `NewAlgorithm` is a new method for solving the convex hull problem. It is based on [describe the core idea or approach].

### Usage

```rust
use convex_hull::{ConvexHullSolver, Point};
use convex_hull::algorithms::new_algorithm::NewAlgorithm;

let solver = NewAlgorithm::new();
let points = vec![
    Point { x: 1.0, y: 2.0 },
    Point { x: 3.0, y: 4.0 },
    // more points
];
let hull = solver.solve(points);
```

### Performance

The `NewAlgorithm` performs well in [describe performance, e.g., time complexity or benchmarks].
```

This ensures that your new algorithm is properly documented and easy for users to understand and use.

By following these steps, you can easily add new algorithms to the library and ensure they are well-documented and benchmarked for performance.



## License

This project is licensed under the MIT license. See the LICENSE file for more details.

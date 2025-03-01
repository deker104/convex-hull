use criterion::{criterion_group, criterion_main, Criterion};
use convex_hull::{ConvexHullSolver, Point};
use convex_hull::algorithms::{bruteforce::BruteForce, andrew::Andrew, graham::Graham, quickhull::QuickHull}; // Алгоритмы

// Генерация случайных точек
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

// Универсальная функция для бенчмаркинга
fn benchmark_algorithm<S: ConvexHullSolver<f64>>(c: &mut Criterion, name: &str, solver: S) {
    let input = generate_random_points(100); // 100 случайных точек

    c.bench_function(name, |b| {
        b.iter(|| {
            solver.solve(input.clone());
        });
    });
}

// Бенчмарки для каждого алгоритма
fn benchmark_bruteforce(c: &mut Criterion) {
    benchmark_algorithm(c, "bruteforce", BruteForce::new());
}

fn benchmark_andrew(c: &mut Criterion) {
    benchmark_algorithm(c, "andrew", Andrew::new());
}

fn benchmark_graham(c: &mut Criterion) {
    benchmark_algorithm(c, "graham", Graham::new());
}

fn benchmark_quickhull(c: &mut Criterion) {
    benchmark_algorithm(c, "quickhull", QuickHull::new());
}

// Группа бенчмарков
criterion_group!(
    benches,
    benchmark_bruteforce,
    benchmark_andrew,
    benchmark_graham, // Добавляем новый бенчмарк в группу
    benchmark_quickhull
);

// Главная функция для запуска бенчмарков
criterion_main!(benches);

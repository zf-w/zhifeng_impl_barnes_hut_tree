use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

mod utils;
use utils::{
    check_tree_force_simulation_on_random_values, check_vanillia_force_simulation_on_random_values,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut len = 1000;
    let mut g = c.benchmark_group("Compare Unchecked One-Calc-Update-Round 1000-Body Simulation");
    g.measurement_time(Duration::from_secs(20));
    g.bench_function("Barnes-Hut-Tree", |b| {
        b.iter(|| check_tree_force_simulation_on_random_values(len))
    });
    g.bench_function("Vanillia", |b| {
        b.iter(|| check_vanillia_force_simulation_on_random_values(len))
    });
    drop(g);
    let mut g = c.benchmark_group("Compare Unchecked One-Calc-Update-Round 10000-Body Simulation");
    g.measurement_time(Duration::from_secs(20));
    len = 10000;
    g.bench_function("Barnes-Hut-Tree", |b| {
        b.iter(|| check_tree_force_simulation_on_random_values(len))
    });
    g.bench_function("Vanillia", |b| {
        b.iter(|| check_vanillia_force_simulation_on_random_values(len))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

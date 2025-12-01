
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day3;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 3 Solutions");

    group.bench_function("day3 solution1", |b| b.iter(|| day3::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

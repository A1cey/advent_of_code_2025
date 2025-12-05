
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day1;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 1 Solutions");

    group.bench_function("day1 solution1", |b| b.iter(|| day1::solution1::solve()));
    group.bench_function("day1 inclusive_range", |b| b.iter(|| day1::inclusive_range::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

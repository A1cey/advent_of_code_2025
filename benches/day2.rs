
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day2;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 2 Solutions");

    group.bench_function("day2 solution1", |b| b.iter(|| day2::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

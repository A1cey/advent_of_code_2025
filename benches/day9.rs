
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day9;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 9 Solutions");

    group.bench_function("day9 solution1", |b| b.iter(|| day9::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

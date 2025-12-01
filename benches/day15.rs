
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day15;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 15 Solutions");

    group.bench_function("day15 solution1", |b| b.iter(|| day15::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

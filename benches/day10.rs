
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day10;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 10 Solutions");

    group.bench_function("day10 solution1", |b| b.iter(|| day10::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

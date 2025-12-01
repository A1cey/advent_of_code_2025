
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day23;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 23 Solutions");

    group.bench_function("day23 solution1", |b| b.iter(|| day23::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

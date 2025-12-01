use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day16;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 16 Solutions");

    group.bench_function("day16 solution1", |b| b.iter(|| day16::solution1::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

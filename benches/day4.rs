use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day4;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 4 Solutions Part One");

    group.bench_function("day4 part_one", |b| b.iter(|| day4::part_one::solve()));

    group.finish();

    let mut group = c.benchmark_group("Day 4 Solutions Part Two");

    group.bench_function("day4 part_two", |b| b.iter(|| day4::part_two::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

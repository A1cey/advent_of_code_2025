
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day6;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6 Solutions Part One");

    group.bench_function("day6 part_one_iterator", |b| b.iter(|| day6::part_one_iterator::solve()));

    group.finish();
    
    let mut group = c.benchmark_group("Day 6 Solutions Part Two");

    group.bench_function("day6 part_two", |b| b.iter(|| day6::part_two::solve()));

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

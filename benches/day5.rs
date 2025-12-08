use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day5;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 5 Solutions Part One");

    group.bench_function("day5 part_one", |b| b.iter(|| day5::part_one::solve()));

    group.finish();

    let mut group = c.benchmark_group("Day 5 Solutions Part Two");

    group.bench_function("day5 part_two", |b| {
        b.iter(|| day5::part_two_very_slow::solve())
    });
    group.bench_function("day5 part_two_sorted_ranges_comparison", |b| {
        b.iter(|| day5::part_two_sorted_ranges_comparison::solve())
    });

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

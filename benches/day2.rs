use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day2;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 2 Solutions");

    group.bench_function("day2 part_one_conversion_to_string", |b| {
        b.iter(|| day2::part_one_conversion_to_string::solve())
    });
    group.bench_function("day2 part_one_all_numbers_with_log", |b| {
        b.iter(|| day2::part_one_all_numbers_with_log::solve())
    });
    group.bench_function("day2 part_one_all_numbers_with_loop", |b| {
        b.iter(|| day2::part_one_all_numbers_with_loop::solve())
    });
    group.bench_function("day2 part_one_iterator", |b| {
        b.iter(|| day2::part_one_iterator::solve())
    });
    group.bench_function("day2 part_two_division", |b| {
        b.iter(|| day2::part_two_division::solve())
    });
    group.bench_function("day2 part_two_compare_remainder", |b| {
        b.iter(|| day2::part_two_compare_remainder::solve())
    });

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

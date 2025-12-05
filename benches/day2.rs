use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day2;

fn solutions(c: &mut Criterion) {
    let mut group_1 = c.benchmark_group("Day 2 Solutions Part One");

    group_1.bench_function("day2 part_one_conversion_to_string", |b| {
        b.iter(|| day2::part_one_conversion_to_string::solve())
    });
    group_1.bench_function("day2 part_one_all_numbers_with_log", |b| {
        b.iter(|| day2::part_one_all_numbers_with_log::solve())
    });
    group_1.bench_function("day2 part_one_all_numbers_with_loop", |b| {
        b.iter(|| day2::part_one_all_numbers_with_loop::solve())
    });
    group_1.bench_function("day2 part_one_iterator", |b| {
        b.iter(|| day2::part_one_iterator::solve())
    });

    group_1.finish();

    let mut group_2 = c.benchmark_group("Day 2 Solutions Part One");

    group_2.bench_function("day2 part_two_division", |b| {
        b.iter(|| day2::part_two_division::solve())
    });
    group_2.bench_function("day2 part_two_compare_remainder", |b| {
        b.iter(|| day2::part_two_compare_remainder::solve())
    });

    group_2.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

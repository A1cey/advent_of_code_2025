
use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2025::day3;

fn solutions(c: &mut Criterion) {
    let mut group_1 = c.benchmark_group("Day 3 Solutions Part 1");

    group_1.bench_function("day3 part_one", |b| b.iter(|| day3::part_one::solve()));

    group_1.finish();
    
    let mut group_2 =c.benchmark_group("Day 3 Solutions Part 2");
    
    group_2.bench_function("day3 part_two", |b| b.iter(|| day3::part_two::solve()));
    group_2.bench_function("day3 part_two_using_bytes_not_chars", |b| b.iter(|| day3::part_two_using_bytes_not_chars::solve()));
    group_2.bench_function("day3 part_two_position_iterator", |b| b.iter(|| day3::part_two_position_iterator::solve()));

    group_2.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);

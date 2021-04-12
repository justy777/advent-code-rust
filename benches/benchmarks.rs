use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_code_rust::year_2015::day_01::{calculate_final_count, calculate_operations_to_value};

pub fn criterion_benchmark(c: &mut Criterion) {
    let contents =
        std::fs::read("input/2015/day-1.txt").expect("Failed to read file to string.");

    c.bench_function("Year 2015 Day 1 - calculate final count", |b| b.iter(|| calculate_final_count(black_box(&contents))));
    c.bench_function("Year 2015 Day 1 - calculate operation to value", |b| b.iter(|| calculate_operations_to_value(black_box(&contents), -1)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
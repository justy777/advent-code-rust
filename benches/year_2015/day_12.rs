use std::fs;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_12::{sum_numbers_in_str, sum_value};

fn sum_numbers_in_str_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_12 - sum_numbers_in_str file", |b| {
        b.iter(|| sum_numbers_in_str(black_box(&contents)));
    });
}

fn sum_value_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_12 - sum_value file", |b| {
        b.iter(|| {
            let value = serde_json::from_str(&contents).unwrap();
            sum_value(black_box(&value));
        });
    });
}

criterion_group!(benches, sum_numbers_in_str_benchmark, sum_value_benchmark);

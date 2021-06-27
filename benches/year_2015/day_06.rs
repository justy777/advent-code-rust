use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_06::{AdjustableBulb, LightGrid, LightInstruction, SimpleBulb};

fn apply_operation_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-06.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_06 - apply_operation SimpleBulb", |b| {
        b.iter(|| {
            let mut grid = LightGrid::<SimpleBulb>::new();
            contents
                .lines()
                .map(|s| LightInstruction::from_str(black_box(s)))
                .filter_map(Result::ok)
                .for_each(|instruction| grid.apply_operation(black_box(&instruction)));
            let _ = grid.total_brightness();
        });
    });

    c.bench_function("year_2015::day_06 - apply_operation AdjustableBulb", |b| {
        b.iter(|| {
            let mut grid = LightGrid::<AdjustableBulb>::new();
            contents
                .lines()
                .map(|s| LightInstruction::from_str(black_box(s)))
                .filter_map(Result::ok)
                .for_each(|instruction| grid.apply_operation(black_box(&instruction)));
            let _ = grid.total_brightness();
        });
    });
}

criterion_group!(benches, apply_operation_benchmark);

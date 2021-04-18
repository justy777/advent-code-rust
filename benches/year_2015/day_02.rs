use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_02::Present;

fn wrapping_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input/2015/day-02.txt").expect("Failed to read file.");

    c.bench_function("year_2015::day_02 - wrapping paper/0x0x0", |b| {
        b.iter(|| {
            if let Ok(present) = Present::from_str(black_box("0x0x0")) {
                present.wrapping_paper_needed();
            }
        });
    });
    c.bench_function("year_2015::day_02 - wrapping paper/file", |b| {
        b.iter(|| {
            let presents: Vec<Present> = contents
                .lines()
                .map(|s| Present::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .collect();

            let _: u32 = presents
                .iter()
                .map(|present| present.wrapping_paper_needed())
                .sum();
        });
    });
}

fn ribbon_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input/2015/day-02.txt").expect("Failed to read file.");

    c.bench_function("year_2015::day_02 - ribbon/0x0x0", |b| {
        b.iter(|| {
            if let Ok(present) = Present::from_str(black_box("0x0x0")) {
                present.ribbon_needed();
            }
        });
    });
    c.bench_function("year_2015::day_02 - ribbon/file", |b| {
        b.iter(|| {
            let presents: Vec<Present> = contents
                .lines()
                .map(|s| Present::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .collect();

            let _: u32 = presents.iter().map(|present| present.ribbon_needed()).sum();
        });
    });
}

criterion_group!(benches, wrapping_benchmark, ribbon_benchmark);

use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code::year_2015::day_01::{floor, position};
use advent_of_code::year_2015::day_02::Present;

pub fn floor_benchmark(c: &mut Criterion) {
    let contents = fs::read("input/2015/day-1.txt").expect("Failed to read file.");

    c.bench_function("Year 2015 Day 1 - floor/empty", |b| {
        b.iter(|| floor(black_box(b"")))
    });
    c.bench_function("Year 2015 Day 1 - floor/file", |b| {
        b.iter(|| floor(black_box(&contents)))
    });
}

pub fn position_benchmark(c: &mut Criterion) {
    let contents = fs::read("input/2015/day-1.txt").expect("Failed to read file.");

    c.bench_function("Year 2015 Day 1 - position/empty/-1", |b| {
        b.iter(|| position(black_box(b""), -1))
    });
    c.bench_function("Year 2015 Day 1 - position/()())/0", |b| {
        b.iter(|| position(black_box(b"()())"), 0))
    });
    c.bench_function("Year 2015 Day 1 - position/file/-1", |b| {
        b.iter(|| position(black_box(&contents), -1))
    });
}

pub fn wrapping_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file.");

    c.bench_function("Year 2015 Day 2 - wrapping paper/0x0x0", |b| {
        b.iter(|| {
            let present = Present::from_str(black_box("0x0x0")).unwrap();
            present.wrapping_paper_needed();
        });
    });
    c.bench_function("Year 2015 Day 2 - wrapping paper/file", |b| {
        b.iter(|| {
            let presents: Vec<Present> = contents
                .lines()
                .map(|line| Present::from_str(line).unwrap())
                .collect();

            let _: u32 = presents
                .iter()
                .map(|present| present.wrapping_paper_needed())
                .sum();
        });
    });
}

pub fn ribbon_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file.");

    c.bench_function("Year 2015 Day 2 - ribbon/0x0x0", |b| {
        b.iter(|| {
            let present = Present::from_str(black_box("0x0x0")).unwrap();
            present.ribbon_needed();
        });
    });
    c.bench_function("Year 2015 Day 2 - ribbon/file", |b| {
        b.iter(|| {
            let presents: Vec<Present> = contents
                .lines()
                .map(|line| Present::from_str(line).unwrap())
                .collect();

            let _: u32 = presents.iter().map(|present| present.ribbon_needed()).sum();
        });
    });
}

criterion_group!(
    benches,
    floor_benchmark,
    position_benchmark,
    wrapping_benchmark,
    ribbon_benchmark
);
criterion_main!(benches);

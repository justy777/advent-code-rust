use std::fs;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_01::{floor, position_to_floor};

fn floor_benchmark(c: &mut Criterion) {
    let contents = fs::read("input/2015/day-01.txt").expect("Failed to read file.");

    c.bench_function("year_2015::day_01 - floor/empty", |b| {
        b.iter(|| floor(black_box(b"")))
    });
    c.bench_function("year_2015::day_01 - floor/file", |b| {
        b.iter(|| floor(black_box(&contents)))
    });
}

fn position_benchmark(c: &mut Criterion) {
    let contents = fs::read("input/2015/day-01.txt").expect("Failed to read file.");

    c.bench_function("year_2015::day_01 - position/empty/-1", |b| {
        b.iter(|| position_to_floor(black_box(b""), black_box(-1)))
    });
    c.bench_function("year_2015::day_01 - position/()())/0", |b| {
        b.iter(|| position_to_floor(black_box(b"()())"), black_box(0)))
    });
    c.bench_function("year_2015::day_01 - position/file/-1", |b| {
        b.iter(|| position_to_floor(black_box(&contents), black_box(-1)))
    });
}

criterion_group!(benches, floor_benchmark, position_benchmark);

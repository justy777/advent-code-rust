use advent_of_code::year_2015::day_01::{floor, position};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let contents = std::fs::read("input/2015/day-1.txt").expect("Failed to read file to string.");

    c.bench_function("Year 2015 Day 1 - floor", |b| {
        b.iter(|| floor(black_box(&contents)))
    });
    c.bench_function("Year 2015 Day 1 - position", |b| {
        b.iter(|| position(black_box(&contents), -1))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

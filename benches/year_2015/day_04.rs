use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_04::find_number;

fn find_number_benchmark(c: &mut Criterion) {
    c.bench_function("year_2015::day_04 - find_number iwrupvqb/5", |b| {
        b.iter(|| find_number(black_box(b"iwrupvqb"), black_box(5)));
    });
    c.bench_function("year_2015::day_04 - find_number iwrupvqb/6", |b| {
        b.iter(|| find_number(black_box(b"iwrupvqb"), black_box(6)));
    });
}

criterion_group!(benches, find_number_benchmark);

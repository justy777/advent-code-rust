use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_11::next_password;

fn next_password_benchmark(c: &mut Criterion) {
    c.bench_function("year_2015::day_11 - next_password abcdefgh", |b| {
        b.iter(|| next_password(black_box("abcdefgh")));
    });
    c.bench_function("year_2015::day_11 - next_password ghijklmn", |b| {
        b.iter(|| next_password(black_box("ghijklmn")));
    });
    c.bench_function("year_2015::day_11 - next_password cqjxjnds", |b| {
        b.iter(|| next_password(black_box("cqjxjnds")));
    });
    c.bench_function("year_2015::day_11 - next_password cqjxxyzz", |b| {
        b.iter(|| next_password(black_box("cqjxxyzz")));
    });
}

criterion_group!(benches, next_password_benchmark);

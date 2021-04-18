use std::fs;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_05::{is_nice_word, is_nice_word2};

fn is_nice_word_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-05.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_05 - is_nice_word file", |b| {
        b.iter(|| {
            contents
                .lines()
                .filter(|s| is_nice_word(black_box(s)))
                .count();
        });
    });
}

fn is_nice_word2_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-05.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_05 - is_nice_word2 file", |b| {
        b.iter(|| {
            contents
                .lines()
                .filter(|s| is_nice_word2(black_box(s)))
                .count();
        });
    });
}

criterion_group!(benches, is_nice_word_benchmark, is_nice_word2_benchmark);

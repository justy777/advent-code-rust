use std::fs;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_08::{escape_string, reformat_string};

fn reformat_string_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-08.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_08 - reformat_string sample", |b| {
        b.iter(|| {
            let list = vec![
                String::from(r#""""#),
                String::from(r#""abc""#),
                String::from(r#""aaa\"aaa""#),
                String::from(r#""\x27""#),
            ];
            let _: usize = list.iter().map(String::len).sum();
            let _: usize = list
                .iter()
                .map(|s| reformat_string(black_box(s)).len())
                .sum();
        });
    });
    c.bench_function("year_2015::day_08 - reformat_string file", |b| {
        b.iter(|| {
            let _: usize = contents.lines().map(str::len).sum();
            let _: usize = contents
                .lines()
                .map(|s| reformat_string(black_box(s)).len())
                .sum();
        });
    });
}

fn escape_string_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-08.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_08 - escape_string sample", |b| {
        b.iter(|| {
            let list = vec![
                String::from(r#""""#),
                String::from(r#""abc""#),
                String::from(r#""aaa\"aaa""#),
                String::from(r#""\x27""#),
            ];
            let _: usize = list.iter().map(String::len).sum();
            let _: usize = list.iter().map(|s| escape_string(black_box(s)).len()).sum();
        });
    });
    c.bench_function("year_2015::day_08 - escape_string file", |b| {
        b.iter(|| {
            let _: usize = contents.lines().map(str::len).sum();
            let _: usize = contents
                .lines()
                .map(|s| escape_string(black_box(s)).len())
                .sum();
        });
    });
}

criterion_group!(benches, reformat_string_benchmark, escape_string_benchmark);

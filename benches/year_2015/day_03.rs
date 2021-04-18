use std::fs;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_03::InfiniteGrid;

fn move_position_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-03.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_03 - move_position 0/file", |b| {
        b.iter(|| {
            let mut grid = InfiniteGrid::new(black_box(0));

            contents
                .chars()
                .for_each(|c| grid.move_position(black_box(c)));

            grid.visited();
        });
    });
    c.bench_function("year_2015::day_03 - move_position 1/file", |b| {
        b.iter(|| {
            let mut grid = InfiniteGrid::new(black_box(1));

            contents
                .chars()
                .for_each(|c| grid.move_position(black_box(c)));

            grid.visited();
        });
    });
    c.bench_function("year_2015::day_03 - move_position 2/file", |b| {
        b.iter(|| {
            let mut grid = InfiniteGrid::new(black_box(2));

            contents
                .chars()
                .for_each(|c| grid.move_position(black_box(c)));

            grid.visited();
        });
    });
}

criterion_group!(benches, move_position_benchmark);

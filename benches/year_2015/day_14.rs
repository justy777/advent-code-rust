use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_14::{
    distance_winning_reindeer_traveled, points_awarded_winning_reindeer, Reindeer,
};

fn distance_winning_reindeer_traveled_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    c.bench_function(
        "year_2015::day_14 - distance_winning_reindeer_traveled sample/1000",
        |b| {
            b.iter(|| {
                let mut reindeer = Vec::new();

                if let Ok(comet) = Reindeer::from_str(black_box(
                    "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                )) {
                    reindeer.push(comet);
                };

                if let Ok(dancer) = Reindeer::from_str(black_box(
                    "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                )) {
                    reindeer.push(dancer);
                };

                distance_winning_reindeer_traveled(black_box(&reindeer), black_box(1000));
            });
        },
    );
    c.bench_function(
        "year_2015::day_14 - distance_winning_reindeer_traveled file/2503",
        |b| {
            b.iter(|| {
                let reindeer: Vec<Reindeer> = contents
                    .lines()
                    .map(|s| Reindeer::from_str(black_box(s)))
                    .filter_map(|result| result.ok())
                    .collect();

                distance_winning_reindeer_traveled(black_box(&reindeer), black_box(2503));
            });
        },
    );
}

fn points_awarded_winning_reindeer_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    c.bench_function(
        "year_2015::day_14 - points_awarded_winning_reindeer sample/1000",
        |b| {
            b.iter(|| {
                let mut reindeer = Vec::new();

                if let Ok(comet) = Reindeer::from_str(black_box(
                    "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                )) {
                    reindeer.push(comet);
                };

                if let Ok(dancer) = Reindeer::from_str(black_box(
                    "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                )) {
                    reindeer.push(dancer);
                };

                points_awarded_winning_reindeer(black_box(&reindeer), black_box(1000));
            });
        },
    );
    c.bench_function(
        "year_2015::day_14 - points_awarded_winning_reindeer file/2503",
        |b| {
            b.iter(|| {
                let reindeer: Vec<Reindeer> = contents
                    .lines()
                    .map(|s| Reindeer::from_str(black_box(s)))
                    .filter_map(|result| result.ok())
                    .collect();

                points_awarded_winning_reindeer(black_box(&reindeer), black_box(2503));
            });
        },
    );
}

criterion_group!(
    benches,
    distance_winning_reindeer_traveled_benchmark,
    points_awarded_winning_reindeer_benchmark
);

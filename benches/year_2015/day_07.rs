use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_07::{Circuit, CircuitInstruction};

fn resolve_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-07-sample.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_07 - resolve sample", |b| {
        b.iter(|| {
            let mut circuit = Circuit::new();
            contents
                .lines()
                .map(|s| CircuitInstruction::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .for_each(|instruction| circuit.add_instruction(black_box(instruction)));
            circuit.resolve();
        });
    });

    let contents =
        fs::read_to_string("input/2015/day-07.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_07 - resolve file", |b| {
        b.iter(|| {
            let mut circuit = Circuit::new();
            contents
                .lines()
                .map(|s| CircuitInstruction::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .for_each(|instruction| circuit.add_instruction(black_box(instruction)));
            circuit.resolve();
        });
    });
}

criterion_group!(benches, resolve_benchmark);

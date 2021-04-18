use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_13::{SeatingPlan, SeatingPreference};

fn happiest_table_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-13-sample.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_13 - happiest_table sample", |b| {
        b.iter(|| {
            let mut plan = SeatingPlan::new();

            contents
                .lines()
                .map(|s| SeatingPreference::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .for_each(|preference| plan.add_preference(black_box(preference)));

            plan.happiest_table();
        });
    });

    let contents =
        fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_13 - happiest_table file", |b| {
        b.iter(|| {
            let mut plan = SeatingPlan::new();

            contents
                .lines()
                .map(|s| SeatingPreference::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .for_each(|preference| plan.add_preference(black_box(preference)));

            plan.happiest_table();
        });
    });

    c.bench_function("year_2015::day_13 - happiest_table file + you", |b| {
        b.iter(|| {
            let mut plan = SeatingPlan::new();
            contents
                .lines()
                .map(|s| SeatingPreference::from_str(black_box(s)))
                .filter_map(|result| result.ok())
                .for_each(|preference| plan.add_preference(black_box(preference)));

            for guest in plan.guests.clone() {
                let s = format!(
                    "You would gain 0 happiness units by sitting next to {}.",
                    guest
                );
                if let Ok(preference) = SeatingPreference::from_str(black_box(&s)) {
                    plan.add_preference(black_box(preference));
                }

                let s = format!(
                    "{} would gain 0 happiness units by sitting next to You.",
                    guest
                );
                if let Ok(preference) = SeatingPreference::from_str(black_box(&s)) {
                    plan.add_preference(black_box(preference));
                }
            }

            plan.happiest_table();
        });
    });
}

criterion_group!(benches, happiest_table_benchmark);

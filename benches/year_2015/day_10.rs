use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_10::look_and_say;

fn look_and_say_benchmark(c: &mut Criterion) {
    c.bench_function("year_2015::day_10 - look_and_say 40", |b| {
        b.iter(|| {
            let mut input = String::from("1113122113");
            for _ in 0..40 {
                input = look_and_say(black_box(&input));
            }
        });
    });

    c.bench_function("year_2015::day_10 - look_and_say 50", |b| {
        b.iter(|| {
            let mut input = String::from("1113122113");
            for _ in 0..50 {
                input = look_and_say(black_box(&input));
            }
        });
    });
}

criterion_group!(benches, look_and_say_benchmark);

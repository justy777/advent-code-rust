use std::fs;
use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};

use advent_of_code::year_2015::day_09::{Edge, Graph};

fn shortest_path_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-09.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_09 - shortest_path sample", |b| {
        b.iter(|| {
            let mut graph = Graph::new();

            if let Ok(edge) = Edge::from_str(black_box("London to Dublin = 464")) {
                graph.add_edge(black_box(edge));
            };
            if let Ok(edge) = Edge::from_str(black_box("London to Belfast = 518")) {
                graph.add_edge(black_box(edge));
            };
            if let Ok(edge) = Edge::from_str(black_box("Dublin to Belfast = 141")) {
                graph.add_edge(black_box(edge));
            }

            let _ = graph.shortest_path();
        });
    });
    c.bench_function("year_2015::day_09 - shortest_path file", |b| {
        b.iter(|| {
            let mut graph = Graph::new();
            contents
                .lines()
                .map(|s| Edge::from_str(black_box(s)))
                .filter_map(Result::ok)
                .for_each(|edge| graph.add_edge(black_box(edge)));

            let _ = graph.shortest_path();
        });
    });
}

fn longest_path_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string("input/2015/day-09.txt").expect("Failed to read file to string.");

    c.bench_function("year_2015::day_09 - longest_path sample", |b| {
        b.iter(|| {
            let mut graph = Graph::new();

            if let Ok(edge) = Edge::from_str(black_box("London to Dublin = 464")) {
                graph.add_edge(black_box(edge));
            };
            if let Ok(edge) = Edge::from_str(black_box("London to Belfast = 518")) {
                graph.add_edge(black_box(edge));
            };
            if let Ok(edge) = Edge::from_str(black_box("Dublin to Belfast = 141")) {
                graph.add_edge(black_box(edge));
            }

            let _ = graph.longest_path();
        });
    });
    c.bench_function("year_2015::day_09 - longest_path file", |b| {
        b.iter(|| {
            let mut graph = Graph::new();
            contents
                .lines()
                .map(|s| Edge::from_str(black_box(s)))
                .filter_map(Result::ok)
                .for_each(|edge| graph.add_edge(black_box(edge)));

            let _ = graph.longest_path();
        });
    });
}

criterion_group!(benches, shortest_path_benchmark, longest_path_benchmark);

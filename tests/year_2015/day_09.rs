use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_09::{Edge, Graph};

#[test]
fn test_shortest_path() {
    let mut graph = Graph::new();

    if let Ok(edge) = Edge::from_str("London to Dublin = 464") {
        graph.add_edge(edge);
    };
    if let Ok(edge) = Edge::from_str("London to Belfast = 518") {
        graph.add_edge(edge);
    };
    if let Ok(edge) = Edge::from_str("Dublin to Belfast = 141") {
        graph.add_edge(edge);
    }

    let min = graph.shortest_path();
    assert_eq!(min, 605);
}

#[test]
#[should_panic]
fn test_empty_shortest_path() {
    let graph = Graph::new();
    let _ = graph.shortest_path();
}

#[test]
fn test_longest_path() {
    let mut graph = Graph::new();

    if let Ok(edge) = Edge::from_str("London to Dublin = 464") {
        graph.add_edge(edge);
    };
    if let Ok(edge) = Edge::from_str("London to Belfast = 518") {
        graph.add_edge(edge);
    };
    if let Ok(edge) = Edge::from_str("Dublin to Belfast = 141") {
        graph.add_edge(edge);
    }

    let min = graph.longest_path();
    assert_eq!(min, 982);
}

#[test]
#[should_panic]
fn test_empty_longest_path() {
    let graph = Graph::new();
    let _ = graph.longest_path();
}

#[test]
fn test_shortest_path_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-09.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    contents
        .lines()
        .map(|s| Edge::from_str(s))
        .filter_map(Result::ok)
        .for_each(|edge| graph.add_edge(edge));

    let min = graph.shortest_path();
    assert_eq!(min, 207);
}

#[test]
fn test_longest_path_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-09.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    contents
        .lines()
        .map(|s| Edge::from_str(s))
        .filter_map(Result::ok)
        .for_each(|edge| graph.add_edge(edge));

    let max = graph.longest_path();
    assert_eq!(max, 804);
}

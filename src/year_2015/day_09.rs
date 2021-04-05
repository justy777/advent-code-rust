use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

struct Graph {
    vertices: HashSet<String>,
    edges: HashMap<[String; 2], u32>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, source: &str, destination: &str, weight: u32) {
        self.vertices.insert(String::from(source));
        self.vertices.insert(String::from(destination));
        let mut edge_key = [String::from(source), String::from(destination)];
        edge_key.sort();
        self.edges.insert(edge_key, weight);
    }

    fn cost(&self, source: &str, destination: &str) -> &u32 {
        let mut edge_key = [String::from(source), String::from(destination)];
        edge_key.sort();
        self.edges.get(&edge_key).unwrap()
    }

    fn shortest_path(&self) -> u32 {
        let permutations: Vec<Vec<&String>> = self
            .vertices
            .iter()
            .permutations(self.vertices.len())
            .collect();

        let mut scores = Vec::new();
        for permutation in permutations {
            let mut cost = 0;
            for i in 0..permutation.len() - 1 {
                cost += self.cost(permutation[i], permutation[i + 1]);
            }
            scores.push(cost);
        }
        scores.iter().min().unwrap().to_owned()
    }

    fn longest_path(&self) -> u32 {
        let permutations: Vec<Vec<&String>> = self
            .vertices
            .iter()
            .permutations(self.vertices.len())
            .collect();

        let mut scores = Vec::new();
        for permutation in permutations {
            let mut cost = 0;
            for i in 0..permutation.len() - 1 {
                cost += self.cost(permutation[i], permutation[i + 1]);
            }
            scores.push(cost);
        }
        scores.iter().max().unwrap().to_owned()
    }
}

#[test]
fn test_shortest_path() {
    let mut graph = Graph::new();
    graph.add_edge("London", "Dublin", 464);
    graph.add_edge("London", "Belfast", 518);
    graph.add_edge("Dublin", "Belfast", 141);

    let min = graph.shortest_path();
    assert_eq!(min, 605);
}

#[test]
fn test_longest_path() {
    let mut graph = Graph::new();
    graph.add_edge("London", "Dublin", 464);
    graph.add_edge("London", "Belfast", 518);
    graph.add_edge("Dublin", "Belfast", 141);

    let min = graph.longest_path();
    assert_eq!(min, 982);
}

#[test]
fn test_year_2015_day_9() {
    println!("Advent of Code 2015 - Day 9");
    let contents =
        fs::read_to_string("input/2015/day-9.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    for line in contents.lines() {
        let s: Vec<&str> = line.split(' ').collect();
        let source = s[0];
        let destination = s[2];
        let cost = s[4].parse::<u32>().unwrap();
        graph.add_edge(source, destination, cost);
    }

    let min = graph.shortest_path();
    println!("The distance of the shortest route is {}.", min);
    assert_eq!(min, 207);

    let max = graph.longest_path();
    println!("The distance of the longest route is {}.", max);
    assert_eq!(max, 804);
}

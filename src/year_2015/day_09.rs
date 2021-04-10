use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

pub struct Graph {
    vertices: HashSet<String>,
    edges: HashMap<[String; 2], u32>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.vertices.insert(edge.source.clone());
        self.vertices.insert(edge.destination.clone());
        let mut edge_key = [edge.source, edge.destination];
        edge_key.sort();
        self.edges.insert(edge_key, edge.weight);
    }

    fn weight(&self, source: &str, destination: &str) -> &u32 {
        let mut edge_key = [String::from(source), String::from(destination)];
        edge_key.sort();
        self.edges.get(&edge_key).unwrap()
    }

    fn all_paths(&self) -> Vec<u32> {
        let permutations: Vec<Vec<&String>> = self
            .vertices
            .iter()
            .permutations(self.vertices.len())
            .collect();

        let mut scores = Vec::new();
        for permutation in permutations {
            let mut cost = 0;
            for i in 0..permutation.len() - 1 {
                cost += self.weight(permutation[i], permutation[i + 1]);
            }
            scores.push(cost);
        }
        scores
    }

    pub fn shortest_path(&self) -> u32 {
        let scores = self.all_paths();
        scores.iter().min().unwrap().to_owned()
    }

    pub fn longest_path(&self) -> u32 {
        let scores = self.all_paths();
        scores.iter().max().unwrap().to_owned()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new()
    }
}

pub struct Edge {
    source: String,
    destination: String,
    weight: u32,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(' ').collect();
        let source = s[0].to_string();
        let destination = s[2].to_string();
        let weight = s[4].parse::<u32>().unwrap();

        Ok(Edge {
            source,
            destination,
            weight,
        })
    }
}

#[test]
fn test_shortest_path() {
    let mut graph = Graph::new();

    let edge = Edge::from_str("London to Dublin = 464").unwrap();
    graph.add_edge(edge);
    let edge = Edge::from_str("London to Belfast = 518").unwrap();
    graph.add_edge(edge);
    let edge = Edge::from_str("Dublin to Belfast = 141").unwrap();
    graph.add_edge(edge);

    let min = graph.shortest_path();
    assert_eq!(min, 605);
}

#[test]
fn test_longest_path() {
    let mut graph = Graph::new();

    let edge = Edge::from_str("London to Dublin = 464").unwrap();
    graph.add_edge(edge);
    let edge = Edge::from_str("London to Belfast = 518").unwrap();
    graph.add_edge(edge);
    let edge = Edge::from_str("Dublin to Belfast = 141").unwrap();
    graph.add_edge(edge);

    let min = graph.longest_path();
    assert_eq!(min, 982);
}

#[test]
fn test_shortest_path_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-9.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    contents
        .lines()
        .map(|line| Edge::from_str(line).unwrap())
        .for_each(|edge| graph.add_edge(edge));

    let min = graph.shortest_path();
    assert_eq!(min, 207);
}

#[test]
fn test_longest_path_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-9.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    contents
        .lines()
        .map(|line| Edge::from_str(line).unwrap())
        .for_each(|edge| graph.add_edge(edge));

    let max = graph.longest_path();
    assert_eq!(max, 804);
}

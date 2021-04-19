use std::str::FromStr;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

use crate::util::CapturesWrapper;

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
        self.vertices.insert(edge.origin.clone());
        self.vertices.insert(edge.destination.clone());
        let mut edge_key = [edge.origin, edge.destination];
        edge_key.sort();
        self.edges.insert(edge_key, edge.weight);
    }

    fn weight(&self, origin: &str, destination: &str) -> &u32 {
        let mut edge_key = [String::from(origin), String::from(destination)];
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
    origin: String,
    destination: String,
    weight: u32,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Edge, ()> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(?P<origin>\w+) to (?P<destination>\w+) = (?P<weight>\d+)$").unwrap();
        }
        if let Some(caps) = REGEX.captures(s) {
            let caps = CapturesWrapper::new(caps);
            let origin = caps.as_string("origin");
            let destination = caps.as_string("destination");
            let weight = caps.parse("weight");

            return Ok(Edge {
                origin,
                destination,
                weight,
            });
        };
        Err(())
    }
}

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Neg;
use std::str::FromStr;

use itertools::Itertools;

pub struct SeatingPlan {
    pub guests: HashSet<String>,
    preferences: HashMap<[String; 2], i32>,
}

impl SeatingPlan {
    pub fn new() -> SeatingPlan {
        SeatingPlan {
            guests: HashSet::new(),
            preferences: HashMap::new(),
        }
    }

    pub fn add_preference(&mut self, preference: SeatingPreference) {
        self.guests.insert(preference.guest.clone());
        let key = [preference.guest, preference.neighbour];
        self.preferences.insert(key, preference.happiness);
    }

    fn happiness(&self, guest: &str, neighbour: &str) -> &i32 {
        let key = [String::from(guest), String::from(neighbour)];
        self.preferences.get(&key).unwrap()
    }

    pub fn happiest_table(&self) -> i32 {
        let permutations: Vec<Vec<&String>> =
            self.guests.iter().permutations(self.guests.len()).collect();

        let mut scores = Vec::new();
        for permutation in permutations {
            let mut happiness = 0;
            for i in 0..permutation.len() {
                let left = if i == 0 {
                    permutation[permutation.len() - 1]
                } else {
                    permutation[i - 1]
                };
                let right = if i == permutation.len() - 1 {
                    permutation[0]
                } else {
                    permutation[i + 1]
                };
                happiness += self.happiness(permutation[i], left);
                happiness += self.happiness(permutation[i], right);
            }
            scores.push(happiness);
        }
        scores.iter().max().unwrap().to_owned()
    }
}

impl Default for SeatingPlan {
    fn default() -> Self {
        SeatingPlan::new()
    }
}

pub struct SeatingPreference {
    guest: String,
    neighbour: String,
    happiness: i32,
}

impl FromStr for SeatingPreference {
    type Err = ();

    fn from_str(s: &str) -> Result<SeatingPreference, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<guest>\w+) would (?P<operative>gain|lose) (?P<happiness>\d+) happiness units by sitting next to (?P<neighbour>\w+).$").unwrap();
        }
        match REGEX.captures(s) {
            Some(caps) => {
                let guest = caps.name("guest").unwrap().as_str().to_string();
                let neighbour = caps.name("neighbour").unwrap().as_str().to_string();
                let operative_word = caps.name("operative").unwrap().as_str();
                let mut happiness = caps
                    .name("happiness")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                if operative_word == "lose" {
                    happiness = happiness.neg();
                }
                Ok(SeatingPreference {
                    guest,
                    neighbour,
                    happiness,
                })
            }
            None => Err(()),
        }
    }
}

#[test]
fn test_happiest_table() {
    let mut plan = SeatingPlan::new();

    let preference =
        SeatingPreference::from_str("Alice would gain 54 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Alice would lose 79 happiness units by sitting next to Carol.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Alice would lose 2 happiness units by sitting next to David.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would gain 83 happiness units by sitting next to Alice.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would lose 7 happiness units by sitting next to Carol.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would lose 63 happiness units by sitting next to David.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Carol would lose 62 happiness units by sitting next to Alice.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Carol would gain 60 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Carol would gain 55 happiness units by sitting next to David.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "David would gain 46 happiness units by sitting next to Alice.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("David would lose 7 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "David would gain 41 happiness units by sitting next to Carol.",
    )
    .unwrap();
    plan.add_preference(preference);

    let max = plan.happiest_table();
    assert_eq!(max, 330);
}

#[test]
fn test_happiest_table_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();
    contents
        .lines()
        .map(|line| SeatingPreference::from_str(line).unwrap())
        .for_each(|preference| plan.add_preference(preference));

    let max = plan.happiest_table();
    assert_eq!(max, 664);
}

#[test]
fn test_happiest_table_input_file_and_you() {
    let contents =
        std::fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();
    contents
        .lines()
        .map(|line| SeatingPreference::from_str(line).unwrap())
        .for_each(|preference| plan.add_preference(preference));

    for guest in plan.guests.clone() {
        let s = format!(
            "You would gain 0 happiness units by sitting next to {}.",
            guest
        );
        let preference = SeatingPreference::from_str(&s).unwrap();
        plan.add_preference(preference);

        let s = format!(
            "{} would gain 0 happiness units by sitting next to You.",
            guest
        );
        let preference = SeatingPreference::from_str(&s).unwrap();
        plan.add_preference(preference);
    }

    let max = plan.happiest_table();
    assert_eq!(max, 640)
}

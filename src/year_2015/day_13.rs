use std::ops::Neg;
use std::str::FromStr;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::CapturesWrapper;

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
        static REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<guest>\w+) would (?P<operative>gain|lose) (?P<happiness>\d+) happiness units by sitting next to (?P<neighbour>\w+).$").unwrap()
        });

        match REGEX.captures(s) {
            Some(caps) => {
                let caps = CapturesWrapper::new(caps);
                let guest = caps.as_string("guest");
                let neighbour = caps.as_string("neighbour");
                let operative_word = caps.as_str("operative");
                let mut happiness = caps.parse::<i32>("happiness");
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

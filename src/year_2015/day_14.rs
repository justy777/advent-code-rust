use std::str::FromStr;

use hashbrown::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::CapturesWrapper;

pub struct Reindeer {
    name: String,
    flying_speed: u32,
    flying_time: u32,
    rest_time: u32,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<name>\w+) can fly (?P<flying_speed>\d+) km/s for (?P<flying_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.$").unwrap()
        });

        match REGEX.captures(s) {
            Some(caps) => {
                let caps = CapturesWrapper::new(caps);
                let name = caps.as_string("name");
                let flying_speed = caps.parse("flying_speed");
                let flying_time = caps.parse("flying_time");
                let rest_time = caps.parse("rest_time");

                Ok(Reindeer {
                    name,
                    flying_speed,
                    flying_time,
                    rest_time,
                })
            }
            None => Err(()),
        }
    }
}

/// # Panics
///
/// Will panic if reindeer is empty
#[must_use]
pub fn distance_winning_reindeer_traveled(reindeer: &[Reindeer], race_time: u32) -> u32 {
    if reindeer.is_empty() {
        panic!("Illegal argument - reindeer slice cannot be empty")
    }

    let mut distances = Vec::new();
    for racer in reindeer {
        let mut time_left = race_time;
        let mut distance: u32 = 0;
        while time_left != 0 {
            let flying_time = if time_left >= racer.flying_time {
                racer.flying_time
            } else {
                time_left
            };
            distance += racer.flying_speed * flying_time;
            time_left -= flying_time;
            let rest_time = if time_left >= racer.rest_time {
                racer.rest_time
            } else {
                time_left
            };
            time_left -= rest_time;
        }
        distances.push(distance);
    }
    distances.iter().max().unwrap().to_owned()
}

#[derive(Debug)]
enum State {
    Flying(u32),
    Resting(u32),
}

#[derive(Debug)]
struct Racer {
    state: State,
    distance: u32,
    points: u32,
}

impl Racer {
    fn new() -> Racer {
        Racer {
            state: State::Flying(0),
            distance: 0,
            points: 0,
        }
    }
}

impl Default for Racer {
    fn default() -> Racer {
        Racer::new()
    }
}

#[must_use]
/// # Panics
///
/// Will panic if reindeer is empty
pub fn points_awarded_winning_reindeer(reindeer: &[Reindeer], race_time: u32) -> u32 {
    if reindeer.is_empty() {
        panic!("illegal argument - reindeer slice cannot be empty");
    }

    let mut racers: HashMap<String, Racer> = HashMap::new();
    for _ in 0..race_time {
        for deer in reindeer {
            let mut racer = racers.entry(deer.name.clone()).or_default();
            match racer.state {
                State::Flying(n) => {
                    racer.state = if n == deer.flying_time {
                        State::Resting(1)
                    } else {
                        racer.distance += deer.flying_speed;
                        State::Flying(n + 1)
                    };
                }
                State::Resting(n) => {
                    racer.state = if n == deer.rest_time {
                        racer.distance += deer.flying_speed;
                        State::Flying(1)
                    } else {
                        State::Resting(n + 1)
                    };
                }
            }
        }
        if let Some(lead_distance) = racers.values().map(|racer| racer.distance).max() {
            racers
                .values_mut()
                .filter(|racer| racer.distance == lead_distance)
                .for_each(|racer| racer.points += 1);
        }
    }
    racers.values().map(|racer| racer.points).max().unwrap()
}

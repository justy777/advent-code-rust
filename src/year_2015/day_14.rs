use std::str::FromStr;

use regex::Regex;

pub struct Reindeer {
    name: String,
    flying_speed: u32,
    flying_time: u32,
    rest_time: u32,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<name>\w+) can fly (?P<flying_speed>\d+) km/s for (?P<flying_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.$").unwrap();
        }
        match REGEX.captures(s) {
            Some(caps) => {
                let name = caps.name("name").unwrap().as_str().to_string();
                let flying_speed = caps
                    .name("flying_speed")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let flying_time = caps
                    .name("flying_time")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let rest_time = caps
                    .name("rest_time")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

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

pub fn distance_winning_reindeer_traveled(reindeer: &[Reindeer], race_time: u32) -> u32 {
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

#[test]
fn test_distance_winning_reindeer_traveled() {
    let mut reindeer = Vec::new();
    reindeer.push(
        Reindeer::from_str(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        )
        .unwrap(),
    );
    reindeer.push(
        Reindeer::from_str(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        )
        .unwrap(),
    );

    let max = distance_winning_reindeer_traveled(&reindeer, 1000);
    assert_eq!(max, 1120);
}

#[test]
fn test_distance_winning_reindeer_traveled_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    let reindeer: Vec<Reindeer> = contents
        .lines()
        .map(|line| Reindeer::from_str(line).unwrap())
        .collect();

    let max = distance_winning_reindeer_traveled(&reindeer, 2503);
    assert_eq!(max, 2640);
}

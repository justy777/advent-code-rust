use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_14::{
    distance_winning_reindeer_traveled, points_awarded_winning_reindeer, Reindeer,
};

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
fn test_points_awarded_winning_reindeer() {
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

    let max = points_awarded_winning_reindeer(&reindeer, 1000);
    assert_eq!(max, 689);
}

#[test]
fn test_distance_winning_reindeer_traveled_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    let reindeer: Vec<Reindeer> = contents
        .lines()
        .map(|line| Reindeer::from_str(line).unwrap())
        .collect();

    let max = distance_winning_reindeer_traveled(&reindeer, 2503);
    assert_eq!(max, 2640);
}

#[test]
fn test_points_awarded_winning_reindeer_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    let reindeer: Vec<Reindeer> = contents
        .lines()
        .map(|line| Reindeer::from_str(line).unwrap())
        .collect();

    let max = points_awarded_winning_reindeer(&reindeer, 2503);
    assert_eq!(max, 1102);
}

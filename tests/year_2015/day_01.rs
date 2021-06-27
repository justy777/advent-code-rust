use std::fs;

use advent_of_code::year_2015::day_01::{floor, position_to_floor};

#[test]
fn test_floor_bad_input() {
    let floor = floor(b"pancakes");
    assert_eq!(floor, 0);
}

#[test]
fn test_floor_empty() {
    let floor = floor(b"");
    assert_eq!(floor, 0);
}

#[test]
fn test_position_bad_input_and_basement_floor() {
    let position = position_to_floor(b"pancakes", -1);
    assert_eq!(position, None);
}

#[test]
fn test_position_empty_and_basement_floor() {
    let position = position_to_floor(b"", -1);
    assert_eq!(position, None);
}

#[test]
fn test_position_bad_input_and_ground_floor() {
    let position = position_to_floor(b"pancakes", 0);
    assert_eq!(position, Some(0));
}

#[test]
fn test_position_empty_and_ground_floor() {
    let position = position_to_floor(b"", 0);
    assert_eq!(position, Some(0));
}

#[test]
fn test_floor_input_file() {
    let contents = fs::read("input/2015/day-01.txt").expect("Failed to read file.");

    let floor = floor(&contents);
    assert_eq!(floor, 280);
}

#[test]
fn test_position_input_file() {
    let contents = fs::read("input/2015/day-01.txt").expect("Failed to read file.");

    let position = position_to_floor(&contents, -1);
    assert_eq!(position, Some(1797));
}

use std::fs;

use advent_of_code::year_2015::day_08::{escape_string, reformat_string};

#[test]
fn test_reformat_string() {
    let list = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];
    let before: usize = list.iter().map(|s| s.len()).sum();
    let after: usize = list.iter().map(|s| reformat_string(s).len()).sum();
    assert_eq!(before, 23);
    assert_eq!(after, 11);
}

#[test]
fn test_escape_string() {
    let list = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];
    let before: usize = list.iter().map(|s| s.len()).sum();
    let after: usize = list.iter().map(|s| escape_string(s).len()).sum();
    assert_eq!(before, 23);
    assert_eq!(after, 42);
}

#[test]
fn test_reformat_string_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-08.txt").expect("Failed to read file to string.");

    let before: usize = contents.lines().map(|s| s.len()).sum();
    let after: usize = contents.lines().map(|s| reformat_string(s).len()).sum();
    assert_eq!(before - after, 1333);
}

#[test]
fn test_escape_string_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-08.txt").expect("Failed to read file to string.");

    let before: usize = contents.lines().map(|s| s.len()).sum();
    let after: usize = contents.lines().map(|s| escape_string(s).len()).sum();
    assert_eq!(after - before, 2046);
}

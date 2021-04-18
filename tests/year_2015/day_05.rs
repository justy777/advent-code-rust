use std::fs;

use advent_of_code::year_2015::day_05::{is_nice_word, is_nice_word2};

#[test]
fn test_is_nice_word_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-5.txt").expect("Failed to read file to string.");

    let count = contents.lines().filter(|s| is_nice_word(s)).count();
    assert_eq!(count, 236);
}

#[test]
fn test_is_nice_word2_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-5.txt").expect("Failed to read file to string.");

    let count = contents.lines().filter(|s| is_nice_word2(s)).count();
    assert_eq!(count, 51);
}

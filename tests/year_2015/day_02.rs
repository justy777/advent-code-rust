use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_02::Present;

#[test]
fn test_present_from_str_bad_input() {
    match Present::from_str("hjhjxjhjhxikjk") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("1x1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("1x1x1x1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("-1x-1x-1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }
}

#[test]
fn test_present_from_str_zero() {
    let present = Present::from_str("0x0x0").unwrap();
    assert_eq!(present.wrapping_paper_needed(), 0);
    assert_eq!(present.ribbon_needed(), 0);
}

#[test]
fn test_wrapping_paper_needed_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file to string.");

    let presents: Vec<Present> = contents
        .lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let wrapping_paper_needed: u32 = presents
        .iter()
        .map(|present| present.wrapping_paper_needed())
        .sum();

    assert_eq!(wrapping_paper_needed, 1586300);
}

#[test]
fn test_ribbon_needed_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file to string.");

    let presents: Vec<Present> = contents
        .lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let ribbon_needed: u32 = presents.iter().map(|present| present.ribbon_needed()).sum();

    assert_eq!(ribbon_needed, 3737498);
}

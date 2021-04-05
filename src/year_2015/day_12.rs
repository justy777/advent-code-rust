use std::fs;

use regex::Regex;

fn sum_numbers_in_string(s: &str) -> i32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"-?\d+").unwrap();
    }
    REGEX
        .find_iter(s)
        .map(|mat| mat.as_str().parse::<i32>().unwrap())
        .sum()
}

#[test]
fn test_sum_numbers() {
    let sum = sum_numbers_in_string("[1,2,3]");
    assert_eq!(sum, 6);

    let sum = sum_numbers_in_string(r#"{"a":2,"b":4}"#);
    assert_eq!(sum, 6);

    let sum = sum_numbers_in_string("[[[3]]]");
    assert_eq!(sum, 3);

    let sum = sum_numbers_in_string(r#"{"a":{"b":4},"c":-1}"#);
    assert_eq!(sum, 3);

    let sum = sum_numbers_in_string(r#"{"a":[-1,1]}"#);
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_string(r#"[-1,{"a":1}]"#);
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_string("[]");
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_string("{}");
    assert_eq!(sum, 0);
}

#[test]
fn test_year_2015_day_12() {
    println!("Advent of Code 2015 - Day 12");
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    let sum = sum_numbers_in_string(&contents);
    println!("The sum of all the numbers in the document is {}.", sum);
    assert_eq!(sum, 156366);
}

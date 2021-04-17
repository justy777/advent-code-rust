use std::fs;

use advent_of_code::year_2015::day_12::{sum_numbers_in_str, sum_value};

#[test]
fn test_sum_numbers() {
    let sum = sum_numbers_in_str("[1,2,3]");
    assert_eq!(sum, 6);

    let sum = sum_numbers_in_str(r#"{"a":2,"b":4}"#);
    assert_eq!(sum, 6);

    let sum = sum_numbers_in_str("[[[3]]]");
    assert_eq!(sum, 3);

    let sum = sum_numbers_in_str(r#"{"a":{"b":4},"c":-1}"#);
    assert_eq!(sum, 3);

    let sum = sum_numbers_in_str(r#"{"a":[-1,1]}"#);
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_str(r#"[-1,{"a":1}]"#);
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_str("[]");
    assert_eq!(sum, 0);

    let sum = sum_numbers_in_str("{}");
    assert_eq!(sum, 0);
}

#[test]
fn test_sum_value() {
    let value = serde_json::from_str("[1,2,3]").unwrap();
    let sum = sum_value(&value);
    assert_eq!(sum, 6);

    let value = serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap();
    let sum = sum_value(&value);
    assert_eq!(sum, 4);

    let value = serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap();
    let sum = sum_value(&value);
    assert_eq!(sum, 0);

    let value = serde_json::from_str(r#"[1,"red",5]"#).unwrap();
    let sum = sum_value(&value);
    assert_eq!(sum, 6);
}

#[test]
fn test_year_2015_day_12() {
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    let sum = sum_numbers_in_str(&contents);
    assert_eq!(sum, 156366);
}

#[test]
fn test_test_sum_value_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    let value = serde_json::from_str(&contents).unwrap();
    let sum = sum_value(&value);
    assert_eq!(sum, 96852);
}

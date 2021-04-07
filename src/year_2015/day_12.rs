use std::fs;

use regex::Regex;
use serde_json::Value;

fn sum_numbers_in_str(s: &str) -> i32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"-?\d+").unwrap();
    }
    REGEX
        .find_iter(s)
        .map(|mat| mat.as_str().parse::<i32>().unwrap())
        .sum()
}

fn sum_value(value: &Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.iter().map(|value| sum_value(value)).sum(),
        Value::Object(map) => {
            let red = map.values().any(|value| match value {
                Value::String(s) => s == "red",
                _ => false,
            });

            if red {
                return 0;
            }

            map.values().map(|value| sum_value(value)).sum()
        }
    }
}

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
    println!("Advent of Code 2015 - Day 12");
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    let sum = sum_numbers_in_str(&contents);
    println!("The sum of all the numbers in the document is {}.", sum);
    assert_eq!(sum, 156366);

    let value = serde_json::from_str(&contents).unwrap();
    let sum = sum_value(&value);
    println!("The sum of the numbers without red objects is {}.", sum);
    assert_eq!(sum, 96852);
}

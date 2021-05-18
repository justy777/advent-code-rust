use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

pub fn sum_numbers_in_str(s: &str) -> i32 {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?\d+").unwrap());

    REGEX
        .find_iter(s)
        .map(|mat| mat.as_str().parse::<i32>().unwrap())
        .sum()
}

pub fn sum_value(value: &Value) -> i64 {
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

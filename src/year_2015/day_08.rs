use std::fs;

use regex::Regex;

fn reformat_string(before: &str) -> String {
    lazy_static! {
        static ref DOUBLE_QUOTES_REGEX: Regex = Regex::new(r#""(?P<s>\S+)?""#).unwrap();
        static ref HEX_REGEX: Regex = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();
        static ref ESCAPED_REGEX: Regex = Regex::new(r"\\(?P<s>\S)").unwrap();
    }
    let s = DOUBLE_QUOTES_REGEX.replace(&before, "$s");
    let s = HEX_REGEX.replace_all(&s, "'");
    ESCAPED_REGEX.replace_all(&s, "$s").to_string()
}

fn escape_string(before: &str) -> String {
    lazy_static! {
        static ref ESCAPE_REGEX: Regex = Regex::new(r#"(?P<s>[\\"])"#).unwrap();
        static ref DOUBLE_QUOTES_REGEX: Regex = Regex::new(r"(?P<s>\S+)").unwrap();
    }
    let s = ESCAPE_REGEX.replace_all(&before, "\\$s");
    DOUBLE_QUOTES_REGEX.replace(&s, "\"$s\"").to_string()
}

#[test]
fn test_reformat_string() {
    let list = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];
    let before: usize = list.iter().map(|s| s.len()).sum();
    let after: usize = list.iter().map(|s| reformat_string(s).len()).sum();
    assert_eq!(before, 23);
    assert_eq!(after, 11);
}

#[test]
fn test_escape_string() {
    let list = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];
    let before: usize = list.iter().map(|s| s.len()).sum();
    let after: usize = list.iter().map(|s| escape_string(s).len()).sum();
    assert_eq!(before, 23);
    assert_eq!(after, 42);
}

#[test]
fn test_2015_day_8() {
    println!("Advent of Code 2015 - Day 8");
    let contents =
        fs::read_to_string("input/2015/day-8.txt").expect("Failed to read file to string.");

    let before: usize = contents.lines().map(|s| s.len()).sum();
    let after: usize = contents.lines().map(|s| reformat_string(s).len()).sum();

    println!("The difference between the string literals characters and the string value characters in memory is {} characters.", before - after);
    assert_eq!(before - after, 1333);

    let after: usize = contents.lines().map(|s| escape_string(s).len()).sum();

    println!("The difference between the newly encoded string characters and the string literals characters is {} characters.", after - before);
    assert_eq!(after - before, 2046);
}

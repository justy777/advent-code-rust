use std::str;

fn rotate_letter(c: u8, n: u8) -> u8 {
    (((c - b'a') + n) % 26) + b'a'
}

fn contains_increasing_straight_of_three(s: &[u8]) -> bool {
    if s.len() < 3 {
        return false;
    }

    let mut iter = s.iter();
    let mut last_char = iter.next().unwrap();
    let mut count = 1;
    for c in iter {
        if *last_char + 1 == *c {
            count += 1;
        } else {
            count = 1;
        }
        if count == 3 {
            break;
        }
        last_char = c;
    }
    count == 3
}

fn contains_forbidden_char(s: &[u8]) -> bool {
    s.iter().any(|c| "iol".as_bytes().contains(c))
}

fn contains_two_pairs(s: &[u8]) -> bool {
    if s.len() < 4 {
        return false;
    }

    let mut iter = s.iter();
    let mut last_char = None;
    let mut count = 0;
    while let Some(c) = iter.next() {
        if last_char == Some(c) {
            count += 1;
            last_char = iter.next();
            continue;
        }
        if count == 2 {
            break;
        }
        last_char = Some(c);
    }
    count == 2
}

fn is_password(s: &[u8]) -> bool {
    contains_increasing_straight_of_three(s) && !contains_forbidden_char(s) && contains_two_pairs(s)
}

fn next_password(old_password: &str) -> String {
    let mut password = Vec::from(old_password);
    if let Some(start_position) = password.iter().position(|c| "iol".as_bytes().contains(c)) {
        password[start_position] = rotate_letter(password[start_position], 1);
        for letter in password.iter_mut().skip(start_position + 1) {
            *letter = b'a';
        }
    };
    let mut position = password.len() - 1;
    while !is_password(&password) || old_password.as_bytes() == password.as_slice() {
        let letter = rotate_letter(password[position], 1);
        password[position] = letter;
        if letter == b'z' {
            position -= 1;
            continue;
        }
        if position < old_password.len() - 1 {
            position += 1;
        }
    }

    String::from_utf8(password).unwrap()
}

#[test]
fn test1() {
    let new_password = next_password("abcdefgh");
    assert_eq!(new_password, "abcdffaa");
}

#[test]
fn test2() {
    let new_password = next_password("ghijklmn");
    assert_eq!(new_password, "ghjaabcc");
}

#[test]
fn test_year_2015_day_11() {
    println!("Advent of Code 2015 - Day 11");
    let new_password = next_password("cqjxjnds");
    println!("His next password should be {}.", new_password);
    assert_eq!(new_password, "cqjxxyzz");

    let new_password = next_password("cqjxxyzz");
    println!("The next one is {}.", new_password);
    assert_eq!(new_password, "cqkaabcc");
}

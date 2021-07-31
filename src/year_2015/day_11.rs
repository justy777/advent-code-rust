use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{fmt, str};

fn rotate_letters(letters: &mut [u8]) {
    let mut first = true;
    for letter in letters {
        if first {
            *letter = (((*letter - b'a') + 1) % 26) + b'a';
            first = false;
        } else {
            *letter = b'a';
        }
    }
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
    s.iter().any(|c| b"iol".contains(c))
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

/// Error type used when calling `next_password`.
#[derive(Debug)]
pub struct NextPasswordError {
    pub(super) _priv: (),
}

impl Error for NextPasswordError {}

impl Display for NextPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not in format '[a-z]{4,}'".fmt(f)
    }
}

/// # Errors
///
/// Will result in an error if `old_password` is less than 4 characters, or if `old_password` contains any non-alphabetic characters
pub fn next_password(old_password: &str) -> Result<String, NextPasswordError> {
    if old_password.len() < 4 {
        return Result::Err(NextPasswordError { _priv: () });
    }

    if !old_password.chars().all(char::is_alphabetic) {
        return Result::Err(NextPasswordError { _priv: () });
    }

    let mut password = Vec::from(old_password);
    if let Some(start_position) = password.iter().position(|c| b"iol".contains(c)) {
        rotate_letters(&mut password[start_position..]);
    };

    while !is_password(&password) || old_password.as_bytes() == password.as_slice() {
        if let Some(position) = password.iter().rposition(|letter| letter != &b'z') {
            rotate_letters(&mut password[position..]);
        }
    }

    unsafe { Result::Ok(String::from_utf8_unchecked(password)) }
}

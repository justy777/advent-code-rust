use std::str;

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

pub fn next_password(old_password: &str) -> String {
    let mut password = Vec::from(old_password);
    if let Some(start_position) = password.iter().position(|c| "iol".as_bytes().contains(c)) {
        rotate_letters(&mut password[start_position..]);
    };

    while !is_password(&password) || old_password.as_bytes() == password.as_slice() {
        let position = password.iter().rposition(|letter| letter != &b'z').unwrap();
        rotate_letters(&mut password[position..]);
    }

    String::from_utf8(password).unwrap()
}

#[test]
fn test_next_password() {
    let new_password = next_password("abcdefgh");
    assert_eq!(new_password, "abcdffaa");
}

#[test]
fn test_next_password_forbidden_letter() {
    let new_password = next_password("ghijklmn");
    assert_eq!(new_password, "ghjaabcc");
}

#[test]
fn test_next_password_input() {
    let new_password = next_password("cqjxjnds");
    assert_eq!(new_password, "cqjxxyzz");
}

#[test]
fn test_next_password_input2() {
    let new_password = next_password("cqjxxyzz");
    assert_eq!(new_password, "cqkaabcc");
}

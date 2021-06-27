use advent_of_code::year_2015::day_11::next_password;

#[test]
fn test_next_password() {
    let new_password = next_password("abcdefgh").unwrap();
    assert_eq!(new_password, "abcdffaa");
}

#[test]
fn test_next_password_forbidden_letter() {
    let new_password = next_password("ghijklmn").unwrap();
    assert_eq!(new_password, "ghjaabcc");
}

#[test]
fn test_next_password_input() {
    let new_password = next_password("cqjxjnds").unwrap();
    assert_eq!(new_password, "cqjxxyzz");
}

#[test]
fn test_next_password_input2() {
    let new_password = next_password("cqjxxyzz").unwrap();
    assert_eq!(new_password, "cqkaabcc");
}

#[test]
fn test_next_password_empty() {
    let new_password = next_password("");
    assert!(new_password.is_err());
}

#[test]
fn test_next_password_numbers() {
    let new_password = next_password("3456");
    assert!(new_password.is_err());
}

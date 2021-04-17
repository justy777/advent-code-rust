use advent_of_code::year_2015::day_11::next_password;

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

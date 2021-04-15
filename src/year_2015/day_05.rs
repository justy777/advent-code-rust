/*!
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.
*/

/// Counts the vowels in the provided `str`.
fn count_vowels(s: &str) -> usize {
    lazy_static! {
        static ref VOWELS: &'static str = "aeiou";
    }
    s.chars().filter(|c| VOWELS.contains(*c)).count()
}

/// Returns `true` if str contains double letters; `false` otherwise.
fn contains_double_letter(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }

    let mut last_char = None;
    let mut found = false;
    for c in s.chars() {
        if last_char == Some(c) {
            found = true;
            break;
        }
        last_char = Some(c);
    }
    found
}

/// Returns `true` if str contains any of the forbidden strings; `false` otherwise.
///
/// The forbidden strings are `ab`, `cd`, `pq`, and `xy`.
fn contains_forbidden_str(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }

    lazy_static! {
        static ref FORBIDDEN_STRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];
    }
    FORBIDDEN_STRINGS.iter().any(|x| s.contains(x))
}

/// Returns `true` if str has all the properties of a nice word.
///
/// Nice words must:
/// - Contains at least 3 vowels
/// - Contains at least one letter that appears twice in a row
/// - Does not contain any of the forbidden strings.
///
/// # Examples
///
/// ```
/// use advent_of_code::year_2015::day_05::is_nice_word;
///
/// let is_nice = is_nice_word("ugknbfddgicrmopn");
/// assert!(is_nice);
/// ```
pub fn is_nice_word(s: &str) -> bool {
    count_vowels(s) >= 3 && contains_double_letter(s) && !contains_forbidden_str(s)
}

/// Returns `true` if contains a pair of two letters that appears at least twice in the provided str without overlapping; `false` otherwise.
fn contains_pair_of_letters_twice(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    let mut found = false;
    for i in 1..(s.len() - 2) {
        for j in (i + 1)..(s.len() - 1) {
            if s[(i - 1)..=i] == s[j..=(j + 1)] {
                found = true;
                break;
            }
        }
    }
    found
}

/// Returns `true` if contains at least one letter which repeats with exactly one letter between them; `false` otherwise.
fn contains_letter_that_repeats_with_letter_between(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let mut found = false;
    for i in 0..(s.len() - 2) {
        if s.chars().nth(i) == s.chars().nth(i + 2) {
            found = true;
            break;
        }
    }
    found
}

/// Returns `true` if str has all the properties of a nice word.
///
/// Nice words must:
/// - Contains a pair of any two letters that appears at least twice in the string without overlapping
/// - Contains at least one letter which repeats with exactly one letter between them
///
/// # Examples
///
/// ```
/// use advent_of_code::year_2015::day_05::is_nice_word2;
///
/// let is_nice = is_nice_word2("qjhvhtzxzqqjkmpb");
/// assert!(is_nice);
/// ```
pub fn is_nice_word2(s: &str) -> bool {
    contains_pair_of_letters_twice(s) && contains_letter_that_repeats_with_letter_between(s)
}

#[test]
fn test_is_nice_word() {
    assert!(is_nice_word("ugknbfddgicrmopn"));
    assert!(is_nice_word("aaa"));
    assert!(!is_nice_word("jchzalrnumimnmhp"));
    assert!(!is_nice_word("haegwjzuvuyypxyu"));
    assert!(!is_nice_word("dvszwmarrgswjxmb"));
}

#[test]
fn test_is_nice_word2() {
    assert!(is_nice_word2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice_word2("xxyxx"));
    assert!(!is_nice_word2("uurcxstgmygtbstg"));
    assert!(!is_nice_word2("ieodomkazucvgmuy"));
}

#[test]
fn test_is_nice_word_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-5.txt").expect("Failed to read file to string.");

    let count = contents.lines().filter(|word| is_nice_word(word)).count();
    assert_eq!(count, 236);
}

#[test]
fn test_is_nice_word2_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-5.txt").expect("Failed to read file to string.");

    let count = contents.lines().filter(|word| is_nice_word2(word)).count();
    assert_eq!(count, 51);
}

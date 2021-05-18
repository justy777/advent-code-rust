/*!
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.
*/

use once_cell::sync::Lazy;

/// Counts the vowels in the provided `str`.
fn count_vowels(s: &[u8]) -> usize {
    static VOWELS: Lazy<&'static [u8]> = Lazy::new(|| b"aeiou");
    s.iter().filter(|c| VOWELS.contains(*c)).count()
}

/// Returns `true` if str contains double letters; `false` otherwise.
fn contains_double_letter(s: &[u8]) -> bool {
    if s.len() < 2 {
        return false;
    }

    let mut last_char = None;
    let mut found = false;
    for c in s.iter() {
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

    static FORBIDDEN_STRINGS: Lazy<[&'static str; 4]> = Lazy::new(|| ["ab", "cd", "pq", "xy"]);
    FORBIDDEN_STRINGS.iter().any(|pat| s.contains(pat))
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
/// // It has 3 vowels (`u,i,o`), a double letter `dd`, and none of the disallowed strings.
/// let is_nice = is_nice_word("ugknbfddgicrmopn");
/// assert!(is_nice);
///
/// // Fills all the requirements even though the letters used by different rules overlap
/// let is_nice = is_nice_word("aaa");
/// assert!(is_nice);
///
/// // It has no double letter
/// let is_nice = is_nice_word("jchzalrnumimnmhp");
/// assert!(!is_nice);
///
/// // It contains the string `xy`
/// let is_nice = is_nice_word("haegwjzuvuyypxyu");
/// assert!(!is_nice);
///
/// // It contains only one vowel
/// let is_nice = is_nice_word("dvszwmarrgswjxmb");
/// assert!(!is_nice);
/// ```
pub fn is_nice_word(s: &str) -> bool {
    count_vowels(s.as_ref()) >= 3
        && contains_double_letter(s.as_ref())
        && !contains_forbidden_str(s)
}

/// Returns `true` if contains a pair of two letters that appears at least twice in the provided str without overlapping; `false` otherwise.
fn contains_pair_of_letters_twice(s: &[u8]) -> bool {
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
fn contains_letter_that_repeats_with_letter_between(s: &[u8]) -> bool {
    if s.len() < 3 {
        return false;
    }
    let mut found = false;
    for i in 0..(s.len() - 2) {
        if s[i] == s[i + 2] {
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
/// // It has a pair that appears twice `qj` and a letter that repeats with exactly one letter between them `zxz`
/// let is_nice = is_nice_word2("qjhvhtzxzqqjkmpb");
/// assert!(is_nice);
///
/// // Fills all requirements even though the letter used by different rules overlap
/// let is_nice = is_nice_word2("xxyxx");
/// assert!(is_nice);
///
/// // It has pair `tg`, but no repeat with a single letter between them.
/// let is_nice = is_nice_word2("uurcxstgmygtbstg");
/// assert!(!is_nice);
///
/// // It has a repeating letter with one between them `odo`, but no pair that appears twice.
/// let is_nice = is_nice_word2("ieodomkazucvgmuy");
/// assert!(!is_nice);
/// ```
pub fn is_nice_word2(s: &str) -> bool {
    contains_pair_of_letters_twice(s.as_ref())
        && contains_letter_that_repeats_with_letter_between(s.as_ref())
}

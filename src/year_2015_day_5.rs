use std::fs;

fn count_vowels(s: &str) -> usize {
    lazy_static! {
        static ref VOWELS: &'static str = "aeiou";
    }
    s.chars().filter(|c| VOWELS.contains(*c)).count()
}

fn contains_double_letter(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }

    let mut last_char = s.chars().nth(0).unwrap();
    let mut found = false;
    for c in s[1..].chars() {
        if last_char == c {
            found = true;
            break;
        }
        last_char = c;
    }
    found
}

fn contains_forbidden_strings(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }

    lazy_static! {
        static ref FORBIDDEN_STRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];
    }
    FORBIDDEN_STRINGS.iter().any(|x| s.contains(x))
}

fn is_nice_word(s: &str) -> bool {
    count_vowels(s) >= 3 && contains_double_letter(s) && !contains_forbidden_strings(s)
}

fn contains_pair_of_letters_twice(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    let mut found = false;
    for i in 1..(s.len() - 2) {
        for j in (i + 1)..(s.len() - 1) {
            if s[(i - 1)..=i] == s[j..=(j+1)] {
                found = true;
                break;
            }
        }
    }
    found
}

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

fn is_nice_word2(s: &str) -> bool {
    contains_pair_of_letters_twice(s) && contains_letter_that_repeats_with_letter_between(s)
}

#[test]
fn test_2015_day_5() {
    println!("Advent of Code 2015 - Day 5");
    let contents = fs::read_to_string("input/2015/day-5.txt")
        .expect("Failed to read file to String");

    let nice_word_count = contents.lines().filter(|line| is_nice_word(line)).count();
    println!("There are {} nice words with the first set of rules.", nice_word_count);
    assert_eq!(nice_word_count, 236);

    let nice_word_count = contents.lines().filter(|line| is_nice_word2(line)).count();
    println!("There are {} nice words with the second set of rules.", nice_word_count);
    assert_eq!(nice_word_count, 51);
}
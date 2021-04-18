use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_13::{SeatingPlan, SeatingPreference};

#[test]
fn test_happiest_table() {
    let contents =
        fs::read_to_string("input/2015/day-13-sample.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();

    contents
        .lines()
        .map(|s| SeatingPreference::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|preference| plan.add_preference(preference));

    let max = plan.happiest_table();
    assert_eq!(max, 330);
}

#[test]
fn test_happiest_table_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();
    contents
        .lines()
        .map(|s| SeatingPreference::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|preference| plan.add_preference(preference));

    let max = plan.happiest_table();
    assert_eq!(max, 664);
}

#[test]
fn test_happiest_table_input_file_and_you() {
    let contents =
        fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();
    contents
        .lines()
        .map(|s| SeatingPreference::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|preference| plan.add_preference(preference));

    for guest in plan.guests.clone() {
        let s = format!(
            "You would gain 0 happiness units by sitting next to {}.",
            guest
        );
        if let Ok(preference) = SeatingPreference::from_str(&s) {
            plan.add_preference(preference);
        }

        let s = format!(
            "{} would gain 0 happiness units by sitting next to You.",
            guest
        );
        if let Ok(preference) = SeatingPreference::from_str(&s) {
            plan.add_preference(preference);
        }
    }

    let max = plan.happiest_table();
    assert_eq!(max, 640)
}

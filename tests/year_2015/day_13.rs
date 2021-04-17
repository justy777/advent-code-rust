use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_13::{SeatingPlan, SeatingPreference};

#[test]
fn test_happiest_table() {
    let mut plan = SeatingPlan::new();

    let preference =
        SeatingPreference::from_str("Alice would gain 54 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Alice would lose 79 happiness units by sitting next to Carol.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Alice would lose 2 happiness units by sitting next to David.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would gain 83 happiness units by sitting next to Alice.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would lose 7 happiness units by sitting next to Carol.")
            .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Bob would lose 63 happiness units by sitting next to David.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Carol would lose 62 happiness units by sitting next to Alice.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("Carol would gain 60 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "Carol would gain 55 happiness units by sitting next to David.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "David would gain 46 happiness units by sitting next to Alice.",
    )
    .unwrap();
    plan.add_preference(preference);
    let preference =
        SeatingPreference::from_str("David would lose 7 happiness units by sitting next to Bob.")
            .unwrap();
    plan.add_preference(preference);
    let preference = SeatingPreference::from_str(
        "David would gain 41 happiness units by sitting next to Carol.",
    )
    .unwrap();
    plan.add_preference(preference);

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
        .map(|line| SeatingPreference::from_str(line).unwrap())
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
        .map(|line| SeatingPreference::from_str(line).unwrap())
        .for_each(|preference| plan.add_preference(preference));

    for guest in plan.guests.clone() {
        let s = format!(
            "You would gain 0 happiness units by sitting next to {}.",
            guest
        );
        let preference = SeatingPreference::from_str(&s).unwrap();
        plan.add_preference(preference);

        let s = format!(
            "{} would gain 0 happiness units by sitting next to You.",
            guest
        );
        let preference = SeatingPreference::from_str(&s).unwrap();
        plan.add_preference(preference);
    }

    let max = plan.happiest_table();
    assert_eq!(max, 640)
}

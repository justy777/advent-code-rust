use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_01::{floor, position};
use advent_of_code::year_2015::day_02::Present;
use advent_of_code::year_2015::day_03::InfiniteGrid;
use advent_of_code::year_2015::day_04::find_md5_hash_leading_zeroes;
use advent_of_code::year_2015::day_05::{is_nice_word, is_nice_word2};
use advent_of_code::year_2015::day_06::{DimmableBulb, LightGrid, LightInstruction, SimpleBulb};
use advent_of_code::year_2015::day_07::Circuit;
use advent_of_code::year_2015::day_08::{escape_string, reformat_string};
use advent_of_code::year_2015::day_09::{Edge, Graph};
use advent_of_code::year_2015::day_10::look_and_say;
use advent_of_code::year_2015::day_11::next_password;
use advent_of_code::year_2015::day_12::{sum_numbers_in_str, sum_value};
use advent_of_code::year_2015::day_13::{SeatingPlan, SeatingPreference};
use advent_of_code::year_2015::day_14::{
    distance_winning_reindeer_traveled, points_awarded_winning_reindeer, Reindeer,
};

fn main() {
    run_2015_01();
    run_2015_02();
    run_2015_03();
    run_2015_04();
    run_2015_05();
    run_2015_06();
    run_2015_07();
    run_2015_08();
    run_2015_09();
    run_2015_10();
    run_2015_11();
    run_2015_12();
    run_2015_13();
    run_2015_14();
}

/// Displays the solutions to Year 2015 Day 1: Not Quite Lisp.
fn run_2015_01() {
    println!("Advent of Code - Year 2015 Day 1: Not Quite Lisp");
    let contents = fs::read("input/2015/day-1.txt").expect("Failed to read file.");

    let f = floor(&contents);
    println!("The instructions take Santa to floor {}.", f);

    let p = position(&contents, -1).unwrap();
    println!("The Santa enters the basement at position {}.", p);
}

fn run_2015_02() {
    println!("Advent of Code 2015 - Day 2");
    let contents =
        fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file to string.");

    let presents: Vec<Present> = contents
        .lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let wrapping_paper_needed: u32 = presents
        .iter()
        .map(|present| present.wrapping_paper_needed())
        .sum();

    println!(
        "The elves need {} square feet of wrapping paper.",
        wrapping_paper_needed
    );

    let ribbon_needed: u32 = presents.iter().map(|present| present.ribbon_needed()).sum();

    println!("The elves need {} feet of ribbon.", ribbon_needed);
}

fn run_2015_03() {
    println!("Advent of Code 2015 - Day 3");
    let contents =
        fs::read_to_string("input/2015/day-3.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(1);

    contents.chars().for_each(|c| grid.move_position(c));

    let houses_visited = grid.position_count();
    println!("Santa visited {} houses at least once.", houses_visited);

    let mut grid = InfiniteGrid::new(2);

    contents.chars().for_each(|c| grid.move_position(c));

    let houses_visited = grid.position_count();
    println!(
        "Santa and Robo-Santa visited {} houses at least once.",
        houses_visited
    );
}

fn run_2015_04() {
    println!("Advent of Code 2015 - Day 4");
    let key = "iwrupvqb";

    let second_half_of_key = find_md5_hash_leading_zeroes(key.as_bytes(), 5).unwrap();
    println!(
        "The secret key is {}, and the answer is {} for an MD5 hash with five leading zeroes.",
        key, second_half_of_key
    );

    let second_half_of_key = find_md5_hash_leading_zeroes(key.as_bytes(), 6).unwrap();
    println!(
        "The secret key is {}, and the answer is {} for an MD5 hash with six leading zeroes.",
        key, second_half_of_key
    );
}

fn run_2015_05() {
    println!("Advent of Code 2015 - Day 5");
    let contents =
        fs::read_to_string("input/2015/day-5.txt").expect("Failed to read file to string.");

    let nice_word_count = contents.lines().filter(|word| is_nice_word(word)).count();
    println!(
        "There are {} nice words with the first set of rules.",
        nice_word_count
    );

    let nice_word_count = contents.lines().filter(|word| is_nice_word2(word)).count();
    println!(
        "There are {} nice words with the second set of rules.",
        nice_word_count
    );
}

fn run_2015_06() {
    println!("Advent of Code 2015 - Day 6");
    let contents =
        fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<SimpleBulb>::new();
    contents
        .lines()
        .map(|line| LightInstruction::from_str(line).unwrap())
        .for_each(|instruction| grid.apply_operation(instruction));
    let lit_lights_count = grid.total_brightness();
    println!("There are {} lights lit.", lit_lights_count);

    let mut grid = LightGrid::<DimmableBulb>::new();
    contents
        .lines()
        .map(|line| LightInstruction::from_str(line).unwrap())
        .for_each(|instruction| grid.apply_operation(instruction));
    let total_brightness = grid.total_brightness();
    println!("The total brightness is {}.", total_brightness);
}

fn run_2015_07() {
    println!("Advent of Code 2015 - Day 7");
    let contents =
        fs::read_to_string("input/2015/day-7.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents
        .lines()
        .for_each(|line| circuit.follow_instruction(line));
    circuit.resolve_circuit();

    let signal_a = circuit.signal("a").unwrap();
    println!("The signal {} is provided to wire 'a'.", signal_a);

    circuit.reset_circuit();
    circuit.follow_instruction("16076 -> b");
    circuit.resolve_circuit();

    let signal_a = circuit.signal("a").unwrap();
    println!("The signal {} is provided to wire 'a'.", signal_a);
}

fn run_2015_08() {
    println!("Advent of Code 2015 - Day 8");
    let contents =
        fs::read_to_string("input/2015/day-8.txt").expect("Failed to read file to string.");

    let before: usize = contents.lines().map(|s| s.len()).sum();
    let after: usize = contents.lines().map(|s| reformat_string(s).len()).sum();

    println!("The difference between the string literals characters and the string value characters in memory is {} characters.", before - after);

    let after: usize = contents.lines().map(|s| escape_string(s).len()).sum();

    println!("The difference between the newly encoded string characters and the string literals characters is {} characters.", after - before);
}

fn run_2015_09() {
    println!("Advent of Code 2015 - Day 9");
    let contents =
        std::fs::read_to_string("input/2015/day-9.txt").expect("Failed to read file to string.");

    let mut graph = Graph::new();
    contents
        .lines()
        .map(|line| Edge::from_str(line).unwrap())
        .for_each(|edge| graph.add_edge(edge));

    let min = graph.shortest_path();
    println!("The distance of the shortest route is {}.", min);

    let max = graph.longest_path();
    println!("The distance of the longest route is {}.", max);
}

fn run_2015_10() {
    println!("Advent of Code 2015 - Day 10");

    let mut input = String::from("1113122113");
    for _ in 0..40 {
        input = look_and_say(&input);
    }

    let length = input.len();
    println!("The length of the result is {}.", length);

    let mut input = String::from("1113122113");
    for _ in 0..50 {
        input = look_and_say(&input);
    }

    let length = input.len();
    println!("The length of the new result is {}.", length);
}

fn run_2015_11() {
    println!("Advent of Code 2015 - Day 11");
    let new_password = next_password("cqjxjnds");
    println!("His next password should be {}.", new_password);

    let new_password = next_password("cqjxxyzz");
    println!("The next one is {}.", new_password);
}

fn run_2015_12() {
    println!("Advent of Code 2015 - Day 12");
    let contents =
        fs::read_to_string("input/2015/day-12.txt").expect("Failed to read file to string.");

    let sum = sum_numbers_in_str(&contents);
    println!("The sum of all the numbers in the document is {}.", sum);

    let value = serde_json::from_str(&contents).unwrap();
    let sum = sum_value(&value);
    println!("The sum of the numbers without red objects is {}.", sum);
}

fn run_2015_13() {
    println!("Advent of Code 2015 - Day 13");
    let contents =
        std::fs::read_to_string("input/2015/day-13.txt").expect("Failed to read file to string.");

    let mut plan = SeatingPlan::new();
    contents
        .lines()
        .map(|line| SeatingPreference::from_str(line).unwrap())
        .for_each(|preference| plan.add_preference(preference));

    let max = plan.happiest_table();
    println!(
        "The total change in happiness for the optimal seating arrangement is {}.",
        max
    );

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
    println!("The total change in happiness for the optimal seating arrangement that actually includes yourself is {}", max);
}

fn run_2015_14() {
    println!("Advent of Code 2015 - Day 14");
    let contents =
        std::fs::read_to_string("input/2015/day-14.txt").expect("Failed to read file to string.");

    let reindeer: Vec<Reindeer> = contents
        .lines()
        .map(|line| Reindeer::from_str(line).unwrap())
        .collect();

    let max = distance_winning_reindeer_traveled(&reindeer, 2503);
    println!("The winning reindeer has travelled {} km.", max);

    let max = points_awarded_winning_reindeer(&reindeer, 2503);
    println!("The winning reindeer has {} points.", max);
}

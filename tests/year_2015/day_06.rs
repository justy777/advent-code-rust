use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_06::{AdjustableBulb, LightGrid, LightInstruction, SimpleBulb};

#[test]
fn test_light_grid_follow_instruction_bad_input() {
    match LightInstruction::from_str("pancakes") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match LightInstruction::from_str("switch 0,0 through 999,0") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match LightInstruction::from_str("switch -1,0 through 999,0") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match LightInstruction::from_str("turn on 1000,1001 through 1000,1002") {
        Ok(_) => panic!(),
        Err(_) => (),
    };
}

#[test]
fn test_light_grid_apply_operation() {
    let mut grid = LightGrid::<SimpleBulb>::new();
    if let Ok(instruction) = LightInstruction::from_str("turn on 0,0 through 999,999") {
        grid.apply_operation(instruction);
    };
    assert_eq!(grid.total_brightness(), 1000000);

    if let Ok(instruction) = LightInstruction::from_str("turn off 499,499 through 500,500") {
        grid.apply_operation(instruction);
    };
    assert_eq!(grid.total_brightness(), 999996);

    let mut grid = LightGrid::<SimpleBulb>::new();
    if let Ok(instruction) = LightInstruction::from_str("toggle 0,0 through 999,0") {
        grid.apply_operation(instruction);
    };
    assert_eq!(grid.total_brightness(), 1000);
}

#[test]
fn test_light_grid_increase_brightness() {
    let mut grid = LightGrid::<AdjustableBulb>::new();
    if let Ok(instruction) = LightInstruction::from_str("turn on 0,0 through 0,0") {
        grid.apply_operation(instruction);
    };
    assert_eq!(grid.total_brightness(), 1);

    let mut grid = LightGrid::<AdjustableBulb>::new();
    if let Ok(instruction) = LightInstruction::from_str("toggle 0,0 through 999,999") {
        grid.apply_operation(instruction);
    };
    assert_eq!(grid.total_brightness(), 2000000);
}

#[test]
fn test_simple_bulbs_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<SimpleBulb>::new();
    contents
        .lines()
        .map(|s| LightInstruction::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|instruction| grid.apply_operation(instruction));
    let count = grid.total_brightness();
    assert_eq!(count, 543903);
}

#[test]
fn test_adjustable_bulbs_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<AdjustableBulb>::new();
    contents
        .lines()
        .map(|s| LightInstruction::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|instruction| grid.apply_operation(instruction));
    let brightness = grid.total_brightness();
    assert_eq!(brightness, 14687245);
}

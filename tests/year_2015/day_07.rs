use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_07::{Circuit, CircuitInstruction};

#[test]
fn test_small_circuit() {
    let contents =
        fs::read_to_string("input/2015/day-07-sample.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents
        .lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(Result::ok)
        .for_each(|instruction| circuit.add_instruction(instruction));
    circuit.resolve();

    assert_eq!(circuit.signal("d"), Some(72));
    assert_eq!(circuit.signal("e"), Some(507));
    assert_eq!(circuit.signal("f"), Some(492));
    assert_eq!(circuit.signal("g"), Some(114));
    assert_eq!(circuit.signal("h"), Some(65412));
    assert_eq!(circuit.signal("i"), Some(65079));
    assert_eq!(circuit.signal("x"), Some(123));
    assert_eq!(circuit.signal("y"), Some(456));
}

#[test]
fn test_circuit_resolve_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-07.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents
        .lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(Result::ok)
        .for_each(|instruction| circuit.add_instruction(instruction));
    circuit.resolve();

    let signal = circuit.signal("a");
    assert_eq!(signal, Some(16076));
}

#[test]
fn test_circuit_resolve_input_file_and_extra_instruction() {
    let contents =
        fs::read_to_string("input/2015/day-07.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents
        .lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(Result::ok)
        .for_each(|instruction| circuit.add_instruction(instruction));

    if let Ok(instruction) = CircuitInstruction::from_str("16076 -> b") {
        circuit.add_instruction(instruction);
    };
    circuit.resolve();

    let signal = circuit.signal("a");
    assert_eq!(signal, Some(2797));
}

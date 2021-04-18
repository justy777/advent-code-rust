use std::fs;
use std::str::FromStr;

use advent_of_code::year_2015::day_07::{Circuit, CircuitInstruction};

#[test]
fn test_small_circuit() {
    let contents =
        fs::read_to_string("input/2015/day-7-sample.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents.lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|instruction| circuit.add_instruction(instruction));
    circuit.resolve_circuit();

    assert_eq!(circuit.signal("d").unwrap(), &72);
    assert_eq!(circuit.signal("e").unwrap(), &507);
    assert_eq!(circuit.signal("f").unwrap(), &492);
    assert_eq!(circuit.signal("g").unwrap(), &114);
    assert_eq!(circuit.signal("h").unwrap(), &65412);
    assert_eq!(circuit.signal("i").unwrap(), &65079);
    assert_eq!(circuit.signal("x").unwrap(), &123);
    assert_eq!(circuit.signal("y").unwrap(), &456);
}

#[test]
fn test_circuit_resolve_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-7.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents.lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|instruction| circuit.add_instruction(instruction));
    circuit.resolve_circuit();

    let signal = circuit.signal("a").unwrap();
    assert_eq!(signal, &16076);
}

#[test]
fn test_circuit_resolve_input_file_and_extra_instruction() {
    let contents =
        fs::read_to_string("input/2015/day-7.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents.lines()
        .map(|s| CircuitInstruction::from_str(s))
        .filter_map(|result| result.ok())
        .for_each(|instruction| circuit.add_instruction(instruction));

    if let Ok(instruction) = CircuitInstruction::from_str("16076 -> b") {
        circuit.add_instruction(instruction);
    };
    circuit.resolve_circuit();

    let signal = circuit.signal("a").unwrap();
    assert_eq!(signal, &2797);
}

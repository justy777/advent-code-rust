use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
enum Endpoint {
    Source(i32),
    Wire(String),
}

impl From<&str> for Endpoint {
    fn from(s: &str) -> Endpoint {
        let test = s.parse::<i32>();
        match test {
            Ok(value) => Endpoint::Source(value),
            Err(_) => Endpoint::Wire(String::from(s)),
        }
    }
}

#[derive(Clone)]
enum Gate {
    NoOp(Endpoint),
    Not(Endpoint),
    And(Endpoint, Endpoint),
    Or(Endpoint, Endpoint),
    LeftShift(Endpoint, i32),
    RightShift(Endpoint, i32),
}

struct Circuit {
    parts: HashMap<String, Gate>,
    signals: HashMap<String, i32>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            parts: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    fn add_gate(&mut self, name: String, gate: Gate) {
        self.parts.insert(name, gate);
    }

    fn resolve_circuit(&mut self) {
        let parts = self.parts.clone();
        parts.keys().for_each(|key| {
            self.resolve_signal(key);
        });
    }

    fn reset_circuit(&mut self) {
        self.signals = HashMap::new();
    }

    fn resolve_signal(&mut self, name: &str) -> i32 {
        if self.signals.contains_key(name) {
            return *self.signals.get(name).unwrap();
        }
        let parts = self.parts.clone();
        let signal = match parts.get(name).unwrap() {
            Gate::And(v1, v2) => self.resolve_endpoint(v1) & self.resolve_endpoint(v2),
            Gate::Or(v1, v2) => self.resolve_endpoint(v1) | self.resolve_endpoint(v2),
            Gate::LeftShift(v1, v2) => self.resolve_endpoint(v1) << *v2,
            Gate::RightShift(v1, v2) => self.resolve_endpoint(v1) >> *v2,
            Gate::Not(v1) => !self.resolve_endpoint(v1),
            Gate::NoOp(v1) => self.resolve_endpoint(v1),
        };
        self.signals.insert(String::from(name), signal);
        signal
    }

    fn resolve_endpoint(&mut self, endpoint: &Endpoint) -> i32 {
        match endpoint {
            Endpoint::Source(value) => *value,
            Endpoint::Wire(name) => self.resolve_signal(&name),
        }
    }

    fn follow_instruction(&mut self, instruction: &str) {
        if instruction.contains("AND") {
            let mut iter = instruction.split(' ');
            let operand1 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let operand2 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::And(operand1, operand2));
        } else if instruction.contains("OR") {
            let mut iter = instruction.split(' ');
            let operand1 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let operand2 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::Or(operand1, operand2));
        } else if instruction.contains("LSHIFT") {
            let mut iter = instruction.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let shift = iter.next().unwrap().parse::<i32>().unwrap();
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::LeftShift(operand, shift))
        } else if instruction.contains("RSHIFT") {
            let mut iter = instruction.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let shift = iter.next().unwrap().parse::<i32>().unwrap();
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::RightShift(operand, shift))
        } else if instruction.contains("NOT") {
            let mut iter = instruction.split(' ');
            iter.next();
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::Not(operand))
        } else {
            let mut iter = instruction.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::NoOp(operand))
        }
    }
}

#[test]
fn test_2015_day_7() {
    println!("Advent of Code 2015 - Day 7");
    let contents =
        fs::read_to_string("input/2015/day-7.txt").expect("Failed to read file to string.");

    let mut circuit = Circuit::new();
    contents
        .lines()
        .for_each(|line| circuit.follow_instruction(line));
    circuit.resolve_circuit();

    let signal_a = circuit.signals.get("a").unwrap();
    println!("The signal {} is provided to wire 'a'.", signal_a);
    assert_eq!(signal_a, &16076);

    circuit.reset_circuit();
    circuit
        .parts
        .insert(String::from("b"), Gate::NoOp(Endpoint::Source(16076)));
    circuit.resolve_circuit();

    let signal_a = circuit.signals.get("a").unwrap();
    println!("The signal {} is provided to wire 'a'.", signal_a);
    assert_eq!(signal_a, &2797);
}

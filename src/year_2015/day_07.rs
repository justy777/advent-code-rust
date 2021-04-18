use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
enum Endpoint {
    Source(u16),
    Wire(String),
}

impl From<&str> for Endpoint {
    fn from(s: &str) -> Endpoint {
        let test = s.parse::<u16>();
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
    LeftShift(Endpoint, u16),
    RightShift(Endpoint, u16),
}

pub struct Circuit {
    wires: HashMap<String, Gate>,
    signals: HashMap<String, u16>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            wires: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    pub fn signal(&self, wire: &str) -> Option<&u16> {
        self.signals.get(wire)
    }

    pub fn add_instruction(&mut self, instruction: CircuitInstruction) {
        self.wires.insert(instruction.wire, instruction.output);
    }

    pub fn resolve_circuit(&mut self) {
        let wires = self.wires.clone();
        wires.keys().for_each(|key| {
            self.resolve_signal(key);
        });
    }

    pub fn reset_circuit(&mut self) {
        self.signals = HashMap::new();
    }

    fn resolve_signal(&mut self, name: &str) -> u16 {
        if self.signals.contains_key(name) {
            return *self.signals.get(name).unwrap();
        }
        let wires = self.wires.clone();
        let signal = match wires.get(name).unwrap() {
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

    fn resolve_endpoint(&mut self, endpoint: &Endpoint) -> u16 {
        match endpoint {
            Endpoint::Source(value) => *value,
            Endpoint::Wire(name) => self.resolve_signal(&name),
        }
    }
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}


pub struct CircuitInstruction {
    wire: String,
    output: Gate,
}

impl FromStr for CircuitInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("AND") {
            let mut iter = s.split(' ');
            let operand1 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let operand2 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction {wire, output: Gate::And(operand1, operand2) })
        } else if s.contains("OR") {
            let mut iter = s.split(' ');
            let operand1 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let operand2 = Endpoint::from(iter.next().unwrap());
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction { wire, output: Gate::Or(operand1, operand2) })
        } else if s.contains("LSHIFT") {
            let mut iter = s.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let shift = iter.next().unwrap().parse::<u16>().unwrap();
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction { wire, output: Gate::LeftShift(operand, shift) })
        } else if s.contains("RSHIFT") {
            let mut iter = s.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let shift = iter.next().unwrap().parse::<u16>().unwrap();
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction { wire, output: Gate::RightShift(operand, shift) })
        } else if s.contains("NOT") {
            let mut iter = s.split(' ');
            iter.next();
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction { wire, output: Gate::Not(operand) })
        } else {
            let mut iter = s.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let wire = String::from(iter.next().unwrap());
            Ok( CircuitInstruction { wire, output: Gate::NoOp(operand) })
        }
    }
}
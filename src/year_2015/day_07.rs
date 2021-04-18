use std::str::FromStr;

use hashbrown::HashMap;
use regex::Regex;

use crate::util::CapturesWrapper;

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
    LeftShift(Endpoint, Endpoint),
    RightShift(Endpoint, Endpoint),
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
            Gate::LeftShift(v1, v2) => self.resolve_endpoint(v1) << self.resolve_endpoint(v2),
            Gate::RightShift(v1, v2) => self.resolve_endpoint(v1) >> self.resolve_endpoint(v2),
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

    fn from_str(s: &str) -> Result<CircuitInstruction, ()> {
        lazy_static! {
            static ref BINARY: Regex = Regex::new(r"^(?P<lhs>\w+|\d+) (?P<operation>AND|OR|LSHIFT|RSHIFT) (?P<rhs>\w+|\d+) -> (?P<wire>\w+)$").unwrap();
        }
        if let Some(caps) = BINARY.captures(s) {
            let caps = CapturesWrapper::new(caps);
            let lhs = Endpoint::from(caps.as_str("lhs"));
            let rhs = Endpoint::from(caps.as_str("rhs"));
            let wire = caps.as_string("wire");
            let output = match caps.as_str("operation") {
                "AND" => Gate::And(lhs, rhs),
                "OR" => Gate::Or(lhs, rhs),
                "LSHIFT" => Gate::LeftShift(lhs, rhs),
                "RSHIFT" => Gate::RightShift(lhs, rhs),
                _ => unreachable!(),
            };

            return Ok(CircuitInstruction { wire, output });
        };

        lazy_static! {
            static ref UNARY: Regex =
                Regex::new(r"^(?P<operator>NOT)?\s?(?P<operand>\w+|\d+) -> (?P<wire>\w+)$")
                    .unwrap();
        }
        if let Some(caps) = UNARY.captures(s) {
            let caps = CapturesWrapper::new(caps);
            let wire = caps.as_string("wire");
            let operand = Endpoint::from(caps.as_str("operand"));
            let output = match caps.name("operator") {
                Some(_) => Gate::Not(operand),
                None => Gate::NoOp(operand),
            };

            return Ok(CircuitInstruction { wire, output });
        };

        Err(())
    }
}

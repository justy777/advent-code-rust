use std::cell::RefCell;
use std::str::FromStr;

use hashbrown::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::CapturesWrapper;

#[derive(Clone)]
enum Signal {
    Source(u16),
    Wire(String),
}

impl From<&str> for Signal {
    fn from(s: &str) -> Signal {
        let test = s.parse::<u16>();
        match test {
            Ok(value) => Signal::Source(value),
            Err(_) => Signal::Wire(String::from(s)),
        }
    }
}

#[derive(Clone)]
enum Gate {
    NoOp(Signal),
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LeftShift(Signal, Signal),
    RightShift(Signal, Signal),
}

pub struct Circuit {
    wires: HashMap<String, Gate>,
    signals: RefCell<HashMap<String, u16>>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            wires: HashMap::new(),
            signals: RefCell::new(HashMap::new()),
        }
    }

    pub fn signal(&self, wire: &str) -> Option<u16> {
        self.signals.borrow().get(wire).copied()
    }

    pub fn add_instruction(&mut self, instruction: CircuitInstruction) {
        self.wires.insert(instruction.wire, instruction.output);
    }

    pub fn resolve(&mut self) {
        self.wires.keys().for_each(|key| {
            self.resolve_wire(key);
        });
    }

    pub fn reset(&mut self) {
        self.signals.borrow_mut().clear();
    }

    fn resolve_wire(&self, name: &str) -> u16 {
        if self.signals.borrow().contains_key(name) {
            return *self.signals.borrow().get(name).unwrap();
        }
        let signal = match self.wires.get(name).unwrap() {
            Gate::And(v1, v2) => self.resolve_signal(v1) & self.resolve_signal(v2),
            Gate::Or(v1, v2) => self.resolve_signal(v1) | self.resolve_signal(v2),
            Gate::LeftShift(v1, v2) => self.resolve_signal(v1) << self.resolve_signal(v2),
            Gate::RightShift(v1, v2) => self.resolve_signal(v1) >> self.resolve_signal(v2),
            Gate::Not(v1) => !self.resolve_signal(v1),
            Gate::NoOp(v1) => self.resolve_signal(v1),
        };
        self.signals.borrow_mut().insert(String::from(name), signal);
        signal
    }

    fn resolve_signal(&self, signal: &Signal) -> u16 {
        match signal {
            Signal::Source(value) => *value,
            Signal::Wire(name) => self.resolve_wire(name),
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
        static BINARY: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<lhs>\w+|\d+) (?P<operation>AND|OR|LSHIFT|RSHIFT) (?P<rhs>\w+|\d+) -> (?P<wire>\w+)$").unwrap()
        });

        if let Some(caps) = BINARY.captures(s) {
            let caps = CapturesWrapper::new(caps);
            let lhs = Signal::from(caps.as_str("lhs"));
            let rhs = Signal::from(caps.as_str("rhs"));
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

        static UNARY: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<operator>NOT)?\s?(?P<operand>\w+|\d+) -> (?P<wire>\w+)$").unwrap()
        });

        if let Some(caps) = UNARY.captures(s) {
            let caps = CapturesWrapper::new(caps);
            let wire = caps.as_string("wire");
            let operand = Signal::from(caps.as_str("operand"));
            let output = match caps.name("operator") {
                Some(_) => Gate::Not(operand),
                None => Gate::NoOp(operand),
            };

            return Ok(CircuitInstruction { wire, output });
        };

        Err(())
    }
}

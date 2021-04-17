use std::collections::HashMap;

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
    parts: HashMap<String, Gate>,
    signals: HashMap<String, u16>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            parts: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    pub fn signal(&self, wire: &str) -> Option<&u16> {
        self.signals.get(wire)
    }

    fn add_gate(&mut self, name: String, gate: Gate) {
        self.parts.insert(name, gate);
    }

    pub fn resolve_circuit(&mut self) {
        let parts = self.parts.clone();
        parts.keys().for_each(|key| {
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

    fn resolve_endpoint(&mut self, endpoint: &Endpoint) -> u16 {
        match endpoint {
            Endpoint::Source(value) => *value,
            Endpoint::Wire(name) => self.resolve_signal(&name),
        }
    }

    pub fn follow_instruction(&mut self, instruction: &str) {
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
            let shift = iter.next().unwrap().parse::<u16>().unwrap();
            iter.next();
            let output = String::from(iter.next().unwrap());
            self.add_gate(output, Gate::LeftShift(operand, shift))
        } else if instruction.contains("RSHIFT") {
            let mut iter = instruction.split(' ');
            let operand = Endpoint::from(iter.next().unwrap());
            iter.next();
            let shift = iter.next().unwrap().parse::<u16>().unwrap();
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

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}

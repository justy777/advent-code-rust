use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use regex::Regex;
use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

pub trait State: Clone + Default {
    fn toggle(&mut self);

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn brightness(&self) -> i32;
}

#[derive(Clone, Default)]
pub struct SimpleBulb {
    state: bool,
}

impl State for SimpleBulb {
    fn toggle(&mut self) {
        self.state = !self.state;
    }

    fn turn_on(&mut self) {
        self.state = true;
    }

    fn turn_off(&mut self) {
        self.state = false;
    }

    fn brightness(&self) -> i32 {
        if self.state {
            1
        } else {
            0
        }
    }
}

#[derive(Clone, Default)]
pub struct DimmableBulb {
    state: i32,
}

impl State for DimmableBulb {
    fn toggle(&mut self) {
        self.state += 2;
    }

    fn turn_on(&mut self) {
        self.state += 1;
    }

    fn turn_off(&mut self) {
        if self.state != 0 {
            self.state -= 1;
        }
    }

    fn brightness(&self) -> i32 {
        self.state
    }
}

pub struct LightGrid<T: State> {
    lights: Vec<Vec<T>>,
}

const GRID_CAPACITY: usize = 1000;

impl<T: State> LightGrid<T> {
    pub fn new() -> LightGrid<T> {
        LightGrid {
            lights: vec![vec![T::default(); GRID_CAPACITY]; GRID_CAPACITY],
        }
    }

    pub fn total_brightness(&self) -> i32 {
        let mut count = 0;
        for x in 0..GRID_CAPACITY {
            for y in 0..GRID_CAPACITY {
                count += self.lights[x][y].brightness();
            }
        }
        count
    }

    pub fn apply_operation(&mut self, instruction: LightInstruction) {
        for x in instruction.start_point.x..=instruction.end_point.x {
            for y in instruction.start_point.y..=instruction.end_point.y {
                match instruction.operation {
                    Operation::Toggle => self.lights[x][y].toggle(),
                    Operation::TurnOn => self.lights[x][y].turn_on(),
                    Operation::TurnOff => self.lights[x][y].turn_off(),
                }
            }
        }
    }
}

impl<T: State> Default for LightGrid<T> {
    fn default() -> Self {
        LightGrid::new()
    }
}

pub struct LightInstruction {
    operation: Operation,
    start_point: Point,
    end_point: Point,
}

impl FromStr for LightInstruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<LightInstruction, ParseInstructionError> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<operation>toggle|turn on|turn off) (?P<x1>\d{1,3}),(?P<y1>\d{1,3}) through (?P<x2>\d{1,3}),(?P<y2>\d{1,3})$").unwrap();
        }

        match REGEX.captures(s) {
            Some(caps) => {
                let operation = match caps.name("operation").unwrap().as_str() {
                    "toggle" => Operation::Toggle,
                    "turn on" => Operation::TurnOn,
                    "turn off" => Operation::TurnOff,
                    _ => unimplemented!(),
                };

                let parse_int = |key| caps.name(key).unwrap().as_str().parse::<usize>().unwrap();
                let x1 = parse_int("x1");
                let y1 = parse_int("y1");
                let x2 = parse_int("x2");
                let y2 = parse_int("y2");

                let start_point = Point { x: x1, y: y1 };
                let end_point = Point { x: x2, y: y2 };

                Ok(LightInstruction {
                    operation,
                    start_point,
                    end_point,
                })
            }
            None => Err(ParseInstructionError { _priv: () }),
        }
    }
}

#[derive(Debug)]
pub struct ParseInstructionError {
    pub(super) _priv: (),
}

impl Error for ParseInstructionError {}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not in format '(toggle|turn on|turn off) (0-999),(0-999) through (0-999),(0-999)'".fmt(f)
    }
}

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
fn test_light_grid_follow_instruction() {
    let mut grid = LightGrid::<SimpleBulb>::new();
    let instruction = LightInstruction::from_str("turn on 0,0 through 999,999").unwrap();
    grid.apply_operation(instruction);
    assert_eq!(grid.total_brightness(), 1000000);

    let instruction = LightInstruction::from_str("turn off 499,499 through 500,500").unwrap();
    grid.apply_operation(instruction);
    assert_eq!(grid.total_brightness(), 999996);

    let mut grid = LightGrid::<SimpleBulb>::new();
    let instruction = LightInstruction::from_str("toggle 0,0 through 999,0").unwrap();
    grid.apply_operation(instruction);
    assert_eq!(grid.total_brightness(), 1000);
}

#[test]
fn test_light_grid_increase_brightness() {
    let mut grid = LightGrid::<DimmableBulb>::new();
    let instruction = LightInstruction::from_str("turn on 0,0 through 0,0").unwrap();
    grid.apply_operation(instruction);
    assert_eq!(grid.total_brightness(), 1);

    let mut grid = LightGrid::<DimmableBulb>::new();
    let instruction = LightInstruction::from_str("toggle 0,0 through 999,999").unwrap();
    grid.apply_operation(instruction);
    assert_eq!(grid.total_brightness(), 2000000);
}

#[test]
fn test_simple_bulbs_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<SimpleBulb>::new();
    contents
        .lines()
        .map(|line| LightInstruction::from_str(line).unwrap())
        .for_each(|instruction| grid.apply_operation(instruction));
    let lit_lights_count = grid.total_brightness();
    assert_eq!(lit_lights_count, 543903);
}

#[test]
fn test_dimmable_bulbs_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<DimmableBulb>::new();
    contents
        .lines()
        .map(|line| LightInstruction::from_str(line).unwrap())
        .for_each(|instruction| grid.apply_operation(instruction));
    let total_brightness = grid.total_brightness();
    assert_eq!(total_brightness, 14687245);
}

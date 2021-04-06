use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{fmt, fs};

use regex::Regex;

struct Point {
    x: usize,
    y: usize,
}

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

trait State: Clone + Default {
    fn toggle(&mut self);

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn brightness(&self) -> i32;
}

#[derive(Clone, Default)]
struct SimpleBulb {
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
struct DimmableBulb {
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

struct LightGrid<T: State> {
    lights: Vec<Vec<T>>,
}

const GRID_CAPACITY: usize = 1000;

impl<T: State> LightGrid<T> {
    fn new() -> LightGrid<T> {
        LightGrid {
            lights: vec![vec![T::default(); GRID_CAPACITY]; GRID_CAPACITY],
        }
    }

    fn total_brightness(&self) -> i32 {
        let mut count = 0;
        for x in 0..GRID_CAPACITY {
            for y in 0..GRID_CAPACITY {
                count += self.lights[x][y].brightness();
            }
        }
        count
    }

    fn apply_operation(&mut self, operation: Operation, start_point: Point, end_point: Point) {
        for x in start_point.x..=end_point.x {
            for y in start_point.y..=end_point.y {
                match operation {
                    Operation::Toggle => self.lights[x][y].toggle(),
                    Operation::TurnOn => self.lights[x][y].turn_on(),
                    Operation::TurnOff => self.lights[x][y].turn_off(),
                }
            }
        }
    }

    fn follow_instruction(&mut self, instruction: &str) -> Result<(), ParseInstructionError> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<operation>toggle|turn on|turn off) (?P<x1>\d{1,3}),(?P<y1>\d{1,3}) through (?P<x2>\d{1,3}),(?P<y2>\d{1,3})$").unwrap();
        }

        match REGEX.captures(instruction) {
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

                let point1 = Point { x: x1, y: y1 };
                let point2 = Point { x: x2, y: y2 };

                self.apply_operation(operation, point1, point2);
                Ok(())
            }
            None => Err(ParseInstructionError { _priv: () }),
        }
    }
}

#[test]
fn test_light_grid_follow_instruction_bad_input() {
    let mut grid = LightGrid::<SimpleBulb>::new();
    match grid.follow_instruction("pancakes") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match grid.follow_instruction("switch 0,0 through 999,0") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match grid.follow_instruction("switch -1,0 through 999,0") {
        Ok(_) => panic!(),
        Err(_) => (),
    };

    match grid.follow_instruction("turn on 1000,1001 through 1000,1002") {
        Ok(_) => panic!(),
        Err(_) => (),
    };
}

#[derive(Debug)]
struct ParseInstructionError {
    pub(super) _priv: (),
}

impl Error for ParseInstructionError {}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not in format '(toggle|turn on|turn off) (0-999),(0-999) through (0-999),(0-999)'".fmt(f)
    }
}

#[test]
fn test_light_grid_follow_instruction() {
    let mut grid = LightGrid::<SimpleBulb>::new();
    grid.follow_instruction("turn on 0,0 through 999,999")
        .unwrap();
    assert_eq!(grid.total_brightness(), 1000000);

    grid.follow_instruction("turn off 499,499 through 500,500")
        .unwrap();
    assert_eq!(grid.total_brightness(), 999996);

    let mut grid = LightGrid::<SimpleBulb>::new();
    grid.follow_instruction("toggle 0,0 through 999,0").unwrap();
    assert_eq!(grid.total_brightness(), 1000);
}

#[test]
fn test_light_grid_increase_brightness() {
    let mut grid = LightGrid::<DimmableBulb>::new();
    grid.follow_instruction("turn on 0,0 through 0,0").unwrap();
    assert_eq!(grid.total_brightness(), 1);

    let mut grid = LightGrid::<DimmableBulb>::new();
    grid.follow_instruction("toggle 0,0 through 999,999")
        .unwrap();
    assert_eq!(grid.total_brightness(), 2000000);
}

#[test]
fn test_2015_day_6() {
    println!("Advent of Code 2015 - Day 6");
    let contents =
        fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::<SimpleBulb>::new();
    contents
        .lines()
        .for_each(|line| grid.follow_instruction(line).unwrap());
    let lit_lights_count = grid.total_brightness();
    println!("There are {} lights lit.", lit_lights_count);
    assert_eq!(lit_lights_count, 543903);

    let mut grid = LightGrid::<DimmableBulb>::new();
    contents
        .lines()
        .for_each(|line| grid.follow_instruction(line).unwrap());
    let total_brightness = grid.total_brightness();
    println!("The total brightness is {}.", total_brightness);
    assert_eq!(total_brightness, 14687245);
}

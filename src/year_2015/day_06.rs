/*!
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.
*/

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::CapturesWrapper;

/// Represents a point or bulb in the grid.
#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Represents the operations of a light bulb.
#[derive(Debug)]
pub enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

/// Bulb describes types that can be used as a light bulb.
pub trait Bulb: Clone + Default {
    fn toggle(&mut self);

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn brightness(&self) -> i32;
}

/// Represents a bulb that is either on or off.
#[derive(Clone, Debug, Default)]
pub struct SimpleBulb {
    state: bool,
}

impl Bulb for SimpleBulb {
    /// Change the light to the opposite state.
    fn toggle(&mut self) {
        self.state = !self.state;
    }

    /// Set the light as off.
    fn turn_on(&mut self) {
        self.state = true;
    }

    /// Set the light as on.
    fn turn_off(&mut self) {
        self.state = false;
    }

    /// Returns `1` if on and `0` if off.
    fn brightness(&self) -> i32 {
        if self.state {
            1
        } else {
            0
        }
    }
}

/// Represents a bulb with brightness of zero or more.
#[derive(Clone, Debug, Default)]
pub struct AdjustableBulb {
    state: i32,
}

impl Bulb for AdjustableBulb {
    /// Turns up the brightness by two.
    fn toggle(&mut self) {
        self.state += 2;
    }

    /// turns up brightness by one.
    fn turn_on(&mut self) {
        self.state += 1;
    }

    /// Turns brightness down by one to a minimum of zero.
    fn turn_off(&mut self) {
        if self.state != 0 {
            self.state -= 1;
        }
    }

    /// Returns the light's brightness.
    fn brightness(&self) -> i32 {
        self.state
    }
}

/// Represents a finite grid of lights.
pub struct LightGrid<T: Bulb> {
    lights: Vec<Vec<T>>,
}

const GRID_CAPACITY: usize = 1000;

impl<T: Bulb> LightGrid<T> {
    /// Constructs a new full `LightGrid<T>`.
    ///
    /// The grid allocates elements right away.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::year_2015::day_06::{LightGrid, SimpleBulb};
    ///
    /// let mut grid: LightGrid<SimpleBulb> = LightGrid::new();
    /// ```
    pub fn new() -> LightGrid<T> {
        LightGrid {
            lights: vec![vec![T::default(); GRID_CAPACITY]; GRID_CAPACITY],
        }
    }

    /// Returns the total brightness from all lights in the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::year_2015::day_06::{LightGrid, SimpleBulb, LightInstruction, Operation, Point};
    ///
    /// let mut grid: LightGrid<SimpleBulb> = LightGrid::new();
    /// let instruction = LightInstruction {
    ///     operation: Operation::Toggle,
    ///     start_point: Point { x: 0, y: 0 },
    ///     end_point: Point { x: 0, y: 999 }
    /// };
    /// grid.apply_operation(instruction);
    /// let count = grid.total_brightness();
    /// assert_eq!(count, 1000);
    /// ```
    pub fn total_brightness(&self) -> i32 {
        let mut count = 0;
        for x in 0..GRID_CAPACITY {
            for y in 0..GRID_CAPACITY {
                count += self.lights[x][y].brightness();
            }
        }
        count
    }

    // TODO: Update with happy path doc tests
    /// Applies the provided instruction to the lights in the grid.
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

impl<T: Bulb> Default for LightGrid<T> {
    fn default() -> Self {
        LightGrid::new()
    }
}

/// Represents an instruction for all lights between the provided start and end points.
#[derive(Debug)]
pub struct LightInstruction {
    pub operation: Operation,
    pub start_point: Point,
    pub end_point: Point,
}

// TODO: Add comments on proper string format
impl FromStr for LightInstruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<LightInstruction, ParseInstructionError> {
        static REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<operation>toggle|turn on|turn off) (?P<x1>\d{1,3}),(?P<y1>\d{1,3}) through (?P<x2>\d{1,3}),(?P<y2>\d{1,3})$").unwrap()
        });

        match REGEX.captures(s) {
            Some(caps) => {
                let caps = CapturesWrapper::new(caps);
                let operation = match caps.as_str("operation") {
                    "toggle" => Operation::Toggle,
                    "turn on" => Operation::TurnOn,
                    "turn off" => Operation::TurnOff,
                    _ => unimplemented!(),
                };

                let x1 = caps.parse("x1");
                let y1 = caps.parse("y1");
                let x2 = caps.parse("x2");
                let y2 = caps.parse("y2");

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

/// Error type used when parsing a light instruction from a `str`.
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

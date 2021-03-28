use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Point, ParseIntError> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse::<usize>()?;
        let y = s.next().unwrap().parse::<usize>()?;

        Ok(Point { x, y })
    }
}

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

trait State {
    fn toggle(&mut self);

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn brightness(&self) -> i32;
}

#[derive(Clone)]
struct SimpleBulb {
    state: bool,
}

impl SimpleBulb {
    fn new() -> SimpleBulb {
        SimpleBulb { state: false }
    }
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

#[derive(Clone)]
struct DimmableBulb {
    state: i32,
}

impl DimmableBulb {
    fn new() -> DimmableBulb {
        DimmableBulb { state: 0 }
    }
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

impl<T: State + Clone> LightGrid<T> {
    fn new(bulb: T) -> LightGrid<T> {
        LightGrid {
            lights: vec![vec![bulb; GRID_CAPACITY]; GRID_CAPACITY],
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

    fn follow_instruction(&mut self, instruction: &str) {
        let mut operation = None;
        let mut s = instruction.split(' ');
        let mut word = s.next().unwrap();
        if word == "toggle" {
            operation = Some(Operation::Toggle);
        } else if word == "turn" {
            word = s.next().unwrap();
            if word == "on" {
                operation = Some(Operation::TurnOn);
            } else if word == "off" {
                operation = Some(Operation::TurnOff);
            }
        }

        let point1 = Point::from_str(s.next().unwrap()).unwrap();
        s.next();
        let point2 = Point::from_str(s.next().unwrap()).unwrap();
        self.apply_operation(operation.unwrap(), point1, point2);
    }
}

#[test]
fn test_2015_day_6() {
    println!("Advent of Code 2015 - Day 6");
    let contents =
        fs::read_to_string("input/2015/day-6.txt").expect("Failed to read file to string.");

    let mut grid = LightGrid::new(SimpleBulb::new());
    contents
        .lines()
        .for_each(|line| grid.follow_instruction(line));
    let lit_lights_count = grid.total_brightness();
    println!("There are {} lights lit.", lit_lights_count);
    assert_eq!(lit_lights_count, 543903);

    let mut grid = LightGrid::new(DimmableBulb::new());
    contents
        .lines()
        .for_each(|line| grid.follow_instruction(line));
    let total_brightness = grid.total_brightness();
    println!("The total brightness is {}.", total_brightness);
    assert_eq!(total_brightness, 14687245);
}

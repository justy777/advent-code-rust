use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse::<usize>()?;
        let y = s.next().unwrap().parse::<usize>()?;

        Ok(Point {x, y})
    }
}

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}


#[derive(Copy, Clone)]
struct Bulb {
    state: bool,
}

impl Bulb {
    fn new() -> Bulb {
        Bulb {
            state: false,
        }
    }

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
        if self.state {1} else {0}
    }
}

struct LightGrid {
    lights: [[Bulb; 1000]; 1000],
}

impl LightGrid {
    fn new() -> LightGrid {
        LightGrid {
            lights: [[Bulb::new(); 1000]; 1000],
        }
    }

    fn count_lit_lights(&self) -> i32 {
        let mut count = 0;
        for x in 0..1000 {
            for y in 0..1000 {
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
                    Operation::TurnOff=> self.lights[x][y].turn_off(),
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
    let contents = fs::read_to_string("input/2015/day-6.txt")
        .expect("Failed to read file to String");

    let mut grid = LightGrid::new();
    contents.lines().for_each(|line| grid.follow_instruction(line));
    let lit_lights_count = grid.count_lit_lights();
    println!("There are {} lights lit.", lit_lights_count);
    assert_eq!(lit_lights_count, 543903);
}
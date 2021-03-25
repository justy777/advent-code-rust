use std::fs;

struct House {
    floor_number: i32,
    move_count: i32,
}

impl House {

    fn new() -> House {
        House {
            floor_number: 0,
            move_count: 0,
        }
    }

    fn move_floor(&mut self, direction: char) {
        match direction {
            '(' => self.floor_number += 1,
            ')' => self.floor_number -= 1,
            _ => (),
        };
        self.move_count += 1;
    }

    fn calculate_destination_floor(&mut self, directions: &str) -> i32 {
        directions.chars().for_each(|c| self.move_floor(c));
        self.floor_number
    }

    fn calculate_moves_to_floor(&mut self, directions: &str, floor: i32) -> i32 {
        for c in directions.chars() {
            self.move_floor(c);
            if self.floor_number == floor {
                break;
            }
        }
        self.move_count
    }
}

#[test]
fn test_2015_day_1() {
    println!("Advent of Code 2015 - Day 1");
    let contents = fs::read_to_string("input/2015/day-1.txt")
        .expect("Failed to read file to String");

    let mut house = House::new();
    let destination_floor = house.calculate_destination_floor(&contents);
    assert_eq!(destination_floor, 280);
    println!("The instructions take Santa to floor {}.", destination_floor);

    house = House::new();
    let first_basement_position = house.calculate_moves_to_floor(&contents, -1);

    assert_eq!(first_basement_position, 1797);
    println!("The first position on floor -1 is {}.", first_basement_position);
}
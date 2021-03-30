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
}

fn calculate_destination_floor(directions: &str) -> i32 {
    let mut house = House::new();
    directions.chars().for_each(|c| house.move_floor(c));
    house.floor_number
}

fn calculate_moves_to_floor(directions: &str, floor: i32) -> i32 {
    let mut house = House::new();
    for c in directions.chars() {
        house.move_floor(c);
        if house.floor_number == floor {
            break;
        }
    }
    house.move_count
}

#[test]
fn test_2015_day_1() {
    println!("Advent of Code 2015 - Day 1");
    let contents =
        fs::read_to_string("input/2015/day-1.txt").expect("Failed to read file to string.");

    let destination_floor = calculate_destination_floor(&contents);
    assert_eq!(destination_floor, 280);
    println!(
        "The instructions take Santa to floor {}.",
        destination_floor
    );

    let first_basement_position = calculate_moves_to_floor(&contents, -1);

    assert_eq!(first_basement_position, 1797);
    println!(
        "The first position on floor -1 is {}.",
        first_basement_position
    );
}

use std::fs;

struct Counter {
    count: i32,
    operations: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
            operations: 0,
        }
    }

    fn apply(&mut self, operation: char) {
        match operation {
            '(' => self.count += 1,
            ')' => self.count -= 1,
            _ => return,
        };
        self.operations += 1;
    }
}

pub fn calculate_final_count(operations: &str) -> i32 {
    let mut counter = Counter::new();
    operations.chars().for_each(|c| counter.apply(c));
    counter.count
}

pub fn calculate_operations_to_value(operations: &str, stop_value: i32) -> i32 {
    let mut counter = Counter::new();
    for c in operations.chars() {
        counter.apply(c);
        if counter.count == stop_value {
            break;
        }
    }
    counter.operations
}

#[test]
fn test_calculate_final_count() {
    let value = calculate_final_count("(())");
    assert_eq!(value, 0);

    let value = calculate_final_count("()()");
    assert_eq!(value, 0);

    let value = calculate_final_count("(((");
    assert_eq!(value, 3);

    let value = calculate_final_count("(()(()(");
    assert_eq!(value, 3);

    let value = calculate_final_count("))(((((");
    assert_eq!(value, 3);

    let value = calculate_final_count("())");
    assert_eq!(value, -1);

    let value = calculate_final_count("))(");
    assert_eq!(value, -1);

    let value = calculate_final_count(")))");
    assert_eq!(value, -3);

    let value = calculate_final_count(")())())");
    assert_eq!(value, -3);
}

#[test]
fn test_calculate_operations_to_value() {
    let moves = calculate_operations_to_value(")", -1);
    assert_eq!(moves, 1);

    let moves = calculate_operations_to_value("()())", -1);
    assert_eq!(moves, 5);
}

#[test]
fn test_apply_bad_input() {
    let mut counter = Counter::new();
    counter.apply('f');
    assert_eq!(counter.count, 0);
    assert_eq!(counter.operations, 0);
}

#[test]
fn test_2015_day_1() {
    println!("Advent of Code 2015 - Day 1");
    let contents =
        fs::read_to_string("input/2015/day-1.txt").expect("Failed to read file to string.");

    let destination_floor = calculate_final_count(&contents);
    println!(
        "The instructions take Santa to floor {}.",
        destination_floor
    );
    assert_eq!(destination_floor, 280);

    let first_basement_position = calculate_operations_to_value(&contents, -1);
    println!(
        "The first position on floor -1 is {}.",
        first_basement_position
    );
    assert_eq!(first_basement_position, 1797);
}

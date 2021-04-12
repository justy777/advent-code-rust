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

    fn apply(self, operation: &u8) -> Counter {
        match operation {
            b'(' => Counter {
                count: self.count + 1,
                operations: self.operations + 1,
            },
            b')' => Counter {
                count: self.count - 1,
                operations: self.operations + 1,
            },
            _ => self,
        }
    }
}

pub fn calculate_final_count(operations: &[u8]) -> i32 {
    let counter = operations
        .iter()
        .fold(Counter::new(), |counter, c| counter.apply(c));
    counter.count
}

pub fn calculate_operations_to_value(operations: &[u8], stop_value: i32) -> i32 {
    let mut counter = Counter::new();
    for c in operations.iter() {
        counter = counter.apply(c);
        if counter.count == stop_value {
            break;
        }
    }
    counter.operations
}

#[test]
fn test_calculate_final_count() {
    let value = calculate_final_count(b"(())");
    assert_eq!(value, 0);

    let value = calculate_final_count(b"()()");
    assert_eq!(value, 0);

    let value = calculate_final_count(b"(((");
    assert_eq!(value, 3);

    let value = calculate_final_count(b"(()(()(");
    assert_eq!(value, 3);

    let value = calculate_final_count(b"))(((((");
    assert_eq!(value, 3);

    let value = calculate_final_count(b"())");
    assert_eq!(value, -1);

    let value = calculate_final_count(b"))(");
    assert_eq!(value, -1);

    let value = calculate_final_count(b")))");
    assert_eq!(value, -3);

    let value = calculate_final_count(b")())())");
    assert_eq!(value, -3);
}

#[test]
fn test_calculate_operations_to_value() {
    let moves = calculate_operations_to_value(b")", -1);
    assert_eq!(moves, 1);

    let moves = calculate_operations_to_value(b"()())", -1);
    assert_eq!(moves, 5);
}

#[test]
fn test_apply_bad_input() {
    let mut counter = Counter::new();
    counter = counter.apply(&b'f');
    assert_eq!(counter.count, 0);
    assert_eq!(counter.operations, 0);
}

#[test]
fn test_calculate_final_count_input_file() {
    let contents =
        std::fs::read("input/2015/day-1.txt").expect("Failed to read file to string.");

    let destination_floor = calculate_final_count(&contents);
    assert_eq!(destination_floor, 280);
}

#[test]
fn test_test_calculate_operations_to_value_input_file() {
    let contents =
        std::fs::read("input/2015/day-1.txt").expect("Failed to read file to string.");

    let first_basement_position = calculate_operations_to_value(&contents, -1);
    assert_eq!(first_basement_position, 1797);
}

use std::fs;

fn sum_floors(contents: &str) -> i32 {
    contents.chars().fold(0, |mut count, char| {
        match char {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }
        count
    })
}

fn floor_position(contents: &str, floor: i32) -> Option<usize> {
    contents.chars()
        .scan(0, |count, char| {
            match char {
                '(' => *count += 1,
                ')' => *count -= 1,
                _ => (),
            }
            Some(*count)
        })
        .position(|x| x == floor)
        .and_then(|x| Some(x + 1))
}

#[test]
fn test_2015_day_1() {
    println!("Advent of Code 2015 - Day 1");
    let contents = fs::read_to_string("input/2015/day-1.txt")
        .expect("Failed to read file to String");

    let floor_sum = sum_floors(&contents);
    assert_eq!(floor_sum, 280);
    println!("The instructions take Santa to floor {}.", floor_sum);

    let first_basement_position = floor_position(&contents, -1)
        .expect("Never reaches basement");

    assert_eq!(first_basement_position, 1797);
    println!("The first position on floor -1 is {}.", first_basement_position);
}
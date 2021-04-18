use std::fs;

use advent_of_code::year_2015::day_03::InfiniteGrid;

#[test]
fn test_grid_single_position_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-03.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(1);

    contents.chars().for_each(|c| grid.move_position(c));

    let visited = grid.visited();
    assert_eq!(visited, 2081);
}

#[test]
fn test_grid_two_positions_input_file() {
    let contents =
        fs::read_to_string("input/2015/day-03.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(2);

    contents.chars().for_each(|c| grid.move_position(c));

    let visited = grid.visited();
    assert_eq!(visited, 2341);
}

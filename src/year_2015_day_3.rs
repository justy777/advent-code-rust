use std::collections::HashSet;
use std::fs;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn move_direction(&self, direction: char) -> Option<Position> {
        match direction {
            '^' => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            'v' => Some(Position {
                x: self.x,
                y: self.y - 1,
            }),
            '>' => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            '<' => Some(Position {
                x: self.x - 1,
                y: self.y,
            }),
            _ => None,
        }
    }
}

pub struct InfiniteGrid {
    current_positions: Vec<Position>,
    past_positions: HashSet<Position>,
    turn: usize,
}

impl InfiniteGrid {
    pub fn new(parallel: usize) -> InfiniteGrid {
        let mut past_positions = HashSet::new();
        past_positions.insert(Position::new());
        InfiniteGrid {
            current_positions: vec![Position::new(); parallel],
            past_positions,
            turn: 0,
        }
    }

    pub fn move_position(&mut self, direction: char) {
        let position = self.current_positions[self.turn];
        if let Some(new_position) = position.move_direction(direction) {
            self.past_positions.insert(new_position);
            self.current_positions[self.turn] = new_position;

            self.turn = (self.turn + 1) % self.current_positions.len();
        }
    }

    pub fn past_positions_len(&self) -> usize {
        self.past_positions.len()
    }
}

#[test]
fn test_grid_move_position_bad_input() {
    let mut grid = InfiniteGrid::new(1);
    grid.move_position('f');
    assert_eq!(grid.current_positions[0], Position::new());
    assert_eq!(grid.turn, 0);
}

#[test]
fn test_grid_single_position() {
    let mut grid = InfiniteGrid::new(1);
    grid.move_position('>');
    assert_eq!(grid.past_positions_len(), 2);

    let mut grid = InfiniteGrid::new(1);
    "^>v<".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.past_positions_len(), 4);

    let mut grid = InfiniteGrid::new(1);
    "^v^v^v^v^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.past_positions_len(), 2);
}

#[test]
fn test_grid_two_positions() {
    let mut grid = InfiniteGrid::new(2);
    "^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.past_positions_len(), 3);

    let mut grid = InfiniteGrid::new(2);
    "^>v<".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.past_positions_len(), 3);

    let mut grid = InfiniteGrid::new(2);
    "^v^v^v^v^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.past_positions_len(), 11);
}

#[test]
fn test_2015_day_3() {
    println!("Advent of Code 2015 - Day 3");
    let contents =
        fs::read_to_string("input/2015/day-3.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(1);

    contents.chars().for_each(|c| grid.move_position(c));

    let houses_visited = grid.past_positions_len();
    println!("Santa visited {} houses at least once.", houses_visited);
    assert_eq!(houses_visited, 2081);

    let mut grid = InfiniteGrid::new(2);

    contents.chars().for_each(|c| grid.move_position(c));

    let houses_visited = grid.past_positions_len();
    println!(
        "Santa and Robo-Santa visited {} houses at least once.",
        houses_visited
    );
    assert_eq!(houses_visited, 2341);
}

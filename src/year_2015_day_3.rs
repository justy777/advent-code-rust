use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new() -> Cell {
        Cell { x: 0, y: 0 }
    }

    fn move_cell(&self, direction: char) -> Option<Cell> {
        match direction {
            '^' => Some(Cell { x: self.x, y: self.y + 1 }),
            'v' => Some(Cell { x: self.x, y: self.y - 1 }),
            '>' => Some(Cell { x: self.x + 1, y: self.y }),
            '<' => Some(Cell { x: self.x - 1, y: self.y }),
            _ => None,
        }
    }
}

struct Grid {
    current_positions: Vec<Cell>,
    past_positions: HashSet<Cell>,
    turn: usize,
}

impl Grid {
    fn new(size: usize) -> Grid {
        let mut past_positions = HashSet::new();
        past_positions.insert(Cell::new());
        Grid {
            current_positions: vec![Cell::new(); size],
            past_positions,
            turn: 0,
        }
    }

    fn move_houses(&mut self, direction: char) {
        let position = self.current_positions[self.turn];
        let new_position = &position.move_cell(direction).unwrap();

        self.past_positions.insert(*new_position);
        self.current_positions[self.turn] = *new_position;

        self.turn = (self.turn + 1) % self.current_positions.len();
    }
}

#[test]
fn test_2015_day_3() {
    println!("Advent of Code 2015 - Day 3");
    let contents = fs::read_to_string("input/2015/day-3.txt")
        .expect("Failed to read file to String");

    let mut grid = Grid::new(1);

    contents.chars().for_each(|c| grid.move_houses(c));

    let houses_visited = grid.past_positions.len();
    println!("Santa visited {} houses at least once.", houses_visited);
    assert_eq!(houses_visited, 2081);

    let mut grid = Grid::new(2);

    contents.chars().for_each(|c| grid.move_houses(c));

    let houses_visited = grid.past_positions.len();
    println!("Santa and Robo-Santa visited {} houses at least once.", houses_visited);
    assert_eq!(houses_visited, 2341);
}
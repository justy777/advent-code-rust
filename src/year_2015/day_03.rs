/*!
--- Day 3: Perfectly Spherical Houses in a Vacuum ---
Santa is delivering presents to an infinite two-dimensional grid of houses.
He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next.
However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once.
*/

use hashbrown::HashSet;

/// Represents a position in the grid.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    /// Constructs a new `Position` at the origin (0,0).
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    /// Returns a new `Position` moved in the provided direction.
    /// If an invalid direction is provided, `None` is returned.
    ///
    /// Moves are always exactly one to the north `^`, south `v`, east `>`, or west `<`.
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

/// Represents an infinite grid of houses.
pub struct InfiniteGrid {
    current_positions: Vec<Position>,
    past_positions: HashSet<Position>,
    turn: usize,
}

impl InfiniteGrid {
    /// Constructs a new `InfiniteGrid` with the provided number of positions at the origin (0,0).
    /// If parallel is more than one, positions will take turns moving.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::year_2015::day_03::InfiniteGrid;
    ///
    /// let g = InfiniteGrid::new(1);
    /// ```
    pub fn new(parallel: usize) -> InfiniteGrid {
        let mut past_positions = HashSet::new();
        past_positions.insert(Position::new());
        InfiniteGrid {
            current_positions: vec![Position::new(); parallel],
            past_positions,
            turn: 0,
        }
    }

    /// Moves the current position in the provided direction.
    ///
    /// If parallel is more than one, positions will take turns moving.
    ///
    /// Moves are always exactly one house to the north `^`, south `v`, east `>`, or west `<`.
    pub fn move_position(&mut self, direction: char) {
        if self.current_positions.is_empty() {
            return;
        }

        let position = self.current_positions[self.turn];
        if let Some(new_position) = position.move_direction(direction) {
            self.past_positions.insert(new_position);
            self.current_positions[self.turn] = new_position;

            self.turn = (self.turn + 1) % self.current_positions.len();
        }
    }

    /// Returns the number of unique positions visited.
    ///
    /// The starting location counts as one visited position.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::year_2015::day_03::InfiniteGrid;
    ///
    /// let mut grid = InfiniteGrid::new(1);
    /// grid.move_position('>');
    /// assert_eq!(grid.visited(), 2);
    /// ```
    pub fn visited(&self) -> usize {
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
fn test_grid_parallel_zero() {
    let mut grid = InfiniteGrid::new(0);
    grid.move_position('^');
    assert!(grid.current_positions.is_empty());
    assert_eq!(grid.current_positions.len(), 0);
    assert_eq!(grid.past_positions.len(), 1);
}

#[test]
fn test_grid_single_position() {
    let mut grid = InfiniteGrid::new(1);
    grid.move_position('>');
    assert_eq!(grid.visited(), 2);

    let mut grid = InfiniteGrid::new(1);
    "^>v<".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.visited(), 4);

    let mut grid = InfiniteGrid::new(1);
    "^v^v^v^v^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.visited(), 2);
}

#[test]
fn test_grid_two_positions() {
    let mut grid = InfiniteGrid::new(2);
    "^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.visited(), 3);

    let mut grid = InfiniteGrid::new(2);
    "^>v<".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.visited(), 3);

    let mut grid = InfiniteGrid::new(2);
    "^v^v^v^v^v".chars().for_each(|c| grid.move_position(c));
    assert_eq!(grid.visited(), 11);
}

#[test]
fn test_grid_single_position_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-3.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(1);

    contents.chars().for_each(|c| grid.move_position(c));

    let visited = grid.visited();
    assert_eq!(visited, 2081);
}

#[test]
fn test_grid_two_positions_input_file() {
    let contents =
        std::fs::read_to_string("input/2015/day-3.txt").expect("Failed to read file to string.");

    let mut grid = InfiniteGrid::new(2);

    contents.chars().for_each(|c| grid.move_position(c));

    let visited = grid.visited();
    assert_eq!(visited, 2341);
}

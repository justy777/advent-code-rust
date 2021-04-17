/*!
--- Day 1: Not Quite Lisp ---
Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing.
*/

/// InfiniteBuilding holds state for the current floor, and position.
///
/// The apartment building is very tall, and the basement is very deep; you will never find the top or bottom floors.
#[derive(Debug)]
struct InfiniteBuilding {
    floor: i32,
    position: i32,
}

impl InfiniteBuilding {
    /// Constructs a new `InfiniteBuilding`.
    ///
    /// Starts on the ground floor (floor 0).
    fn new() -> InfiniteBuilding {
        InfiniteBuilding {
            floor: 0,
            position: 0,
        }
    }

    fn apply(&mut self, c: &u8) {
        match c {
            b'(' => {
                self.floor += 1;
                self.position += 1;
            }
            b')' => {
                self.floor -= 1;
                self.position += 1;
            }
            _ => (),
        }
    }
}

/// Returns the floor that the instructions take you to.
///
/// An opening parenthesis `(` means he should go up one floor, and a closing parenthesis `)` means he should go down one floor.
///
/// # Examples
///
/// ```
/// use advent_of_code::year_2015::day_01::floor;
///
/// let f = floor(b"(())");
/// assert_eq!(f, 0);
///
/// let f = floor(b"()()");
/// assert_eq!(f, 0);
///
/// let f = floor(b"(((");
/// assert_eq!(f, 3);
///
/// let f = floor(b"(()(()(");
/// assert_eq!(f, 3);
///
/// let f = floor(b"))(((((");
/// assert_eq!(f, 3);
///
/// let f = floor(b"())");
/// assert_eq!(f, -1);
///
/// let f = floor(b"))(");
/// assert_eq!(f, -1);
///
/// let f = floor(b")))");
/// assert_eq!(f, -3);
///
/// let f = floor(b")())())");
/// assert_eq!(f, -3);
/// ```
pub fn floor(instructions: &[u8]) -> i32 {
    let mut building = InfiniteBuilding::new();
    instructions.iter().for_each(|c| building.apply(c));
    building.floor
}

/// Returns the position of the first instruction that bring you to the provided floor.
/// If no instruction takes you to the provided floor `None` is returned.
///
/// An opening parenthesis `(` means he should go up one floor, and a closing parenthesis `)` means he should go down one floor.
///
/// # Examples
///
/// ```
/// use advent_of_code::year_2015::day_01::position;
///
/// let p = position(b")", -1);
/// assert_eq!(p, Some(1));
///
/// let p = position(b"()())", -1);
/// assert_eq!(p, Some(5));
/// ```
pub fn position(parentheses: &[u8], floor: i32) -> Option<i32> {
    if floor == 0 {
        return Some(0);
    }

    let mut building = InfiniteBuilding::new();
    for c in parentheses.iter() {
        building.apply(c);
        if building.floor == floor {
            return Some(building.position);
        }
    }
    None
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_change(direction: &Direction) -> [i32; 2] {
    match direction {
        Direction::Up => [0, -1],
        Direction::Right => [1, 0],
        Direction::Down => [0, 1],
        Direction::Left => [-1, 0],
    }
}
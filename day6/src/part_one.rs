use std::collections::HashSet;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct XY {
    x: usize,
    y: usize,
}

pub fn solve_part_one(input: &[String]) -> usize {
    // Convert Vector of strings -> 2d vector of chars
    let input_arrays: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // Find the starting position of the guard
    let position: Vec<_> = input_arrays
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, n)| if *n == '^' { Some([x, y]) } else { None })
        })
        .flatten()
        .collect();

    let mut visited: HashSet<XY> = HashSet::new();
    take_step(
        &input_arrays,
        XY {
            x: position[0],
            y: position[1],
        },
        &mut visited,
        Direction::Up,
    )
    .len()
}

fn direction_to_coordinates(direction: &Direction) -> [i32; 2] {
    match direction {
        Direction::Up => [0, -1],
        Direction::Right => [1, 0],
        Direction::Down => [0, 1],
        Direction::Left => [-1, 0],
    }
}

fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn take_step(
    map: &Vec<Vec<char>>,
    position: XY,
    visited: &mut HashSet<XY>,
    direction: Direction,
) -> HashSet<XY> {
    if is_done(map, &position, &direction) {
        visited.insert(position);
        return visited.clone();
    }

    let next_position = XY {
        x: (position.x as i32 + direction_to_coordinates(&direction)[0]) as usize,
        y: (position.y as i32 + direction_to_coordinates(&direction)[1]) as usize,
    };

    if map[next_position.y][next_position.x] == '#' {
        return take_step(map, position, visited, next_direction(&direction));
    }

    visited.insert(position);
    take_step(map, next_position, visited, direction)
}

fn is_done(map: &[Vec<char>], position: &XY, direction: &Direction) -> bool {
    match direction {
        Direction::Up => position.y == 0,
        Direction::Right => position.x == map[0].len() - 1,
        Direction::Down => position.y == map.len() - 1,
        Direction::Left => position.x == 0,
    }
}

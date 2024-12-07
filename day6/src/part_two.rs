use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq)]
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

pub fn solve_part_two(input: &[String]) -> usize {
    // Convert Vector of strings -> 2d vector of chars
    let input_arrays: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // Find the starting position of the guard
    let start_position_vec: Vec<_> = input_arrays
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, n)| if *n == '^' { Some([x, y]) } else { None })
        })
        .flatten()
        .collect();

    let start_position = XY {
        x: start_position_vec[0],
        y: start_position_vec[1],
    };

    let visited = find_path(
        &input_arrays,
        &start_position,
        &mut HashSet::new(),
        Direction::Up,
    );

    visited.iter().fold(0, |acc, xy| {
        if xy == &start_position {
            return acc;
        }

        if is_looping(
            &input_arrays,
            &start_position,
            &mut HashSet::new(),
            &Direction::Up,
            xy,
        ) {
            acc + 1
        } else {
            acc
        }
    })
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

fn find_path(
    map: &Vec<Vec<char>>,
    position: &XY,
    visited: &mut HashSet<XY>,
    direction: Direction,
) -> HashSet<XY> {
    if is_done(map, &position, &direction) {
        visited.insert(position.clone());
        return visited.clone();
    }

    let next_position = XY {
        x: (position.x as i32 + direction_to_coordinates(&direction)[0]) as usize,
        y: (position.y as i32 + direction_to_coordinates(&direction)[1]) as usize,
    };

    if map[next_position.y][next_position.x] == '#' {
        return find_path(map, position, visited, next_direction(&direction));
    }

    visited.insert(position.clone());
    find_path(map, &next_position, visited, direction)
}

fn is_looping(
    map: &Vec<Vec<char>>,
    position: &XY,
    obstructions_seen: &mut HashSet<(XY, Direction)>,
    direction: &Direction,
    new_obstruction: &XY,
) -> bool {
    if is_done(map, &position, direction) {
        return false;
    }

    let next_position = XY {
        x: (position.x as i32 + direction_to_coordinates(direction)[0]) as usize,
        y: (position.y as i32 + direction_to_coordinates(direction)[1]) as usize,
    };

    if map[next_position.y][next_position.x] == '#' || &next_position == new_obstruction {
        if obstructions_seen.contains(&(next_position.clone(), direction.clone())) {
            return true;
        }

        obstructions_seen.insert((next_position.clone(), direction.clone()));
        return is_looping(
            map,
            position,
            obstructions_seen,
            &next_direction(direction),
            new_obstruction,
        );
    }

    is_looping(
        map,
        &next_position,
        obstructions_seen,
        direction,
        new_obstruction,
    )
}

fn is_done(map: &[Vec<char>], position: &XY, direction: &Direction) -> bool {
    match direction {
        Direction::Up => position.y == 0,
        Direction::Right => position.x == map[0].len() - 1,
        Direction::Down => position.y == map.len() - 1,
        Direction::Left => position.x == 0,
    }
}

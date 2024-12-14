enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct XY {
    x: i32,
    y: i32,
}

pub fn solve_part_two(input: &[String]) -> usize {
    let input_nums: Vec<Vec<usize>> = input
        .iter()
        .map(|line| line.chars().map(|c| c as usize - '0' as usize).collect())
        .collect();

    input_nums.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(acc, |acc, (x, elevation)| {
            if *elevation != 0 {
                return acc;
            }

            acc + check_path(
                &input_nums,
                XY {
                    x: x as i32,
                    y: y as i32,
                },
                0,
                0,
            )
        })
    })
}

fn check_path(map: &[Vec<usize>], pos: XY, elevation: usize, paths_found: usize) -> usize {
    if elevation == 9 {
        return paths_found + 1;
    }

    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    directions.iter().fold(paths_found, |acc, d| {
        let change = direction_to_coordinates(d);
        let next_pos = XY {
            x: pos.x + change[0],
            y: pos.y + change[1],
        };
        if next_pos.x < 0 || next_pos.x >= map[0].len() as i32 {
            return acc;
        }

        if next_pos.y < 0 || next_pos.y >= map.len() as i32 {
            return acc;
        }

        if map[next_pos.y as usize][next_pos.x as usize] == elevation + 1 {
            check_path(map, next_pos, elevation + 1, acc)
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

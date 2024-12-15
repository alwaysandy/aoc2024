use std::collections::HashSet;
use utils::*;

pub fn solve(floor: &[Vec<char>], moves: &[char]) -> usize {
    let walls: HashSet<XY> = floor
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                if *c == '#' {
                    Some(XY {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    let mut boxes: HashSet<XY> = floor
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                if *c == 'O' {
                    Some(XY {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    let robot: XY = floor
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter().position(|c| *c == '@').map(|x| XY {
                x: x as i32,
                y: y as i32,
            })
        })
        .collect::<Vec<_>>()[0]
        .clone();

    move_robot(moves, robot, &walls, &mut boxes);

    boxes.iter().fold(0, |acc, box_pos| {
        acc + ((100 * box_pos.y) + box_pos.x) as usize
    })
}

fn move_robot(moves: &[char], robot: XY, walls: &HashSet<XY>, boxes: &mut HashSet<XY>) {
    moves.iter().fold(robot, |robot, c| {
        let direction = char_to_direction(c);
        let change = get_change(&direction);
        let next_pos = XY {
            x: robot.x + change[0],
            y: robot.y + change[1],
        };

        if walls.contains(&next_pos) {
            return robot;
        }

        if boxes.contains(&next_pos) {
            if move_boxes(&next_pos, &direction, walls, boxes) {
                boxes.remove(&next_pos);
                return next_pos;
            } else {
                return robot;
            }
        }

        next_pos
    });
}

fn move_boxes(
    pos: &XY,
    direction: &Direction,
    walls: &HashSet<XY>,
    boxes: &mut HashSet<XY>,
) -> bool {
    if walls.contains(pos) {
        return false;
    }

    let change = get_change(direction);
    let next_pos = XY {
        x: pos.x + change[0],
        y: pos.y + change[1],
    };

    if boxes.contains(pos) {
        return move_boxes(&next_pos, direction, walls, boxes);
    }

    boxes.insert(pos.clone());
    true
}

fn char_to_direction(c: &char) -> Direction {
    match c {
        '>' => Direction::Right,
        '<' => Direction::Left,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => panic!("This should be impossible!!"),
    }
}

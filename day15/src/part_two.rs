use std::collections::{HashMap, HashSet};
use std::io;
use utils::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Side {
    Left,
    Right,
}

pub fn solve(floor: &[Vec<char>], moves: &[char]) -> usize {
    let walls: HashSet<XY> = floor
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter().enumerate().flat_map(move |(x, c)| {
                if *c == '#' {
                    vec![XY {
                        x: x as i32 * 2,
                        y: y as i32,
                    },
                    XY {
                        x: (x as i32 * 2) + 1,
                        y: y as i32
                    }]
                } else {
                    vec![]
                }
            })
        })
        .collect();

    let mut boxes: HashMap<XY, Side> = HashMap::new();
    floor
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| {
                if *c == 'O' {
                    boxes.insert(XY {x: x as i32 * 2, y: y as i32}, Side::Left);
                    boxes.insert(XY {x: (x as i32 * 2) + 1, y: y as i32}, Side::Right);
                }
            })
        });

    let robot: XY = floor.iter().enumerate().filter_map(|(y, line)| {
        let x = line.iter().position(|c| *c == '@');
        if x.is_some() {
            Some(XY {x: x.unwrap() as i32 * 2, y: y as i32})
        } else {
            None
        }
    }).collect::<Vec<_>>()[0].clone();

    let moved_boxes = move_robot(moves, &robot, &walls, &mut boxes);
    moved_boxes.iter().fold(0, |acc, box_pos| {
        if box_pos.1 == &Side::Left {
            acc + ((100*box_pos.0.y) + box_pos.0.x) as usize
        } else {
            acc
        }
    })
}

fn move_robot(moves: &[char], robot: &XY, walls: &HashSet<XY>, boxes: &mut HashMap<XY, Side>) -> HashMap<XY, Side> {
    moves.iter().fold((robot.clone(), boxes.clone()), |(robot, boxes), c| {
        let direction = char_to_direction(c);
        let change = get_change(&direction);
        let next_pos = XY {
            x: robot.x + change[0],
            y: robot.y + change[1]
        };

        if walls.contains(&next_pos) {
            return (robot, boxes);
        }

        if !boxes.contains_key(&next_pos) {
            return (next_pos, boxes);
        }

        if let Some(new_boxes) = can_move_boxes(&next_pos, &direction, walls, &boxes) {
            (next_pos, new_boxes)
        } else {
            (robot, boxes)
        }
    }).1
}


fn can_move_boxes(pos: &XY, direction: &Direction, walls: &HashSet<XY>, boxes: &HashMap<XY, Side>) -> Option<HashMap<XY, Side>> {
    match direction {
        Direction::Left | Direction::Right => {
            if walls.contains(pos) {
                return None;
            }

            if !boxes.contains_key(pos) {
                return Some(boxes.clone())
            }

            let change = get_change(direction);
            let next_pos = XY {
                x: pos.x + change[0],
                y: pos.y + change[1]
            };

            if let Some(mut moved_boxes) = can_move_boxes(&next_pos, direction, walls, boxes) {
                let side = moved_boxes.get(pos).unwrap().clone();
                moved_boxes.remove(pos);
                moved_boxes.insert(next_pos, side);
                Some(moved_boxes)
            } else {
                None
            }
        },
        Direction::Down | Direction::Up => {
            if walls.contains(pos) {
                return None;
            }

            if !boxes.contains_key(pos) {
                return Some(boxes.clone());
            }

            let change = get_change(direction);
            let next_pos = XY {
                x: pos.x + change[0],
                y: pos.y + change[1]
            };

            let side = boxes.get(&pos).unwrap().clone();
            let side_connected = get_opposite_side(&side);
            let pos_connected = if side_connected == Side::Left {
                XY { x: pos.x - 1, y: pos.y }
            } else {
                XY { x: pos.x + 1, y: pos.y }
            };

            let next_pos_connected = XY {
                x: pos_connected.x,
                y: pos_connected.y + change[1]
            };

            if let Some(mut moved_boxes) = can_move_boxes(&next_pos, direction, walls, boxes) {
                if let Some(mut moved_boxes) = can_move_boxes(&next_pos_connected, direction, walls, &mut moved_boxes) {
                    moved_boxes.remove(&pos);
                    moved_boxes.remove(&pos_connected);
                    moved_boxes.insert(next_pos.clone(), side.clone());
                    moved_boxes.insert(next_pos_connected.clone(), side_connected.clone());
                    Some(moved_boxes)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
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

fn get_opposite_side(side: &Side) -> Side {
    match side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}

fn print_all(boxes: &HashMap<XY, Side>, walls: &HashSet<XY>, robot: &XY) {
    let width: usize = walls.iter().map(|c| c.x).max().unwrap() as usize;
    let height: usize = walls.iter().map(|c| c.y).max().unwrap() as usize;
    let mut view: Vec<Vec<char>> = vec![vec!['.'; width as usize + 1]; height as usize + 1];
    for y in 0..=height {
        for x in 0..=width {
            if walls.contains(&XY { x: x as i32, y: y as i32 }) {
                view[y][x] = '#';
            }

            if boxes.contains_key(&XY { x: x as i32, y: y as i32 }) {
                if boxes.get(&XY { x: x as i32, y: y as i32}).unwrap() == &Side::Left {
                    view[y][x] = '[';
                } else {
                    view[y][x] = ']';
                }
            }

            if x == robot.x as usize && y == robot.y as usize {
                view[y][x] = '@';
            }
        }
    }

    for line in view {
        println!("{}", line.iter().collect::<String>());
    }
}
use std::collections::{HashMap, VecDeque};
use utils::*;

pub fn solve(input: &[Vec<char>]) {
    let start: XY = input
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter().position(|c| *c == 'S').map(|x| XY {
                x: x as i32,
                y: y as i32,
            })
        })
        .collect::<Vec<_>>()[0]
        .clone();

    let end: XY = input
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter().position(|c| *c == 'E').map(|x| XY {
                x: x as i32,
                y: y as i32,
            })
        })
        .collect::<Vec<_>>()[0]
        .clone();

    let mut scores: HashMap<XY, usize> = HashMap::new();
    let mut queue: VecDeque<(XY, Direction, usize)> = VecDeque::new();
    scores.insert(start.clone(), 0);
    queue.push_front((start.clone(), Direction::Right, 0));

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    while !queue.is_empty() {
        let (curr, dir, score) = queue.pop_front().unwrap();

        if scores.contains_key(&curr) {
            if scores.get(&curr).unwrap() >= &score {
                scores.insert(curr.clone(), score);
            } else {
                continue;
            }
        } else {
            scores.insert(curr.clone(), score);
        }

        directions.iter().for_each(|direction| {
            let new_score = if direction == &dir {
                score + 1
            } else if direction == &get_opposite_direction(&dir) {
                score + 2001
            } else {
                score + 1001
            };

            let change = get_change(direction);
            if curr.x + change[0] < 0 || curr.y + change[1] < 0 {
                return;
            } else if curr.x + change[0] >= input[0].len() as i32
                || curr.y + change[1] >= input.len() as i32
            {
                return;
            }

            let new_loc = XY {
                x: curr.x + change[0],
                y: curr.y + change[1],
            };

            if scores.contains_key(&new_loc) {
                if scores.get(&new_loc).unwrap() <= &new_score {
                    return;
                }
            }

            if input[new_loc.y as usize][new_loc.x as usize] == '.'
                || input[new_loc.y as usize][new_loc.x as usize] == 'E'
            {
                queue.push_front((new_loc, direction.clone(), new_score));
            }
        });
    }

    println!("{}", scores.get(&end).unwrap());
}

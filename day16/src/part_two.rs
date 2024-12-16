use std::collections::{HashMap, HashSet, VecDeque};
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
                // THIS MAY NOT BE THE WAY, WE MAY NEED TO TRACK DIRECTION IN THE FUTURE????
                // THIS IS THE PLACE THINGS WILL DEFINITELY GO WRONG
                continue;
            }
        } else {
            scores.insert(curr.clone(), usize::max_value());
        }

        directions
            .iter()
            .for_each(|direction| {
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
                } else if curr.x + change[0] >= input[0].len() as i32 || curr.y + change[1] >= input.len() as i32 {
                    return;
                }

                let new_loc = XY {
                    x: curr.x + change[0],
                    y: curr.y + change[1],
                };

                if scores.contains_key(&new_loc) {
                    if scores.get(&new_loc).unwrap() <= &new_score {
                        return
                    }
                }

                if input[new_loc.y as usize][new_loc.x as usize] == '.' || input[new_loc.y as usize][new_loc.x as usize] == 'E' {
                    queue.push_front((new_loc, direction.clone(), new_score));
                }
            });
    }

    let max_score=  scores.get(&end).unwrap();
    let valid_squares: HashSet<XY> = scores.iter().filter_map(|(c, score)| if score <= &max_score {Some(c.clone())} else {None}).collect();

    let mut correct_path: HashSet<XY> = HashSet::new();
    correct_path.insert(start.clone());
    let mut failed_at: HashMap<(XY, Direction), usize> = HashMap::new();
    dfs(&valid_squares, &mut HashSet::new(), &Direction::Right, &start, &end, &mut correct_path, 0, max_score.clone(), &mut failed_at);

    println!("{:?}", correct_path.len());
    print_all(input, &correct_path);
}

fn dfs(input: &HashSet<XY>, path: &mut HashSet<XY>, dir: &Direction, loc: &XY, end: &XY, correct_path: &mut HashSet<XY>, score: usize, max_score: usize, failed_at: &mut HashMap<(XY, Direction), usize>) -> bool {
    if failed_at.contains_key(&(loc.clone(), dir.clone())) {
        if &score >= failed_at.get(&(loc.clone(), dir.clone())).unwrap() {
            return false;
        }
    }

    if loc == end {
        correct_path.insert(loc.clone());
        return true;
    }

    let directions = get_all_directions();
    path.insert(loc.clone());

    let mut is_valid = false;
    directions.iter().for_each(|next_dir| {
        let new_score = if dir == next_dir {
            score + 1
        } else if &get_opposite_direction(&dir) == next_dir {
            score + 2001
        } else {
            score + 1001
        };

        if new_score > max_score {
            return;
        }

        let change = get_change(next_dir);
        let next_loc = XY {
            x: loc.x + change[0],
            y: loc.y + change[1],
        };

        if path.contains(&next_loc) {
            return;
        }

        if !input.contains(&next_loc) {
            return;
        }

        if dfs(input, path, next_dir, &next_loc, end, correct_path, new_score, max_score, failed_at) {
            is_valid = true;
            correct_path.insert(next_loc.clone());
        }
    });

    path.remove(&loc);

    if !is_valid {
        failed_at.insert((loc.clone(), dir.clone()), score);
    }
    is_valid
}

fn print_all(input: &[Vec<char>], valid_squares: &HashSet<XY>) {
    let mut editable = input.clone().to_owned();
    valid_squares.iter().for_each(|s| {
        editable[s.y as usize][s.x as usize] = '$';
    });

    editable.iter().for_each(|s| {println!("{}", s.into_iter().collect::<String>())})
}
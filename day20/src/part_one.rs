use std::collections::{HashMap, HashSet, VecDeque};
use utils::{CHANGES, XY};

pub fn solve(input: &[String]) {
    let maze: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let start: XY = maze
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

    let path = initial_bfs(&maze, &start);
    let walls_to_skip =  get_to_skip(&maze, &path);
    let initial =  bfs(&maze, &start, &XY {x: -1, y: -1});

    let mut skipped_gains: HashMap<usize, usize> = HashMap::new();
     walls_to_skip.iter().for_each(|wall_to_skip| {
        let gains = (initial - bfs(&maze, &start, wall_to_skip));
         *skipped_gains.entry(gains).or_insert(0) += 1;
    });

    let mut over_one_hundred = 0;
    skipped_gains.iter().for_each(|(gain, count)| {
        if *gain >= 100 {
            over_one_hundred += count;
        }
    });

    println!("Over one hundred: {}", over_one_hundred);
}

fn initial_bfs(input: &[Vec<char>], start: &XY) -> HashSet<XY> {
    let mut explored: HashSet<XY> = HashSet::new();
    let mut queue: VecDeque<(XY, usize)> = VecDeque::new();

    explored.insert(start.clone());
    queue.push_back((start.clone(), 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if input[curr.y as usize][curr.x as usize] == 'E' {
            return explored;
        }

        CHANGES.iter().for_each(|d| {
            if !is_valid(&curr, input, d) {
                return;
            }

            let next = XY {
                x: curr.x + d[0],
                y: curr.y + d[1],
            };

            if explored.contains(&next) || input[next.y as usize][next.x as usize] == '#' {
                return;
            }

            explored.insert(next.clone());
            queue.push_back((next, steps + 1));
        });
    }

    HashSet::new()
}

fn bfs(input: &[Vec<char>], start: &XY, wall_to_skip: &XY) -> usize {
    let mut explored: HashSet<XY> = HashSet::new();
    let mut queue: VecDeque<(XY, usize)> = VecDeque::new();

    explored.insert(start.clone());
    queue.push_back((start.clone(), 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if input[curr.y as usize][curr.x as usize] == 'E' {
            return steps;
        }

        CHANGES.iter().for_each(|d| {
            if !is_valid(&curr, input, d) {
                return;
            }

            let next = XY {
                x: curr.x + d[0],
                y: curr.y + d[1],
            };

            if explored.contains(&next) {
                return;
            }

            if &next != wall_to_skip && input[next.y as usize][next.x as usize] == '#' {
                return;
            }

            explored.insert(next.clone());
            queue.push_back((next, steps + 1));
        });
    }

    0
}

fn get_to_skip(input: &[Vec<char>], path: &HashSet<XY>) -> HashSet<XY> {
    path.iter().flat_map(|pos| {
        CHANGES.iter().filter_map(|c| {
            if !is_valid(pos, input, c) {
                return None;
            }

            let wall = XY {
                x: pos.x + c[0],
                y: pos.y + c[1]
            };

            if input[wall.y as usize][wall.x as usize] != '#' {
                return None;
            }

            if count_empty_space(input, &wall) >= 2 {
                Some(wall)
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).collect()
}

fn count_empty_space(input: &[Vec<char>], pos: &XY) -> usize {
    CHANGES.iter().fold(0, |acc, c| {
        if is_valid(pos, input, c) {
            if input[(pos.y + c[1]) as usize][(pos.x + c[0]) as usize] != '#' {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

fn is_valid(pos: &XY, input: &[Vec<char>], c: &[i32; 2]) -> bool {
    !(pos.x + c[0] < 0 || pos.y + c[1] < 0 || pos.x + c[0] >= input[0].len() as i32 || pos.y + c[1] >= input.len() as i32)
}
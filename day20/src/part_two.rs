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

    let path = get_path(&maze, &start);
    println!("{:?}", path.iter().fold(0, |mut acc, (p, s)| {
        acc + cells_n_away(&path, p, 20)
    }));

}

fn get_path(input: &[Vec<char>], start: &XY) -> HashMap<XY, usize> {
    let mut path: HashMap<XY, usize> = HashMap::new();
    let mut queue: VecDeque<(XY, usize)> = VecDeque::new();

    path.insert(start.clone(), 0);
    queue.push_back((start.clone(), 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if input[curr.y as usize][curr.x as usize] == 'E' {
            return path;
        }

        CHANGES.iter().for_each(|d| {
            if !is_valid(&curr, input, d) {
                return;
            }

            let next = XY {
                x: curr.x + d[0],
                y: curr.y + d[1],
            };

            if path.contains_key(&next) || input[next.y as usize][next.x as usize] == '#' {
                return;
            }

            path.insert(next.clone(), steps + 1);
            queue.push_back((next, steps + 1));
        });
    }

    HashMap::new()
}

fn cells_n_away(path: &HashMap<XY, usize>, start: &XY, walls_to_skip: usize) -> usize {
    let mut explored: HashSet<XY> = HashSet::new();
    let mut queue: VecDeque<(XY, usize)> = VecDeque::new();
    let mut over_one_hundred = 0;

    explored.insert(start.clone());
    queue.push_back((start.clone(), 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if path.contains_key(&curr) {
            if path[&curr] > path[start] {
                let dist_saved = path[&curr] - path[start] - steps;
                if dist_saved >= 100 {
                    over_one_hundred += 1;
                }
            }
        }


        if steps == walls_to_skip {
            continue;
        }

        CHANGES.iter().for_each(|d| {
            let next = XY {
                x: curr.x + d[0],
                y: curr.y + d[1],
            };

            if explored.contains(&next) {
                return;
            }

            explored.insert(next.clone());
            queue.push_back((next, steps + 1));
        });
    }

    over_one_hundred
}

fn is_valid(pos: &XY, input: &[Vec<char>], c: &[i32; 2]) -> bool {
    !(pos.x + c[0] < 0 || pos.y + c[1] < 0 || pos.x + c[0] >= input[0].len() as i32 || pos.y + c[1] >= input.len() as i32)
}
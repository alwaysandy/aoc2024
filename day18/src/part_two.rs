use std::collections::{HashSet, VecDeque};
use utils::*;

pub fn solve(input: &[String]) -> XY {
    let mut bytes = 2875;
    loop {
        if bytes == 3451 {
            break;
        }
        let mut last = XY { x: 0, y: 0 };
        let corrupted: HashSet<XY> = input
            .iter()
            .enumerate()
            .filter_map(|(n, s)| {
                if n > bytes {
                    return None;
                }

                let (x, y) = s.split_once(',').unwrap();
                if n == bytes {
                    last = XY {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                }

                Some(XY {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                })
            })
            .collect();
        let p_len = bfs(&corrupted, 70);
        if p_len == 0 {
            return last;
        }

        bytes += 1;
    }

    return XY { x: 0, y: 0 };
}

fn bfs(corrupted: &HashSet<XY>, max: i32) -> usize {
    let mut explored: HashSet<XY> = HashSet::new();
    let mut queue: VecDeque<(XY, usize)> = VecDeque::new();

    explored.insert(XY { x: 0, y: 0 });
    queue.push_back((XY { x: 0, y: 0 }, 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if curr == (XY { x: max, y: max }) {
            return steps;
        }

        CHANGES.iter().for_each(|d| {
            if curr.x + d[0] < 0 || curr.y + d[1] < 0 || curr.x + d[0] > max || curr.y + d[1] > max
            {
                return;
            }

            let next = XY {
                x: curr.x + d[0],
                y: curr.y + d[1],
            };

            if explored.contains(&next) || corrupted.contains(&next) {
                return;
            }

            explored.insert(next.clone());
            queue.push_back((next, steps + 1));
        });
    }
    0
}

fn print_corrupted(corrupted: &HashSet<XY>, max: i32) {
    for y in 0..max {
        let mut line = ['.'; 70];
        for x in 0..max {
            if corrupted.contains(&XY { x: x, y: y }) {
                line[x as usize] = '#';
            }
        }

        println!("{}", line.iter().collect::<String>());
    }
}

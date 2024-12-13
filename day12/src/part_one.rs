use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct RegionNode {
    x: i32,
    y: i32,
    c: char,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve_part_one(input: &[Vec<char>]) -> usize {
    let mut regions: HashMap<char, Vec<HashSet<RegionNode>>> = HashMap::new();
    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            let start_node = RegionNode {
                x: x as i32,
                y: y as i32,
                c: *c,
            };

            if regions.contains_key(&c) {
                if regions[c].iter().any(|region| region.contains(&start_node)) {
                    return;
                }
            }

            regions
                .entry(*c)
                .or_insert(Vec::new())
                .push(flood_fill(input, start_node));
        })
    });

    regions.iter().fold(0, |acc, (c, region_vec)| {
        region_vec.iter().fold(acc, |acc, region| {
            acc + region.len() * find_perimeter(input, region)
        })
    })
}

fn flood_fill(input: &[Vec<char>], start: RegionNode) -> HashSet<RegionNode> {
    let mut stack: Vec<RegionNode> = Vec::new();
    stack.push(start.clone());
    let mut region: HashSet<RegionNode> = HashSet::new();
    region.insert(start);

    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    loop {
        if stack.is_empty() {
            return region;
        }

        let node = stack.pop().unwrap();
        region.insert(node.clone());
        for direction in &directions {
            if let Some(change) = is_valid_change(input, &node, direction) {
                let next_node = RegionNode {
                    x: node.x + change[0],
                    y: node.y + change[1],
                    c: node.c,
                };

                if !region.contains(&next_node) {
                    stack.push(next_node);
                }
            }
        }
    }
}

fn find_perimeter(input: &[Vec<char>], region: &HashSet<RegionNode>) -> usize {
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    region.iter().fold(0, |acc, node| {
        directions.iter().fold(acc, |acc, direction| {
            if let Some(_) = is_valid_change(input, &node, direction) {
                acc
            } else {
                acc + 1
            }
        })
    })
}

// TODO please stop requiring this..
fn direction_to_coordinates(direction: &Direction) -> [i32; 2] {
    match direction {
        Direction::Up => [0, -1],
        Direction::Right => [1, 0],
        Direction::Down => [0, 1],
        Direction::Left => [-1, 0],
    }
}

fn is_valid_change(
    input: &[Vec<char>],
    node: &RegionNode,
    direction: &Direction,
) -> Option<[i32; 2]> {
    let change = direction_to_coordinates(direction);
    if node.x + change[0] < 0
        || node.x + change[0] >= input[0].len() as i32
        || node.y + change[1] < 0
        || node.y + change[1] >= input.len() as i32
    {
        return None;
    }

    if input[(node.y + change[1]) as usize][(node.x + change[0]) as usize] == node.c {
        Some(change)
    } else {
        None
    }
}

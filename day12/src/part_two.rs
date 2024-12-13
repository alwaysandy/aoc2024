use std::collections::{HashMap, HashSet};
use std::env::current_dir;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct RegionNode {
    x: i32,
    y: i32,
    c: char,
}

// WE NEED EVERY DIRECTION
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve_part_two(input: &[Vec<char>]) -> usize {
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

    for (c, region_vec) in &regions {
        for region in region_vec {
            println!("{} {}", c, count_edges(input, region));
        }
    }

    regions.iter().fold(0, |acc, (c, region_vec)| {
        region_vec.iter().fold(acc, |acc, region| {
            return acc + (region.len() * count_edges(input, region));
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
fn count_edges(input: &[Vec<char>], region: &HashSet<RegionNode>) -> usize {
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let edge_nodes: HashSet<RegionNode> = region
        .iter()
        .filter_map(|node| {
            let valid_edges = directions.iter().fold(0, |acc, direction| {
                if let Some(_) = is_valid_change(input, &node, direction) {
                    acc + 1
                } else {
                    acc
                }
            });

            if valid_edges == 4 {
                None
            } else {
                Some(node.clone())
            }
        })
        .collect();

    // let left_node = edge_nodes
    //     .iter()
    //     .find(|node| {
    //         if let Some(_) = is_valid_change(input, *node, &Direction::Left) {
    //             false
    //         } else {
    //             true
    //         }
    //     })
    //     .unwrap()
    //     .clone();

    let left_node = edge_nodes.iter().reduce(|acc, node| {
        if node.x < acc.x {
            node
        } else {
            acc
        }
    }).unwrap();

    let start_node = RegionNode {
        x: left_node.x - 1,
        y: left_node.y,
        c: left_node.c,
    };

    find_all_edges(
        input,
        &edge_nodes,
        &start_node,
    )
}

fn find_all_edges(
    input: &[Vec<char>],
    region: &HashSet<RegionNode>,
    current_node: &RegionNode,
) -> usize {
    let mut end_position = current_node.clone();
    let mut current_position = current_node.clone();
    let mut current_direction = Direction::Up;
    let mut facing_direction = Direction::Right;
    let mut edges = 0;
    let mut visited: HashSet<RegionNode> = HashSet::new();

    loop {
        println!("{:?} {:?} {:?}", current_position, facing_direction, current_direction);
        if is_valid_edge_change(input, &current_position, &current_direction, region).is_some() && is_valid_edge_change(input, &current_position, &facing_direction, region).is_some() {
            let facing_change = direction_to_coordinates(&facing_direction);
            let facing_node = RegionNode {
                x: current_position.x + facing_change[0],
                y: current_position.y + facing_change[1],
                c: current_position.c,
            };
            visited.insert(facing_node);

            facing_direction = get_prev_direction(&facing_direction);
            current_direction = get_prev_direction(&current_direction);
            edges += 1;


            let facing_change = direction_to_coordinates(&facing_direction);
            let facing_node = RegionNode {
                x: current_position.x + facing_change[0],
                y: current_position.y + facing_change[1],
                c: current_position.c,
            };
            visited.insert(facing_node);
        }

        if let Some(_) = is_valid_edge_change(input, &current_position, &facing_direction, region) {
            let facing_change = direction_to_coordinates(&facing_direction);
            let facing_node = RegionNode {
                x: current_position.x + facing_change[0],
                y: current_position.y + facing_change[1],
                c: current_position.c,
            };
            visited.insert(facing_node);

            if is_valid_edge_change(input, &current_position, &current_direction, region).is_some() {
                continue;
            }

            let change = direction_to_coordinates(&current_direction);
            current_position = RegionNode {
                x: current_position.x + change[0],
                y: current_position.y + change[1],
                c: current_position.c,
            };
        } else {
            let change = direction_to_coordinates(&facing_direction);
            current_position = RegionNode {
                x: current_position.x + change[0],
                y: current_position.y + change[1],
                c: current_position.c,
            };

            facing_direction = get_next_direction(&facing_direction);
            current_direction = get_next_direction(&current_direction);
            edges += 1;
        }

        if current_position == end_position {
            break;
        }
    }

    if visited.len() > region.len() {
        println!("{:?}", visited);
    }
    while visited.len() < region.len() {
        println!("Hitting here?");
        println!("{} {}", visited.len(), region.len());
        let unvisited_node: RegionNode = region.iter().fold(None, |acc, node| {
            if visited.contains(node) {
                return acc;
            }

            return Some(node)
        }).unwrap().clone();

        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        facing_direction = directions
            .iter()
            .find(|direction| {
                if let Some(_) = is_valid_change(input, &unvisited_node, &direction) {
                    false
                } else {
                    true
                }
            })
            .unwrap()
            .clone();

        visited.insert(unvisited_node.clone());
        current_position = unvisited_node.clone();

        let to_find = (current_position.clone(), facing_direction.clone());
        current_direction =
            get_prev_direction(&facing_direction);

        loop {
            if is_valid_change(input, &current_position, &facing_direction).is_none() && is_valid_change(input, &current_position, &current_direction).is_none() {
                visited.insert(current_position.clone());
                facing_direction = get_prev_direction(&facing_direction);
                current_direction = get_prev_direction(&current_direction);

                edges += 1;
            }


            if (current_position.clone(), facing_direction.clone()) == to_find {
                break;
            }

            if let Some(_) =
                is_valid_change(input, &current_position, &facing_direction)
            {
                let change = direction_to_coordinates(&facing_direction);

                current_position = RegionNode {
                    x: current_position.x + change[0],
                    y: current_position.y + change[1],
                    c: current_position.c,
                };

                visited.insert(current_position.clone());
                facing_direction = get_next_direction(&facing_direction);
                current_direction = get_next_direction(&current_direction);
                edges += 1;
            } else {
                visited.insert(current_position.clone());

                let change = direction_to_coordinates(&current_direction);
                current_position = RegionNode {
                    x: current_position.x + change[0],
                    y: current_position.y + change[1],
                    c: current_position.c,
                };
            }

            if (current_position.clone(), facing_direction.clone()) == to_find {
                break;
            }
        }
    }

    edges
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

fn is_valid_edge_change(
    input: &[Vec<char>],
    node: &RegionNode,
    direction: &Direction,
    region: &HashSet<RegionNode>,
) -> Option<[i32; 2]> {
    let change = direction_to_coordinates(direction);
    let test_node = RegionNode {
        x: node.x + change[0],
        y: node.y + change[1],
        c: node.c,
    };

    if region.contains(&test_node) {
        Some(change)
    } else {
        None
    }
}

fn get_next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

fn get_prev_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
    }
}

use itertools::structs::Combinations;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct XY {
    x: i32,
    y: i32,
}

pub fn solve_part_two(input: &[String]) -> usize {
    let char_array: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let antennas: HashMap<char, Vec<XY>> =
        char_array
            .iter()
            .enumerate()
            .fold(HashMap::new(), |acc, (y, line)| {
                line.iter().enumerate().fold(acc, |mut acc, (x, c)| {
                    if *c == '.' {
                        return acc;
                    }

                    acc.entry(*c).or_insert(vec![]).push(XY {
                        x: x as i32,
                        y: y as i32,
                    });
                    acc
                })
            });

    let antinodes: HashSet<XY> = antennas.iter().fold(HashSet::new(), |mut acc, (c, line)| {
        line.iter().combinations(2).for_each(|comb| {
            let directions = get_directions(comb[0], comb[1]);
            get_antinodes(input, comb[0], comb[1], &directions.0, Vec::new())
                .into_iter()
                .for_each(|antinode| {
                    acc.insert(antinode);
                });

            get_antinodes(input, comb[1], comb[0], &directions.1, Vec::new())
                .into_iter()
                .for_each(|antinode| {
                    acc.insert(antinode);
                });
        });

        acc
    });

    antinodes.len()
}

fn get_directions(a: &XY, b: &XY) -> (Direction, Direction) {
    if a.x == b.x {
        if a.y < b.y {
            (Direction::Up, Direction::Down)
        } else {
            (Direction::Down, Direction::Up)
        }
    } else if a.y == b.y {
        if a.x < b.x {
            (Direction::Left, Direction::Right)
        } else {
            (Direction::Right, Direction::Left)
        }
    } else if a.x < b.x {
        if a.y < b.y {
            (Direction::UpLeft, Direction::DownRight)
        } else {
            (Direction::DownLeft, Direction::UpRight)
        }
    } else {
        if a.y < b.y {
            (Direction::UpRight, Direction::DownLeft)
        } else {
            (Direction::DownRight, Direction::UpLeft)
        }
    }
}

fn get_change(direction: &Direction) -> [i32; 2] {
    match direction {
        Direction::Up => [0, -1],
        Direction::Down => [0, 1],
        Direction::Left => [-1, 0],
        Direction::Right => [1, 0],
        Direction::UpLeft => [-1, -1],
        Direction::DownLeft => [-1, 1],
        Direction::UpRight => [1, -1],
        Direction::DownRight => [1, 1],
    }
}

fn get_antinodes(
    input: &[String],
    a: &XY,
    b: &XY,
    direction: &Direction,
    mut antinodes: Vec<XY>,
) -> Vec<XY> {
    antinodes.push(a.clone());
    antinodes.push(b.clone());
    let x_difference = (a.x - b.x).abs();
    let y_difference = (a.y - b.y).abs();
    let change = get_change(direction);
    let x_change = change[0] * x_difference;
    let y_change = change[1] * y_difference;
    if a.x + x_change < 0 || a.x + x_difference >= input[0].len() as i32 {
        antinodes
    } else if a.y + y_change < 0 || a.y + y_difference >= input.len() as i32 {
        antinodes
    } else {
        get_antinodes(
            input,
            &XY {
                x: a.x + x_change,
                y: a.y + y_change,
            },
            a,
            direction,
            antinodes,
        )
    }
}

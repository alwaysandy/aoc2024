use regex::Regex;
use parse_file::parse_file;
use utils::*;

mod part_one;
mod part_two;

#[derive(Debug)]
struct Robot {
    position: XY,
    velocity: XY,
}

fn main() {
    let input = parse_file("input.txt");
    let robots: Vec<Robot> = input
        .iter()
        .map(|line| {
            let n_re = Regex::new(r"-?\d+").unwrap();
            let nums: Vec<i32> = n_re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();

            Robot {
                position: XY {
                    x: nums[0],
                    y: nums[1]
                },
                velocity: XY {
                    x: nums[2],
                    y: nums[3],
                }
            }
        })
        .collect();

    println!("{}", part_one::solve_part_one(&robots));
    part_two::solve_part_two(&robots);

    // println!("{:?}", robots)
}

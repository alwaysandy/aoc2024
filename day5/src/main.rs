mod part_one;
mod part_two;

use parse_file::parse_file;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_file("input");
    println!("{:?}", part_one::solve_part_one(&input));
    println!("{:?}", part_two::solve_part_two(&input));
}
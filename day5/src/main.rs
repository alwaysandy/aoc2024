mod part_one;

use parse_file::parse_file;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_file("input");
    println!("{:?}", part_one::solve_part_one(&input));
}
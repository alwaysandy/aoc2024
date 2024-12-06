use parse_file::parse_file;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    let input = parse_file("input_pt1");

    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let mut left_nums: Vec<isize> = Vec::new();
    let mut right_nums: Vec<isize> = Vec::new();
    for s in input {
        let mut split = s.split_ascii_whitespace();
        let left_num = split.next().unwrap().parse::<isize>().unwrap();
        left_nums.push(left_num);
        let right_num = split.next().unwrap().parse::<isize>().unwrap();
        right_nums.push(right_num);
    }

    let mut distance: usize = 0;
    left_nums.sort();
    right_nums.sort();
    for i in 0..left_nums.len() {
        distance += (left_nums[i] - right_nums[i]).abs() as usize;
    }

    distance
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let mut left_nums: HashSet<usize> = HashSet::new();
    let mut right_nums: HashMap<usize, usize> = HashMap::new();
    for s in input {
        let mut split = s.split_ascii_whitespace();
        let left_num = split.next().unwrap().parse::<usize>().unwrap();
        left_nums.insert(left_num);
        let right_num = split.next().unwrap().parse::<usize>().unwrap();
        if right_nums.contains_key(&right_num) {
            right_nums.insert(right_num, right_nums[&right_num] + 1);
        } else {
            right_nums.insert(right_num, 1);
        }
    }

    let mut similarity: usize = 0;
    for num in left_nums {
        if !right_nums.contains_key(&num) {
            continue
        }

        similarity += num * right_nums[&num];
    }

    similarity
}

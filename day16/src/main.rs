mod part_one;
mod part_two;

use parse_file::parse_file;

fn main() {
    let input: Vec<Vec<char>> = parse_file("input.txt")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    part_one::solve(&input);
    part_two::solve(&input);
}

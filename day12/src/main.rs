use parse_file::parse_file;

mod part_one;
mod part_two;

fn main() {
    let input: Vec<Vec<char>> = parse_file("input.txt")
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("{}", part_one::solve_part_one(&input));
    println!("{}", part_two::solve_part_two(&input));
}

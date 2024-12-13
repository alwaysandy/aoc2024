mod part_two;
mod part_one;

use parse_file::parse_file;

fn main() {
    let input = parse_file("input.txt");
    println!("{}", part_one::solve_part_one(&input));
    println!("{}", part_two::solve_part_two(&input));
}

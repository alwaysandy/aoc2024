mod part_one;
mod part_two;

use parse_file::parse_file;

fn main() {
    let input = parse_file("input.txt");
    // part_one::solve(&input);
    part_two::solve(&input);
}

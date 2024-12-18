mod part_one;
mod part_two;

use parse_file::get_full_file;

fn main() {
    let input = get_full_file("input.txt");
    part_one::solve(&input);
    part_two::solve(&input);
}

mod day_one;
mod day_two;

use parse_file::parse_file;

fn main() {
    let input = parse_file("input");
    println!("{}", day_one::solve_part_one(&input));
    println!("{}", day_two::solve_part_two(&input));
}

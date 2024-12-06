use parse_file::parse_file;
mod part_one;
mod part_two;

fn main() {
    let input = parse_file("input");
    println!("{:?}", part_one::solve_part_one(&input));
}

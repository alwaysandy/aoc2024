use parse_file::get_full_file;

mod part_one;
mod part_two;

fn main() {
    let input = get_full_file("input.txt");
    let input_nums: Vec<usize> = input
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{}", part_one::solve_part_one(&input_nums));
    println!("{}", part_two::solve_part_two(&input_nums));
}

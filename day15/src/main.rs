mod part_one;
mod part_two;

use parse_file::parse_file;

fn main() {
    let input = parse_file("input.txt");
    let pos = input.iter().position(|s| s.is_empty()).unwrap();

    let floor: Vec<Vec<char>> = (0..pos).map(|i| input[i].chars().collect()).collect();
    let moves: Vec<char> = (pos + 1..input.len())
        .flat_map(|i| input[i].chars().collect::<Vec<char>>())
        .collect();

    println!("{}", part_one::solve(&floor, &moves));
    println!("{}", part_two::solve(&floor, &moves));
}

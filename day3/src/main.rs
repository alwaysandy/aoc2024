use parse_file::get_full_file;
use regex::Regex;

fn main() {
    let input = get_full_file("input");
    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &str) -> usize {
    find_mul_pairs(input)
        .iter()
        .map(|expr| calculate_mul_pairs(*expr))
        .sum::<usize>()
}

fn find_mul_pairs(expr: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(expr).map(|m| m.as_str()).collect::<Vec<_>>()
}

fn calculate_mul_pairs(expr: &str) -> usize {
    let nre = Regex::new(r"\d+").unwrap();
    nre.find_iter(expr)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .product::<usize>()
}

fn solve_part_two(input: &str) -> usize {
    input
        .split("don't()")
        .enumerate()
        .flat_map(|(i, expr)| {
            if i == 0 {
                return find_mul_pairs(expr);
            }

            let do_split = expr.split_once("do()");
            return if let Some(expr) = do_split {
                find_mul_pairs(expr.1)
            } else {
                Vec::new()
            };
        })
        .map(calculate_mul_pairs)
        .sum::<usize>()
}

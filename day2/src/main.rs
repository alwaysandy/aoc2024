use parse_file::parse_file;

fn main() {
    let input = parse_file("input.txt");
    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .filter(|line| check_report_safety(line))
        .count()
}

fn check_report_safety(input: &str) -> bool {
    let nums: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let growing = nums[0] < nums[1];
    nums.windows(2).all(|n| meets_condition(n, growing))
}

fn solve_part_two(input: &[String]) -> usize {
    input
        .iter()
        .filter(|line| check_report_safety_pt2(line))
        .count()
}

fn check_report_safety_pt2(input: &str) -> bool {
    let nums: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut test_cases: Vec<Vec<usize>> = Vec::new();
    for i in 0..nums.len() {
        test_cases.push(
            nums.iter()
                .enumerate()
                .filter_map(|(j, n)| if i == j { None } else { Some(*n) })
                .collect(),
        );
    }

    test_cases.iter().any(|test_case| {
        if test_case[0] == test_case[1] {
            return false;
        }

        let growing = test_case[0] < test_case[1];
        test_case
            .windows(2)
            .all(|window| meets_condition(window, growing))
    })
}

fn meets_condition(n: &[usize], growing: bool) -> bool {
    if n[0] == n[1] {
        return false;
    }

    if growing {
        n[0] < n[1] && n[1] - n[0] > 0 && n[1] - n[0] <= 3
    } else {
        n[0] > n[1] && n[0] - n[1] > 0 && n[0] - n[1] <= 3
    }
}

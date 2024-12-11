pub fn solve_part_one(input: &[usize]) -> usize {
    blink(input.to_vec(), 25).len()
}

fn blink(input: Vec<usize>, n: usize) -> Vec<usize> {
    if n == 0 {
        return input;
    }

    blink(
    input.iter().flat_map(|n| {
        if *n == 0 {
            vec![1]
        } else {
            let power: u32 = n.checked_ilog10().unwrap() + 1;
            if power == 0  || power % 2 != 0 {
                vec![n * 2024]
            } else {
                split_digit(n, power / 2)
            }
        }
    }).collect::<Vec<usize>>(), n - 1)
}

fn split_digit(n: &usize, power: u32) -> Vec<usize> {
    vec![
        n / 10_i32.pow(power) as usize,
        n % 10_i32.pow(power) as usize,
    ]
}
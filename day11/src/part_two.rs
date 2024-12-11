use std::collections::HashMap;

pub fn solve_part_two(input: &[usize]) -> usize {
    let mut memoizer: HashMap<(usize, usize), usize> = HashMap::new();
    input.iter().fold(0, |acc ,n| {
        acc + blink(*n, 250, &mut memoizer)
    })
}

fn blink(input: usize, n: usize, memoizer: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if input == 0 {
        blink(1, n-1, memoizer)
    } else {
        if memoizer.contains_key(&(input, n)) {
            return memoizer[&(input, n)];
        }

        let power: u32 = input.ilog10() + 1;
        if power == 0  || power % 2 != 0 {
            let power_return = blink(input * 2024, n - 1, memoizer);
            memoizer.insert((input, n), power_return);
            power_return
        } else {
            let split_return = split_digit(&input, power / 2, n, memoizer);
            memoizer.insert((input, n), split_return);
            split_return
        }
    }
}

fn split_digit(input: &usize, power: u32, n: usize, memoizer: &mut HashMap<(usize, usize), usize>) -> usize {
    let input_div = blink(input / 10_i32.pow(power) as usize, n - 1, memoizer);
    let input_mod = blink(input % 10_i32.pow(power) as usize, n - 1, memoizer);
    input_div + input_mod
}
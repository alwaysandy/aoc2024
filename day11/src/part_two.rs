use std::collections::HashMap;

pub fn solve_part_two(input: &[usize]) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .iter()
        .fold(0, |acc, n| acc + blink(*n, 75, &mut cache))
}

fn blink(input: usize, n: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if input == 0 {
        blink(1, n - 1, cache)
    } else {
        if let Some(v) = cache.get(&(input, n)) {
            return *v;
        }

        let input_len: u32 = input.ilog10() + 1;
        if input_len == 0 || input_len % 2 != 0 {
            let power_return = blink(input * 2024, n - 1, cache);
            cache.insert((input, n), power_return);
            power_return
        } else {
            let split_return = split_digit(&input, input_len / 2, n, cache);
            cache.insert((input, n), split_return);
            split_return
        }
    }
}

fn split_digit(
    input: &usize,
    power: u32,
    n: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let input_div = blink(input / 10_i32.pow(power) as usize, n - 1, cache);
    let input_mod = blink(input % 10_i32.pow(power) as usize, n - 1, cache);
    input_div + input_mod
}

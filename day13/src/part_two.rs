use regex::Regex;

#[derive(Debug)]
struct XY {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Machine {
    a: XY,
    b: XY,
    prize: XY
}



pub fn solve_part_two(input: &[String]) -> u64 {
    let input_machines: Vec<Machine> = input.windows(3).step_by(4).map(|lines| {
        let n_re = Regex::new(r"\d+").unwrap();
        let a: Vec<u64> = n_re.find_iter(&lines[0]).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
        let b: Vec<u64> = n_re.find_iter(&lines[1]).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
        let prize: Vec<u64> = n_re.find_iter(&lines[2]).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
        Machine {
            a: XY {x: a[0], y: a[1]},
            b: XY {x: b[0], y: b[1]},
            prize: XY {x: prize[0] + 10000000000000, y: prize[1] + 10000000000000}
        }
    }).collect();

    input_machines.iter().fold(0, |acc, machine| {
        if let Some(tokens) = system_of_equations(machine) {
            acc + tokens
        } else {
            acc
        }
    })
}

fn find_smallest_token(machine: &Machine) -> Option<usize> {
    let max_a_presses = find_max_presses(machine.a.x, machine.a.y, &machine.prize) as usize;
    let max_b_presses = find_max_presses(machine.b.x, machine.b.y, &machine.prize) as usize;
    let combos: Vec<[usize; 2]> = (0..=max_a_presses).filter_map(|presses|{
        let current_x = machine.a.x as usize * presses;
        let current_y = machine.a.y as usize * presses;

        for m in 0..=max_b_presses {
            if current_x + (machine.b.x as usize * m) > machine.prize.x as usize{
                return None
            } else if current_y + (machine.b.y as usize * m) > machine.prize.y as usize {
                return None
            } else if current_x + (machine.b.x as usize * m) == machine.prize.x as usize && current_y + (machine.b.y as usize * m) == machine.prize.y as usize {
                return Some([presses, m])
            } else {
                continue;
            }
        }
        None
    }).collect();

    combos.iter().map(|presses| {
        (presses[0] * 3 + presses[1])
    }).min()
}

fn find_max_presses(x: u64, y: u64, prize: &XY) -> u64 {
    if prize.x / x < prize.y / y {
        prize.x / x
    } else {
        prize.y / y
    }
}

fn find_min_presses(x: u64, y: u64, prize: &XY) -> u64 {
    if prize.x / x < prize.y / y {
        prize.y / y
    } else {
        prize.x / x
    }
}

fn better_to_press(machine: &Machine) -> bool {
    find_max_presses(machine.a.x, machine.a.y, &machine.prize) < find_max_presses(machine.b.x, machine.b.y, &machine.prize) * 3
}

fn is_divisble(a: u64, b: u64, n: u64) -> bool {
    for i in 0..=b {
        if (n - (i * a)) % b == 0 {
            return true;
        }
    }

    false
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    (a * b) / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

fn system_of_equations(machine: &Machine) -> Option<(u64)>{
    let left_top =  machine.a.x * machine.a.y;
    let right_top = machine.b.x * machine.a.y;
    let prize_top = machine.prize.x * machine.a.y;

    let left_bottom = machine.a.y * machine.a.x;
    let right_bottom = machine.b.y * machine.a.x;
    let prize_bottom = machine.prize.y * machine.a.x;

    if right_top > right_bottom {
        let sub_right = right_top - right_bottom;
        if prize_top < prize_bottom {
            return None
        }
        let sub_prize = prize_top - prize_bottom;
        let a = sub_prize / sub_right;
        if prize_top < (right_top * a) {
            return None;
        }
        let b = (prize_top - (right_top * a)) / left_top;

        if (b * machine.a.x) + (a * machine.b.x) != machine.prize.x || (b * machine.a.y) + (a * machine.b.y) != machine.prize.y {
            return None;
        }
        Some((b * 3) + a)
    } else if right_bottom > right_top {
        let sub_right = right_bottom - right_top;
        if prize_bottom < prize_top {
            return None
        }
        let sub_prize = prize_bottom - prize_top;
        let a = sub_prize / sub_right;
        if prize_top < (right_top * a) {
            return None;
        }
        let b = (prize_top - (right_top * a)) / left_top;

        if (b * machine.a.x) + (a * machine.b.x) != machine.prize.x || (b * machine.a.y) + (a * machine.b.y) != machine.prize.y {
            return None;
        }
        Some((b * 3) + a)
    } else {
        None
    }
}
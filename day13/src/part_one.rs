use regex::Regex;

#[derive(Debug)]
struct XY {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Machine {
    a: XY,
    b: XY,
    prize: XY
}

pub fn solve_part_one(input: &[String]) -> usize {
    let input_machines: Vec<Machine> = input.windows(3).step_by(4).map(|lines| {
        let n_re = Regex::new(r"\d+").unwrap();
        let a: Vec<i32> = n_re.find_iter(&lines[0]).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let b: Vec<i32> = n_re.find_iter(&lines[1]).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let prize: Vec<i32> = n_re.find_iter(&lines[2]).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        Machine {
            a: XY {x: a[0], y: a[1]},
            b: XY {x: b[0], y: b[1]},
            prize: XY {x: prize[0], y: prize[1]}
        }
    }).collect();

    input_machines.iter().fold(0, |acc, machine| {
        if let Some(tokens) = find_smallest_token(machine) {
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

fn find_max_presses(x: i32, y: i32, prize: &XY) -> i32 {
    if prize.x / x < prize.y / y {
        prize.x / x
    } else {
        prize.y / y
    }
}
//
// fn least_common_multiple(a: i32, b: i32) -> i32 {
//     (a * b) / greatest_common_divisor(a, b)
// }
//
// fn greatest_common_divisor(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         a
//     } else {
//         greatest_common_divisor(b, a % b)
//     }
// }
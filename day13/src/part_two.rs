use regex::Regex;

#[derive(Debug)]
struct XY {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Machine {
    a: XY,
    b: XY,
    prize: XY,
}

pub fn solve_part_two(input: &[String]) -> u64 {
    let input_machines: Vec<Machine> = input
        .windows(3)
        .step_by(4)
        .map(|lines| {
            let n_re = Regex::new(r"\d+").unwrap();
            let a: Vec<u64> = n_re
                .find_iter(&lines[0])
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect();
            let b: Vec<u64> = n_re
                .find_iter(&lines[1])
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect();
            let prize: Vec<u64> = n_re
                .find_iter(&lines[2])
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect();
            Machine {
                a: XY { x: a[0], y: a[1] },
                b: XY { x: b[0], y: b[1] },
                prize: XY {
                    x: prize[0] + 10000000000000,
                    y: prize[1] + 10000000000000,
                },
            }
        })
        .collect();

    input_machines.iter().fold(0, |acc, machine| {
        if let Some(tokens) = system_of_equations(machine) {
            acc + tokens
        } else {
            acc
        }
    })
}

fn system_of_equations(machine: &Machine) -> Option<(u64)> {
    let left_top = machine.a.x * machine.a.y;
    let right_top = machine.b.x * machine.a.y;
    let prize_top = machine.prize.x * machine.a.y;

    let left_bottom = machine.a.y * machine.a.x;
    let right_bottom = machine.b.y * machine.a.x;
    let prize_bottom = machine.prize.y * machine.a.x;

    let sub_prize;
    let sub_right;
    if right_top > right_bottom {
        sub_right = right_top - right_bottom;
        if prize_top < prize_bottom {
            return None;
        }
        sub_prize = prize_top - prize_bottom;
    } else if right_bottom > right_top {
        sub_right = right_bottom - right_top;
        if prize_bottom < prize_top {
            return None;
        }
        sub_prize = prize_bottom - prize_top;
    } else {
        return None;
    }

    let b = sub_prize / sub_right;
    if prize_top < (right_top * b) {
        return None;
    }
    let a = (prize_top - (right_top * b)) / left_top;
    if (a * machine.a.x) + (b * machine.b.x) != machine.prize.x
        || (a * machine.a.y) + (b * machine.b.y) != machine.prize.y
    {
        return None;
    }

    Some((a * 3) + b)
}

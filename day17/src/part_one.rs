use regex::Regex;

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug)]
struct Program {
    ins: Vec<i64>,
    ins_ptr: usize,
}

pub fn solve(input: &String) {
    let n_re = Regex::new(r"\d+").unwrap();
    let mut reg: Registers = Registers { a: 0, b: 0, c: 0 };

    let mut program: Program = Program {
        ins: Vec::new(),
        ins_ptr: 0,
    };

    let mut out_nums: Vec<i64> = Vec::new();

    n_re.find_iter(input)
        .enumerate()
        .map(|(n, m)| (n, m.as_str().to_string().parse::<i64>().unwrap()))
        .for_each(|(n, m)| match n {
            0 => reg.a = m,
            1 => reg.b = m,
            2 => reg.c = m,
            _ => program.ins.push(m),
        });

    while program.ins_ptr < program.ins.len() {
        match program.ins[program.ins_ptr] {
            0 => {
                let calc = adv(&reg, program.ins[program.ins_ptr + 1]);
                reg.a = calc;
                program.ins_ptr += 2;
            }
            1 => {
                let calc = bxl(&reg, program.ins[program.ins_ptr + 1]);
                reg.b = calc;
                program.ins_ptr += 2;
            }
            2 => {
                let calc = bst(&reg, program.ins[program.ins_ptr + 1]);
                reg.b = calc;
                program.ins_ptr += 2;
            }
            3 => {
                if let Some(calc) = jnz(&reg, program.ins[program.ins_ptr + 1]) {
                    program.ins_ptr = calc as usize;
                } else {
                    program.ins_ptr += 2;
                }
            }
            4 => {
                let calc = bxc(&reg, program.ins[program.ins_ptr + 1]);
                reg.b = calc;
                program.ins_ptr += 2;
            }
            5 => {
                let calc = out(&reg, program.ins[program.ins_ptr + 1]);
                out_nums.push(calc);
                program.ins_ptr += 2;
            }
            6 => {
                let calc = bdv(&reg, program.ins[program.ins_ptr + 1]);
                reg.b = calc;
                program.ins_ptr += 2;
            }
            7 => {
                let calc = cdv(&reg, program.ins[program.ins_ptr + 1]);
                reg.c = calc;
                program.ins_ptr += 2;
            }
            _ => unreachable!(),
        }
    }

    println!(
        "{:?\
    }",
        out_nums
    );
}

fn adv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a / 2_i64.pow(operand as u32),
        4 => reg.a / 2_i64.pow(reg.a as u32),
        5 => reg.a / 2_i64.pow(reg.b as u32),
        6 => reg.a / 2_i64.pow(reg.c as u32),
        _ => panic!("Panic in adv: combo operand of: {} given", operand),
    }
}

fn bxl(reg: &Registers, operand: i64) -> i64 {
    reg.b ^ operand
}

fn bst(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => operand % 8,
        4 => reg.a % 8,
        5 => reg.b % 8,
        6 => reg.c % 8,
        _ => panic!("Panic in bst: combo operand of: {} given", operand),
    }
}

fn jnz(reg: &Registers, operand: i64) -> Option<i64> {
    if reg.a == 0 {
        None
    } else {
        Some(operand)
    }
}

fn bxc(reg: &Registers, operand: i64) -> i64 {
    reg.b ^ reg.c
}

fn out(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => operand % 8,
        4 => reg.a % 8,
        5 => reg.b % 8,
        6 => reg.c % 8,
        _ => panic!("Panic in out: combo operand of: {} given", operand),
    }
}

fn bdv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a / 2_i64.pow(operand as u32),
        4 => reg.a / 2_i64.pow(reg.a as u32),
        5 => reg.a / 2_i64.pow(reg.b as u32),
        6 => reg.a / 2_i64.pow(reg.c as u32),
        _ => panic!("Panic in bdv: combo operand of: {} given", operand),
    }
}

fn cdv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a / 2_i64.pow(operand as u32),
        4 => reg.a / 2_i64.pow(reg.a as u32),
        5 => reg.a / 2_i64.pow(reg.b as u32),
        6 => reg.a / 2_i64.pow(reg.c as u32),
        _ => panic!("Panic in cdv: combo operand of: {} given", operand),
    }
}

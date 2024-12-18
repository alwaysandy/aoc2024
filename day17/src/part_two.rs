use regex::Regex;

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Clone, Debug)]
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

    n_re.find_iter(input)
        .enumerate()
        .map(|(n, m)| (n, m.as_str().to_string().parse::<i64>().unwrap()))
        .for_each(|(n, m)| match n {
            0 | 1 | 2 => (),
            _ => program.ins.push(m),
        });

    let mut n = 0;
    let mut ith = program.ins.len() - 1;
    // let mut n = 34632;
    loop {
        reg.a = n;
        reg.b = 0;
        reg.c = 0;
        program.ins_ptr = 0;

        let out_nums = run_program(&mut reg, &mut program);
        if out_nums
            .iter()
            .enumerate()
            .all(|(i, n)| out_nums[i] == program.ins[ith + i])
        {
            if ith == 0 {
                break;
            }
            n <<= 3;
            ith -= 1;
        } else {
            n += 1;
        }
    }

    println!("{}", n);
}

fn run_program(reg: &mut Registers, program: &mut Program) -> Vec<i64> {
    let mut out_nums: Vec<i64> = Vec::new();
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

    out_nums
}

fn adv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a >> operand,
        4 => reg.a >> reg.a,
        5 => reg.a >> reg.b,
        6 => reg.a >> reg.c,
        _ => panic!("Panic in adv: combo operand of: {} given", operand),
    }
}

fn bxl(reg: &Registers, operand: i64) -> i64 {
    reg.b ^ operand
}

fn bst(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => operand & 7,
        4 => reg.a & 7,
        5 => reg.b & 7,
        6 => reg.c & 7,
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
        0 | 1 | 2 | 3 => operand & 7,
        4 => reg.a & 7,
        5 => reg.b & 7,
        6 => reg.c & 7,
        _ => panic!("Panic in out: combo operand of: {} given", operand),
    }
}

fn bdv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a >> operand,
        4 => reg.a >> reg.a,
        5 => reg.a >> reg.b,
        6 => reg.a >> reg.c,
        _ => panic!("Panic in bdv: combo operand of: {} given", operand),
    }
}

fn cdv(reg: &Registers, operand: i64) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => reg.a >> operand,
        4 => reg.a >> reg.a,
        5 => reg.a >> reg.b,
        6 => reg.a >> reg.c,
        _ => panic!("Panic in cdv: combo operand of: {} given", operand),
    }
}

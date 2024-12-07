pub fn solve_part_one(input: &[String]) -> usize {
    input.iter().fold(0, |acc, line| {
        let final_val = line.split(":").next().unwrap().parse::<usize>().unwrap();
        let operands: Vec<usize> = line.split(":").last().unwrap().trim().split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let valid = is_valid(&operands, 0, operands[0], final_val);
        if valid {
            acc + final_val
        } else {
            acc
        }
    })
}

fn is_valid(operands: &Vec<usize>, current_index: usize, current_val: usize, final_val: usize) -> bool {
    if current_val == final_val && current_index == operands.len() - 1 {
        return true;
    }

    if current_index == operands.len() - 1 {
        return false;
    }

    if is_valid(operands, current_index + 1, current_val + operands[current_index + 1], final_val) {
        return true;
    }

    is_valid(operands, current_index + 1, current_val * operands[current_index + 1], final_val)
}
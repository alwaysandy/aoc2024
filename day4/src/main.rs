use parse_file::parse_file;

fn main() {
    let input = parse_file("input");
    println!("{}", solve_part_one(&input));
}

fn solve_part_one(input: &[String]) -> usize {
    let input_arrays: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let directions: [[i32; 2]; 8] = [
        [0, 1],
        [1, 0],
        [1, 1],
        [-1, 1],
        [0, -1],
        [-1, -1],
        [-1, 0],
        [1, -1]
    ];

    input_arrays.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(acc, |acc, (x, n)| {
            if input_arrays[y][x] != 'X' {
                return acc;
            }

            directions.iter().fold(acc, |acc, direction| {
                if check_direction(&input_arrays, direction, x, y) {
                    acc + 1
                } else {
                    acc
                }
            })
        })
    })
}

fn check_direction(input: &Vec<Vec<char>>, direction: &[i32; 2], start_x: usize, start_y: usize) -> bool {
    // Who needs a for loop
    let chars = ['X', 'M', 'A', 'S'];
    (1..=3).all(|i| {
        let new_x = direction[0] * i + start_x as i32;
        if new_x < 0 || new_x >= input.len() as i32 {
            return false;
        }

        let new_y = direction[1] * i + start_y as i32;
        if new_y < 0 || new_y >= input.len() as i32 {
            return false;
        }

        input[new_y as usize][new_x as usize] == chars[i as usize]
    })
}
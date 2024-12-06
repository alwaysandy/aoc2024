pub fn solve_part_two(input: &[String]) -> usize {
    let input_arrays: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    input_arrays.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(acc, |acc, (x, n)| {
            if *n != 'A' {
                return acc;
            }

            if check_x(&input_arrays, x, y) {
                acc + 1
            } else {
                acc
            }
        })
    })
}

fn check_x(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || x == input.len() - 1 {
        return false;
    }
    if y == 0 || y == input.len() - 1 {
        return false;
    }

    let directions: [[isize; 4]; 2] = [[1, -1, -1, 1], [1, 1, -1, -1]];

    directions.iter().all(|direction| {
        let x_one = (x as isize + direction[0]) as usize;
        let x_two = (x as isize + direction[2]) as usize;
        let y_one = (y as isize + direction[1]) as usize;
        let y_two = (y as isize + direction[3]) as usize;
        if input[y_one][x_one] == input[y_two][x_two] {
            return false;
        }

        (input[y_one][x_one] == 'M' || input[y_one][x_one] == 'S')
            && (input[y_two][x_two] == 'M' || input[y_two][x_two] == 'S')
    })
}

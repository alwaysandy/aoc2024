use crate::Robot;
use std::collections::HashMap;
use utils::*;

pub fn solve_part_two(input: &[Robot]) {
    let width = 101;
    let height = 103;
    let largest_quadrant = (0..100000).fold([0, 0], |acc, n| {
        let robots = get_robots(input, width, height, n);
        let largest_quadrant = *count_quadrants(&robots, width, height)
            .iter()
            .max()
            .unwrap();
        if largest_quadrant > acc[0] {
            [largest_quadrant, n]
        } else {
            acc
        }
    });

    display_robots(&get_robots(input, width, height, largest_quadrant[1]));
    println!("{}", largest_quadrant[1]);
}

fn count_quadrants(robots: &HashMap<XY, usize>, width: i32, height: i32) -> [i32; 4] {
    robots.iter().fold([0, 0, 0, 0], |acc, (robot, count)| {
        if robot.x >= 0 && robot.x <= width / 2 - 1 && robot.y >= 0 && robot.y <= height / 2 - 1 {
            [acc[0] + (*count as i32), acc[1], acc[2], acc[3]]
        } else if robot.x >= width / 2 + 1
            && robot.x <= width
            && robot.y >= 0
            && robot.y <= height / 2 - 1
        {
            [acc[0], acc[1] + (*count as i32), acc[2], acc[3]]
        } else if robot.x >= 0
            && robot.x <= width / 2 - 1
            && robot.y >= robot.y / 2 + 1
            && robot.y <= height
        {
            [acc[0], acc[1], acc[2] + *count as i32, acc[3]]
        } else if robot.x >= width / 2 + 1
            && robot.x <= width
            && robot.y >= height / 2 + 1
            && robot.y <= height
        {
            [acc[0], acc[1], acc[2], acc[3] + *count as i32]
        } else {
            acc
        }
    })
}

fn get_robots(input: &[Robot], width: i32, height: i32, seconds: i32) -> HashMap<XY, usize> {
    let mut robots = HashMap::new();
    for robot in input {
        let mut x = (robot.position.x + (robot.velocity.x * seconds)) % width;
        let mut y = (robot.position.y + (robot.velocity.y * seconds)) % height;

        if x < 0 {
            x = width + x;
        }

        if y < 0 {
            y = height + y;
        }

        *robots.entry(XY { x, y }).or_insert(0) += 1;
    }

    robots
}

fn display_robots(robots: &HashMap<XY, usize>) {
    let mut display: [[char; 101]; 103] = [[' '; 101]; 103];
    for (robot, _) in robots {
        display[robot.y as usize][robot.x as usize] = '#';
    }

    for line in display {
        let s = line.iter().collect::<String>();
        println!("{}", s);
    }
}

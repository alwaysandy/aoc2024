use crate::Robot;
use std::collections::HashMap;
use utils::*;

pub fn solve_part_one(input: &[Robot]) -> usize {
    let mut robots: HashMap<XY, usize> = HashMap::new();
    let height = 103;
    let width = 101;
    input.iter().for_each(|robot| {
        let mut x = ((robot.position.x + (robot.velocity.x * 100)) % width);
        let mut y = ((robot.position.y + (robot.velocity.y * 100)) % height);

        if x < 0 {
            x = width + x;
        }

        if y < 0 {
            y = height + y;
        }

        *robots.entry(XY { x, y }).or_insert(0) += 1;
    });

    let total = count_quadrant(&robots, 0, width / 2 - 1, 0, height / 2 - 1)
        * count_quadrant(&robots, width / 2 + 1, width, height / 2 + 1, height)
        * count_quadrant(&robots, 0, width / 2 - 1, height / 2 + 1, height)
        * count_quadrant(&robots, width / 2 + 1, width, 0, height / 2 - 1);

    return total;
}

fn count_quadrant(
    robots: &HashMap<XY, usize>,
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
) -> usize {
    robots.iter().fold(0, |acc, (robot, count)| {
        if robot.x >= start_x && robot.x <= end_x && robot.y >= start_y && robot.y <= end_y {
            acc + count
        } else {
            acc
        }
    })
}

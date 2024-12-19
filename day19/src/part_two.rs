use std::collections::{HashMap, HashSet};

pub fn solve(input: &[String]) {
    let patterns: HashSet<String> = input[0].split(", ").map(|p| p.to_string()).collect();

    let answer = input[2..].iter().fold(0, |acc, s| {
        acc + count_possibilities(s, &patterns, &mut HashMap::new())
    });

    println!("{}", answer);
}

fn count_possibilities(
    towel: &str,
    patterns: &HashSet<String>,
    succeeded: &mut HashMap<(usize, String), usize>,
) -> usize {
    if towel.len() == 0 {
        return 1;
    }

    (1..=8).rev().fold(0, |acc, i| {
        if i > towel.len() {
            return acc;
        }

        if patterns.contains(&towel[0..i]) {
            let tobenamed = &towel[i..];
            if let Some(answer) = succeeded.get(&(i, tobenamed.to_string())) {
                return acc + answer
            }

            let answer = count_possibilities(tobenamed, patterns, succeeded);
            if answer == 0 {
                return acc;
            }

            succeeded.insert((i, tobenamed.to_string()), answer);
            acc + answer
        } else {
            acc
        }
    })
}

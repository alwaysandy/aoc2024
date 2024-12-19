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
    succeeded: &mut HashMap<String, usize>,
) -> usize {
    if towel.len() == 0 {
        return 1;
    }

    if let Some(answer) = succeeded.get(towel) {
        return *answer;
    }

    let answer = (1..=8).rev().fold(0, |acc, i| {
        if i > towel.len() {
            return acc;
        }

        if patterns.contains(&towel[0..i]) {
            acc + count_possibilities(&towel[i..], patterns, succeeded)
        } else {
            acc
        }
    });

    succeeded.insert(towel.to_string(), answer);
    answer
}

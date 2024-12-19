use std::collections::{HashMap, HashSet};

pub fn solve(input: &[String]) -> usize {
    let patterns: HashSet<String> = input[0].split(", ").map(|p| p.to_string()).collect();
    input[2..].iter().fold(0, |acc, s| {
        acc + count_possibilities(s, &patterns, &mut HashMap::new())
    })
}

fn count_possibilities(
    towel: &str,
    patterns: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if towel.len() == 0 {
        return 1;
    }

    if let Some(answer) = cache.get(towel) {
        return *answer;
    }

    let answer = (1..=8).rev().fold(0, |acc, i| {
        if i > towel.len() {
            return acc;
        }

        if patterns.contains(&towel[0..i]) {
            acc + count_possibilities(&towel[i..], patterns, cache)
        } else {
            acc
        }
    });

    cache.insert(towel.to_string(), answer);
    answer
}

use std::collections::{HashMap, HashSet};

pub fn solve(input: &[String]) {
    let patterns: HashSet<String> = input[0].split(", ").map(|p| p.to_string()).collect();

    let answer = input[2..].iter().fold(0, |acc, s| {
        let mut succeeded: HashMap<(usize, String), usize> = HashMap::new();
        acc + count_possibilities(s, &patterns, &mut succeeded)
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

    let mut count = 0;
    let mut i = if towel.len() > 8 { 8 } else { towel.len() };
    loop {
        if i == 0 {
            return count;
        }

        if patterns.contains(&towel[0..i]) {
            let tobenamed = &towel[i..];
            if let Some(answer) = succeeded.get(&(i, tobenamed.to_string())) {
                count += answer;
                i -= 1;
                continue;
            }

            let answer = count_possibilities(&tobenamed, patterns, succeeded);
            if answer == 0 {
                i -= 1;
                continue;
            }

            succeeded.insert((i, tobenamed.to_string()), answer);
            count += answer;
        }

        i -= 1
    }
}

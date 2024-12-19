use std::collections::HashSet;

pub fn solve(input: &[String]) {
    let mut patterns: Vec<Vec<String>> = Vec::new();
    for i in 0..9 {
        patterns.push(Vec::new());
    }

    input[0].split(", ").for_each(|p| {
        patterns[p.len()].push(p.to_string());
    });

    let answer = input[2..].iter().fold(0, |acc, s| {
        let mut failed: HashSet<(usize, String)> = HashSet::new();
        if is_possible(s.to_string(), 0, &patterns, &mut failed) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", answer);
}

fn is_possible(towel: String, pos: usize, patterns: &Vec<Vec<String>>, failed: &mut HashSet<(usize, String)>) -> bool {
    if towel.len() == 0 {
        return true;
    }

    let mut i = if towel.len() > 8 { 8 } else { towel.len() };
    loop {
        if i == 0 {
            return false;
        }

        if patterns[i].iter().any(|p| {
            if let Some(offset) = towel.find(p) {
                if offset != 0 {
                    return false;
                }

                let tobenamed = towel[i..].to_string();
                if failed.contains(&(0, tobenamed.clone())) {
                    return false;
                }

                if !is_possible(tobenamed.clone(), i, patterns, failed) {
                    failed.insert((0, tobenamed.clone()));
                    return false;
                }

                true
            } else {
                false
            }
        }) {
            return true;
        }

        i -= 1
    }
}

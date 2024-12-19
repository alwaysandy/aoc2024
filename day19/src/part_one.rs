use std::collections::HashSet;

pub fn solve(input: &[String]) -> usize {
    let mut patterns: Vec<Vec<String>> = Vec::new();
    for i in 0..9 {
        patterns.push(Vec::new());
    }

    input[0].split(", ").for_each(|p| {
        patterns[p.len()].push(p.to_string());
    });

    input[2..].iter().fold(0, |acc, s| {
        let mut failed: HashSet<String> = HashSet::new();
        if is_possible(s.to_string(), &patterns, &mut failed) {
            acc + 1
        } else {
            acc
        }
    })
}

fn is_possible(towel: String, patterns: &Vec<Vec<String>>, failed: &mut HashSet<String>) -> bool {
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
                let left = towel[0..offset].to_string();
                if failed.contains(&left) {
                    return false;
                }

                let right = towel[offset + i..].to_string();
                if failed.contains(&right) {
                    return false;
                }

                if !is_possible(left.clone(), patterns, failed) {
                    failed.insert(left);
                    return false;
                }

                if !is_possible(right.clone(), patterns, failed) {
                    failed.insert(right);
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

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
        println!("First!");
        let mut failed: HashSet<String> = HashSet::new();
        acc + is_possible(s.to_string(), &patterns, &mut failed)
    });

    println!("{}", answer);
}

fn is_possible(towel: String, patterns: &Vec<Vec<String>>, failed: &mut HashSet<String>) -> usize {
    if towel.len() == 0 {
        return 1;
    }

    let mut count = 0;
    let mut i = if towel.len() > 8 { 8 } else { towel.len() };
    loop {
        if i == 0 {
            return count;
        }

        patterns[i].iter().for_each(|p| {
            if let Some(offset) = towel.find(p) {
                if offset != 0 {
                    return;
                }

                let tobenamed = towel[i..].to_string();
                if failed.contains(&tobenamed) {
                    return;
                }

                let answer= is_possible(tobenamed.clone(), patterns, failed);
                if answer == 0 {
                    failed.insert(tobenamed);
                    return;
                }

                count += answer;
            }
        });

        i -= 1
    }
}

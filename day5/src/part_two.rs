use std::collections::{HashMap, HashSet};

pub fn solve_part_two(input: &[String]) -> usize {
    let rule_split_index = input.iter().position(|s| s.is_empty()).unwrap();
    let rules: HashMap<_, _> = input[0..rule_split_index]
        .iter()
        .map(|s| {
            s.split("|")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut rules, rule| {
                if !rules.contains_key(&rule[1]) {
                    rules.insert(rule[1], HashSet::new());
                }
                rules.get_mut(&rule[1]).unwrap().insert(rule[0]);
                rules
            },
        );

    let pages: Vec<Vec<usize>> = input[rule_split_index + 1..]
        .iter()
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let answer = pages.iter().fold(0, |acc, page| {
        let is_in_order = page.iter().enumerate().all(|(i, n)| {
            page[0..i].iter().all(|o| {
                if let Some(rule) = rules.get(o) {
                    return !rule.contains(n);
                } else {
                    true
                }
            })
        });

        if is_in_order {
            acc
        } else {
            let sorted_page = sort_page(&mut page.clone(), &rules);
            acc + sorted_page[sorted_page.len() / 2]
        }
    });

    answer
}

fn sort_page(page: &mut Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    page.sort_by(|a, b| {
        if let Some(rule) = rules.get(a) {
            if rule.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }

        if let Some(rule) = rules.get(b) {
            if rule.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    });

    page.to_vec()
}

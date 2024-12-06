use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Rule {
    pub first: u32,
    pub second: u32,
}

impl Rule {
    fn is_followed_by(&self, manual: &Manual) -> bool {
        // Scan manual for last of "first", making sure there are no second before first.
        if let Some(pos) = manual.iter().rposition(|x| *x == self.first) {
            // Then check all positions before pos for second. If we find one, rule is not followed
            for i in 0..pos {
                if manual[i] == self.second {
                    return false;
                }
            }
            true
        } else {
            true
        }
    }
}

type Manual = Vec<u32>;

/// Read input as map of coords -> letters at said coords
fn read_input() -> (Vec<Rule>, Vec<Manual>) {
    let file = File::open("inputs/day5.txt").unwrap();
    let mut rules = vec![];
    let mut manuals = vec![];

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.contains("|") {
            let mut it = line.split("|");
            if let (Some(first), Some(second)) = (it.next(), it.next()) {
                rules.push(Rule {
                    first: u32::from_str(first).unwrap(),
                    second: u32::from_str(second).unwrap()
                });
            }
        } else if line.contains(",") {
            let manual = line.split(",")
                .map(|n| u32::from_str(n).unwrap())
                .collect();
            manuals.push(manual);
        }
    }

    (rules, manuals)
}

fn is_manual_valid(manual: &Manual, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !rule.is_followed_by(manual) {
            return false;
        }
    }

    true
}

fn build_sorting_rules(rules: &Vec<Rule>) -> HashMap<(u32, u32), Ordering> {
    let mut out = HashMap::new();
    for rule in rules {
        // Insert both ways: first, second and second, first
        out.insert((rule.first, rule.second), Ordering::Less);
        out.insert((rule.second, rule.first), Ordering::Greater);
    }
    out
}

// Sort manuals by providing a sort function that uses the rules.
// sorting_rules is a dict that provides the ordering for any two numbers in the rules.
fn sort_manual(manual: &mut Manual, sorting_rules: &HashMap<(u32, u32), Ordering>) {
    manual.sort_by(|a, b| {
        sorting_rules.get(&(*a, *b)).copied().unwrap_or(Ordering::Equal)
    })
}

/// Only return manuals which conform to all rules
fn get_valid_manuals(rules: Vec<Rule>, manuals: Vec<Manual>) -> Vec<Manual> {
    manuals.into_iter()
        .filter(|manual| {
            is_manual_valid(manual, &rules)
        })
        .collect()
}

/// Returns all the invalid manuals, but fixed
fn get_invalid_manuals(rules: Vec<Rule>, manuals: Vec<Manual>) -> Vec<Manual> {
    let sorting_rules = build_sorting_rules(&rules);
    manuals.into_iter()
        .filter_map(|mut manual| {
            if !is_manual_valid(&manual, &rules) {
                sort_manual(&mut manual, &sorting_rules);
                Some(manual)
            } else {
                None
            }
        })
        .collect()
}

fn get_value(manuals: Vec<Manual>) -> u32 {
    manuals.into_iter()
        .map(|manual| manual[manual.len() / 2])
        .sum()
}

fn main() {
    let (rules, manuals) = read_input();
    let manuals = get_invalid_manuals(rules, manuals);
    let result = get_value(manuals);
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_rules() -> Vec<Rule> {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        rules.into_iter()
            .map(|(first, second)| Rule { first, second })
            .collect()
    }

    fn get_test_manuals() -> Vec<Manual> {
        vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ]
    }

    #[test]
    fn simple_case() {
        let manuals = get_valid_manuals(get_test_rules(), get_test_manuals());
        assert_eq!(get_value(manuals), 143)
    }

    #[test]
    fn simple_case_part2() {
        let manuals = get_invalid_manuals(get_test_rules(), get_test_manuals());
        assert_eq!(get_value(manuals), 123)
    }
}

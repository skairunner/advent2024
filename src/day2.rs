use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

/// Read input and parse.
fn read_input() -> Vec<Vec<i64>> {
    let file = File::open("inputs/day2.txt").unwrap();

    let mut out = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line = line
            .split(" ")
            .map(|entry| i64::from_str(entry).unwrap())
            .collect();
        out.push(line);
    }

    out
}

fn is_problematic(prev: &i64, next: &i64, increasing: Option<bool>) -> bool {
    let diff = prev.abs_diff(*next);
    if diff < 1 || diff > 3 {
        return true;
    }
    match increasing {
        Some(true) => prev >= next,
        Some(false) => prev <= next,
        None => false,
    }
}

fn is_safe(report: &[i64]) -> bool {
    // Determine whether increasing or decreasing
    let mut increasing: Option<bool> = None;
    let mut prev = report[0];
    for entry in report.iter().skip(1) {
        if is_problematic(&prev, entry, increasing) {
            return false;
        }
        if increasing.is_none() {
            // Determine trend
            increasing = Some(prev < *entry);
        }
        prev = *entry;
    }

    true
}

fn is_safe_tolerant(report: &[i64]) -> bool {
    // First, check if is safe.
    if is_safe(report) {
        return true;
    }
    // Else, try removing an element from each report
    for i in 0..report.len() {
        let mut v: Vec<_> = report.iter().cloned().collect();
        v.remove(i);
        if is_safe(&v) {
            return true;
        }
    }

    // If we got here we can give up
    false
}

fn main() {
    let input = read_input();
    let score: i64 = input
        .iter()
        .map(|report| match is_safe_tolerant(report) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("{score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let inputs = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];
        for (report, expected) in inputs {
            assert_eq!(
                is_safe(&report),
                expected,
                "{report:?} should be {expected}"
            );
        }
    }

    #[test]
    fn test_tolerant_case() {
        let inputs = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], true),
            (vec![8, 6, 4, 4, 1], true),
            (vec![1, 3, 6, 7, 9], true),
        ];
        for (report, expected) in inputs {
            assert_eq!(
                is_safe_tolerant(&report),
                expected,
                "{report:?} should be {expected}"
            );
        }
    }
}

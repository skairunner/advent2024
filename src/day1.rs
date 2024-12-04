use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

/// Read input and parse.
fn read_input() -> (Vec<i64>, Vec<i64>) {
    let file = File::open("inputs/day1.txt").unwrap();
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut it = line.split("   ").map(|entry| i64::from_str(entry).unwrap());
        list1.push(it.next().unwrap());
        list2.push(it.next().unwrap());
    }

    (list1, list2)
}

struct Number {
    pub n: i64,
    pub pos: usize,
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let o = self.n.partial_cmp(&other.n);
        match o {
            Some(Ordering::Equal) => self.pos.partial_cmp(&other.pos),
            other => other,
        }
    }
}

impl Eq for Number {}

// bit of a funky partialeq implementation but i'm lazy
impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n.cmp(&other.n)
    }
}

/// Transform lists into pairs of (number, position)
fn transform_list(list: &[i64]) -> Vec<Number> {
    list.iter()
        .enumerate()
        .map(|(pos, n)| Number { pos, n: *n })
        .collect()
}

/// Do the operation in the problem description
fn match_numbers(list1: &[i64], list2: &[i64]) -> usize {
    // First, parse the list numbers into pairs of (number, smallest position)
    let mut list1 = transform_list(list1);
    let mut list2 = transform_list(list2);
    list1.sort();
    list2.sort();

    // Now zip the two and calculate scores.
    list1
        .iter()
        .zip(&list2)
        .map(|(n1, n2)| n1.pos.abs_diff(n2.pos))
        .sum()
}

fn main() {
    let (list1, list2) = read_input();
    let n = match_numbers(&list1, &list2);
    println!("{n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(match_numbers(&list1, &list2), 11);
    }
}

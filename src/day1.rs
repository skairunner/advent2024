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


/// Do the operation in the problem description
fn match_numbers(mut list1: Vec<i64>, mut list2: Vec<i64>) -> u64 {
    list1.sort();
    list2.sort();

    // Now zip the two and calculate scores.
    list1
        .into_iter()
        .zip(list2)
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum()
}

fn calculate_similarity(list1: Vec<i64>, list2: Vec<i64>) -> usize {
    // Transform list2 into a count of numbers
    let mut numbers: HashMap<i64, usize> = HashMap::new();
    for n in list2 {
        if let Some(count) = numbers.get_mut(&n) {
            *count += 1;
        } else {
            numbers.insert(n, 1);
        }
    }

    // Then check how many times list1's numbers appear in list2
    list1.into_iter()
        .map(|n| (n as usize) * numbers.get(&n).unwrap_or(&0))
        .sum()
}

fn main() {
    let (list1, list2) = read_input();
    let n = calculate_similarity(list1, list2);
    println!("{n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(match_numbers(list1, list2), 11);
    }

    #[test]
    fn test_similarity() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_similarity(list1, list2), 31);
    }
}

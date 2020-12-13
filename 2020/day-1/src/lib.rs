use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp::Ordering;

use std::collections::{HashSet, HashMap};

pub fn run(path: &str) {
    print!("Day 1 Part 1 running...");
    assert_eq!(1013211, part_1(path).unwrap());
    println!("Finished.");
}

// O(n) HashSet
fn part_1(path: &str) -> Option<u32> {
    let input_file = File::open(path).unwrap();
    let reader = BufReader::new(&input_file);
    let mut visited = HashSet::new();

    for expense in reader.lines() {
        let expense = expense.unwrap().parse::<u32>().unwrap();
        let matching_expense = 2020 - expense;

        if visited.contains(&matching_expense) {
            return Some(matching_expense * expense);
        }

        visited.insert(expense);
    }

    None
}

fn part_2(path: &str) -> Option<u32> {
    let expenses = file_lines_to_u32s(Path::new(path));
    let mut visited = HashMap::new();
    let mut counter = 0;

    for i in 0..expenses.len() {
        let expense = expenses[i];
        let matching_expense = 2020 - expense;

        if visited.contains_key(&matching_expense) {
            let value = visited.get(&matching_expense).unwrap();

            return Some(expense * value * (2020 - expense - value));
        }

        for i in 0..counter {
            visited.insert(expense + expenses[i], expense);
        }

        counter += 1;
    }

    None
}

// Quick sort + a single search. O(n*logn)
fn part_1_old(path: &str) -> Option<u32> {
    let mut expenses: Vec<u32> = file_lines_to_u32s(Path::new(path));
    expenses.sort_unstable();

    let mut low_index = 0;
    let mut high_index = expenses.len() - 1;

    while low_index < high_index {
        let current_sum = expenses[low_index] + expenses[high_index];
        
        match current_sum.partial_cmp(&2020).unwrap() {
            Ordering::Equal => return Some(expenses[low_index] * expenses[high_index]),
            Ordering::Less => low_index += 1,
            Ordering::Greater => high_index -= 1
        }
    }

    None
}

fn file_lines_to_u32s(path: &Path) -> Vec<u32> {
    let input_file = File::open(path).unwrap();
    let reader = BufReader::new(&input_file);

    reader.lines()
        .map(|number_string| number_string.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn file_lines_to_strings(path: &Path) -> Vec<String> {
    let input_file = File::open(path).unwrap();
    let reader = BufReader::new(&input_file);

    reader.lines()
        .map(|number_string| number_string.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_valid() {
        assert_eq!(1013211, part_1("input.txt").unwrap());
    }

    #[test]
    fn part_2_valid() {
        assert_eq!(13891280, part_2("input.txt").unwrap());
    }
}
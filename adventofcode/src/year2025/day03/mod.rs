use std::fs;
use std::io::{self, BufRead};

use crate::local_path;

fn max_two_digits(digits_str: &str) -> u32 {
    let mut digits = digits_str.chars().filter_map(|c| c.to_digit(10));

    let mut max_first_digit = match digits.next() {
        Some(d) => d,
        None => return 0,
    };

    let mut max_number = 0;

    for second_digit in digits {
        let number = max_first_digit * 10 + second_digit;
        max_number = max_number.max(number);

        max_first_digit = max_first_digit.max(second_digit);
    }

    max_number
}

fn max_twelve_digits(digits_str: &str) -> u64 {
    let digits: Vec<u32> = digits_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let target_length = 12;

    if digits.len() <= target_length {
        return to_number(&digits);
    }

    let selected = select_max_subsequence(&digits, target_length);
    to_number(&selected)
}

fn select_max_subsequence(digits: &[u32], count: usize) -> Vec<u32> {
    let to_remove = digits.len() - count;
    let mut stack = Vec::with_capacity(count);
    let mut removals_left = to_remove;

    for &digit in digits {
        while !stack.is_empty()
              && removals_left > 0
              && *stack.last().unwrap() < digit
        {
            stack.pop();
            removals_left -= 1;
        }

        stack.push(digit);
    }

    stack.truncate(count);

    stack
}

fn to_number(digits: &[u32]) -> u64 {
    digits.iter().fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}

pub fn solve1() -> io::Result<u32> {
    let file_path = local_path!();

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let total = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| max_two_digits(line.trim()))
        .sum();

    Ok(total)
}

pub fn solve2() -> io::Result<u64> {
    let file_path = local_path!();
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let total: u64 = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| max_twelve_digits(line.trim()))
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_two_digits() {
        assert_eq!(max_two_digits("987654321111111"), 98);
        assert_eq!(max_two_digits("811111111111119"), 89);
        assert_eq!(max_two_digits("234234234234278"), 78);
        assert_eq!(max_two_digits("818181911112111"), 92);

        assert_eq!(max_two_digits("12"), 12);
        assert_eq!(max_two_digits("21"), 21);
        assert_eq!(max_two_digits("99"), 99);
        assert_eq!(max_two_digits("19"), 19);
    }

#[test]
    fn test_select_max_subsequence() {
        let digits = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let result = select_max_subsequence(&digits, 12);
        assert_eq!(result, vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);
    }
}
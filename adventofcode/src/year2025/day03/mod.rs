use std::fs;
use std::io::{self, BufRead};

use crate::local_path;

fn max_joltage_from_bank(bank: &str) -> u32 {
    let mut digits = bank.chars().filter_map(|c| c.to_digit(10));

    let mut max_left = match digits.next() {
        Some(d) => d,
        None => return 0,
    };
    let mut max_joltage = 0;

    for current in digits {
        let joltage = max_left * 10 + current;
        max_joltage = max_joltage.max(joltage);

        max_left = max_left.max(current);
    }

    max_joltage
}

pub fn solve1() -> io::Result<u32> {
    let file_path = local_path!();

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let total = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| max_joltage_from_bank(line.trim()))
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage_from_bank() {
        assert_eq!(max_joltage_from_bank("987654321111111"), 98);
        assert_eq!(max_joltage_from_bank("811111111111119"), 89);
        assert_eq!(max_joltage_from_bank("234234234234278"), 78);
        assert_eq!(max_joltage_from_bank("818181911112111"), 92);

        assert_eq!(max_joltage_from_bank("12"), 12);
        assert_eq!(max_joltage_from_bank("21"), 21);
        assert_eq!(max_joltage_from_bank("99"), 99);
        assert_eq!(max_joltage_from_bank("19"), 19);
    }
}
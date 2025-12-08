use std::fs;
use std::io::{self, BufRead};

use crate::local_path;

fn find_max_two_digit_number(digits_str: &str) -> u32 {
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

pub fn solve1() -> io::Result<u32> {
    let file_path = local_path!();

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let total = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| find_max_two_digit_number(line.trim()))
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage_from_bank() {
        assert_eq!(find_max_two_digit_number("987654321111111"), 98);
        assert_eq!(find_max_two_digit_number("811111111111119"), 89);
        assert_eq!(find_max_two_digit_number("234234234234278"), 78);
        assert_eq!(find_max_two_digit_number("818181911112111"), 92);

        assert_eq!(find_max_two_digit_number("12"), 12);
        assert_eq!(find_max_two_digit_number("21"), 21);
        assert_eq!(find_max_two_digit_number("99"), 99);
        assert_eq!(find_max_two_digit_number("19"), 19);
    }
}
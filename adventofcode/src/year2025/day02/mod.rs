use std::{fs, io};

use crate::local_path;

fn has_matching_halves(id: u64) -> bool {
    let digit_count = count_digits(id);

    if !has_even_digit_count(digit_count) {
        return false;
    }

    let (first_half, second_half) = split_number_in_half(id, digit_count);

    first_half == second_half
}

fn count_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn has_even_digit_count(digit_count: u32) -> bool {
    digit_count % 2 == 0
}

fn split_number_in_half(n: u64, digit_count: u32) -> (u64, u64) {
    let midpoint = digit_count / 2;
    let divisor = 10u64.pow(midpoint);

    let first_half = n / divisor;
    let second_half = n % divisor;

    (first_half, second_half)
}

fn parse_range(range_str: &str) -> Option<(u64, u64)> {
    let (start_str, end_str) = range_str.split_once('-')?;

    let start = start_str.parse().ok()?;
    let end = end_str.parse().ok()?;

    Some((start, end))
}

fn calculate_matching_halves_sum(file_path: &str) -> io::Result<u64> {
    let ranges = fs::read_to_string(file_path)?;

    let result = ranges
        .split(',')
        .filter_map(|range_str| parse_range(range_str.trim()))
        .flat_map(|(start, end)| start..=end)
        .filter(|&id| has_matching_halves(id))
        .sum();

    Ok(result)
}

pub fn solve1() -> io::Result<u64> {
    let file_path = local_path!();

    let result = calculate_matching_halves_sum(&file_path)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids_with_matching_halves() {
        assert!(has_matching_halves(55));
        assert!(has_matching_halves(6464));
        assert!(has_matching_halves(123123));
        assert!(has_matching_halves(11));
        assert!(has_matching_halves(22));
        assert!(has_matching_halves(99));
        assert!(has_matching_halves(1010));
    }

    #[test]
    fn test_ids_without_matching_halves() {
        assert!(!has_matching_halves(101));
        assert!(!has_matching_halves(12));
        assert!(!has_matching_halves(1234));
        assert!(!has_matching_halves(100));
    }

    #[test]
    fn test_parse_range_valid() {
        assert_eq!(parse_range("11-22"), Some((11, 22)));
        assert_eq!(parse_range("100-200"), Some((100, 200)));
    }

    #[test]
    fn test_parse_range_invalid() {
        assert_eq!(parse_range("11"), None);
        assert_eq!(parse_range("11-22-33"), None);
        assert_eq!(parse_range("abc-def"), None);
    }
}
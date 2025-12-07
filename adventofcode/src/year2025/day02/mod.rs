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

fn has_repeated_pattern(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    (1..=len / 2)
        .filter(|&pattern_len| is_valid_pattern_length(len, pattern_len))
        .any(|pattern_len| matches_repeated_pattern(&s, pattern_len))
}

fn is_valid_pattern_length(total_len: usize, pattern_len: usize) -> bool {
    total_len % pattern_len == 0 && total_len / pattern_len >= 2
}

fn matches_repeated_pattern(s: &str, pattern_len: usize) -> bool {
    let pattern = &s[..pattern_len];
    let repeats = s.len() / pattern_len;

    (1..repeats).all(|i| {
        let start = i * pattern_len;
        let end = start + pattern_len;

        &s[start..end] == pattern
    })
}

fn parse_range(range_str: &str) -> Option<(u64, u64)> {
    let (start_str, end_str) = range_str.split_once('-')?;

    let start = start_str.parse().ok()?;
    let end = end_str.parse().ok()?;

    Some((start, end))
}

fn sum_matching_ids<F>(ranges: &str, predicate: F) -> u64
where
    F: Fn(u64) -> bool,
{
    ranges
        .split(',')
        .filter_map(|range_str| parse_range(range_str.trim()))
        .flat_map(|(start, end)| start..=end)
        .filter(|&id| predicate(id))
        .sum()
}

pub fn solve1() -> io::Result<u64> {
    let file_path = local_path!();
    let ranges = fs::read_to_string(file_path)?;

    let result = sum_matching_ids(&ranges, has_matching_halves);
    Ok(result)
}

pub fn solve2() -> io::Result<u64> {
    let file_path = local_path!();
    let ranges = fs::read_to_string(file_path)?;

    let result = sum_matching_ids(&ranges, has_repeated_pattern);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_matching_halves() {
        assert!(has_matching_halves(55));
        assert!(has_matching_halves(6464));
        assert!(has_matching_halves(123123));
        assert!(has_matching_halves(11));
        assert!(has_matching_halves(22));
        assert!(has_matching_halves(99));
        assert!(has_matching_halves(1010));

        assert!(!has_matching_halves(101));
        assert!(!has_matching_halves(12));
        assert!(!has_matching_halves(1234));
        assert!(!has_matching_halves(100));
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(5), 1);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1000), 4);
    }

    #[test]
    fn test_split_number_in_half() {
        assert_eq!(split_number_in_half(1234, 4), (12, 34));
        assert_eq!(split_number_in_half(123123, 6), (123, 123));
        assert_eq!(split_number_in_half(5555, 4), (55, 55));
    }

    #[test]
    fn test_is_valid_pattern_length() {
        assert!(is_valid_pattern_length(6, 3));
        assert!(is_valid_pattern_length(6, 2));
        assert!(is_valid_pattern_length(6, 1));

        assert!(!is_valid_pattern_length(6, 4)); // doesn't divide evenly
        assert!(!is_valid_pattern_length(6, 6)); // only 1 repeat
    }

    #[test]
    fn test_has_repeated_pattern() {
        assert!(has_repeated_pattern(11));
        assert!(has_repeated_pattern(99));
        assert!(has_repeated_pattern(111));
        assert!(has_repeated_pattern(1010));
        assert!(has_repeated_pattern(565656));
        assert!(has_repeated_pattern(824824824));
        assert!(has_repeated_pattern(2121212121));

        assert!(!has_repeated_pattern(101));
        assert!(!has_repeated_pattern(1698522));
    }

    #[test]
    fn test_matches_repeated_pattern() {
        assert!(matches_repeated_pattern("1111", 1));
        assert!(matches_repeated_pattern("123123", 3));
        assert!(matches_repeated_pattern("ababab", 2));
        assert!(!matches_repeated_pattern("1234", 2));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("11-22"), Some((11, 22)));
        assert_eq!(parse_range("100-200"), Some((100, 200)));
        assert_eq!(parse_range("11"), None);
        assert_eq!(parse_range("11-22-33"), None);
        assert_eq!(parse_range("abc-def"), None);
    }
}
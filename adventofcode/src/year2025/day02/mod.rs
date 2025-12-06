use std::{fs, io};

use crate::local_path;

fn has_matching_halves(id: u64) -> bool {
    let id_string = id.to_string();
    let digit_count = id_string.len();

    if digit_count % 2 != 0 {
        return false;
    }

    let midpoint = digit_count / 2;
    let (first_half, second_half) = id_string.split_at(midpoint);

    first_half == second_half
}

fn parse_range(range_str: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = range_str.split('-').collect();

    if parts.len() != 2 {
        return None;
    }

    let start = parts[0].parse::<u64>().ok()?;
    let end = parts[1].parse::<u64>().ok()?;

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
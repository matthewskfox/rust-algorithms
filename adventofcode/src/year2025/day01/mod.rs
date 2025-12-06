use std::{fs, i32};
use std::io::{self, BufRead};

use crate::local_path;

const DIAL_SIZE: i32 = 100;
const STARTING_POSITION: i32 = 50;

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let direction = match line.chars().next()? {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return None,
        };

        let distance = line[1..].parse::<i32>().ok()?;

        Some(Rotation {
            direction,
            distance,
        })
    }

    fn apply(&self, current_position: i32) -> i32 {
        let new_position = match self.direction {
            Direction::Left => current_position - self.distance,
            Direction::Right => current_position + self.distance,
        };

        // Normalize to 0-99 range using modulo arithmetic
        // Add DIAL_SIZE before mod to handle negative numbers correctly
        ((new_position % DIAL_SIZE) + DIAL_SIZE) % DIAL_SIZE
    }

    fn count_zeros_during_rotation(&self, start_position: i32) -> i32 {
        if self.distance == 0 {
            return 0;
        }

        let end_position = self.apply(start_position);
        let complete_loops = self.distance / DIAL_SIZE;

        let crosses_zero = match self.direction {
            // For leftward movement, if we start at 0, that is not crossing
            Direction::Left => start_position < end_position && start_position != 0,
            Direction::Right => end_position < start_position,
        };

        let lands_on_zero  = end_position == 0 && start_position != 0;

        let partial_crosses = if crosses_zero || lands_on_zero  { 1 } else { 0 };

        complete_loops + partial_crosses
    }
}

fn count_zero_positions(rotations: &[Rotation]) -> usize {
    let mut position = STARTING_POSITION;
    let mut zero_count = 0;

    for rotation in rotations {
        position = rotation.apply(position);
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn count_zero_crossings(rotations: &[Rotation]) -> usize {
    let mut position = STARTING_POSITION;
    let mut zero_count = 0;

    for rotation in rotations {
        zero_count += rotation.count_zeros_during_rotation(position) as usize;
        position = rotation.apply(position);
    }

    zero_count
}

fn read_rotations_from_file(filename: &str) -> io::Result<Vec<Rotation>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    let directions = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| Rotation::parse(&line))
        .collect();

    Ok(directions)
}

pub fn solve1() -> io::Result<usize> {
    let file_path = local_path!();

    let rotations = read_rotations_from_file(&file_path)?;
    let result = count_zero_positions(&rotations);

    Ok(result)
}

pub fn solve2() -> io::Result<usize> {
    let file_path = local_path!();

    let rotations = read_rotations_from_file(&file_path)?;
    let result = count_zero_crossings(&rotations);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_parsing() {
        let rotation = Rotation::parse("L68").unwrap();
        assert!(matches!(rotation.direction, Direction::Left));
        assert_eq!(rotation.distance, 68);

        let rotation = Rotation::parse("R48").unwrap();
        assert!(matches!(rotation.direction, Direction::Right));
        assert_eq!(rotation.distance, 48);
    }

    #[test]
    fn test_rotation_application() {
        // Starting at 11, R8 -> 19
        let rotation = Rotation {
            direction: Direction::Right,
            distance: 8,
        };
        assert_eq!(rotation.apply(11), 19);

        // Starting at 19, L19 -> 0
        let rotation = Rotation {
            direction: Direction::Left,
            distance: 19,
        };
        assert_eq!(rotation.apply(19), 0);

        // Starting at 5, L10 -> 95
        let rotation = Rotation {
            direction: Direction::Left,
            distance: 10,
        };
        assert_eq!(rotation.apply(5), 95);

        // Starting at 95, R5 -> 0
        let rotation = Rotation {
            direction: Direction::Right,
            distance: 5,
        };
        assert_eq!(rotation.apply(95), 0);
    }

    #[test]
    fn test_individual_moves() {
        let mut pos = 50;

        let r = Rotation::parse("L68").unwrap();
        pos = r.apply(pos);
        assert_eq!(pos, 82);

        let r = Rotation::parse("L30").unwrap();
        pos = r.apply(pos);
        assert_eq!(pos, 52);

        let r = Rotation::parse("R48").unwrap();
        pos = r.apply(pos);
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_count_zero_crossings() {
        let rotations = vec![
            Rotation::parse("L68").unwrap(),
            Rotation::parse("L30").unwrap(),
            Rotation::parse("R48").unwrap(),
            Rotation::parse("L5").unwrap(),
            Rotation::parse("R60").unwrap(),
            Rotation::parse("L55").unwrap(),
            Rotation::parse("L1").unwrap(),
            Rotation::parse("L99").unwrap(),
            Rotation::parse("R14").unwrap(),
            Rotation::parse("L82").unwrap(),
        ];

        let result = count_zero_crossings(&rotations);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_large_rotation() {
        let rotation = Rotation::parse("R1000").unwrap();
        let zeros = rotation.count_zeros_during_rotation(50);
        assert_eq!(zeros, 10);
    }
}

use crate::util;
use itertools::Itertools;

pub fn number_of_depth_increases(values: &[i32]) -> usize {
    values
        .iter()
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

pub fn number_of_depth_increases_sliding(values: &[i32]) -> usize {
    values
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

pub fn part1() -> usize {
    let lines = util::read_lines("inputs/day1").unwrap();
    let parsed_lines: Vec<i32> = lines
        .iter()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    number_of_depth_increases(&parsed_lines)
}

pub fn part2() -> usize {
    let lines = util::read_lines("inputs/day1").unwrap();
    let parsed_lines: Vec<i32> = lines
        .iter()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    number_of_depth_increases_sliding(&parsed_lines)
}

use std::collections::HashSet;

use crate::util::DaySolution;
use itertools::Itertools;

pub struct Solution {
    year: u64,
    day: u64,
}

fn char_to_priority(c: char) -> usize {
    let mut value = if c.is_uppercase() { 27 } else { 1 };
    value += (c.to_ascii_lowercase() as usize) - ('a' as usize);
    value
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        lines
            .iter()
            .map(|s| {
                let l = s.len();
                let (first, second) = s[..].split_at(l / 2);
                let left_set = HashSet::<_>::from_iter(first.chars());
                let right_set = HashSet::from_iter(second.chars());
                left_set.intersection(&right_set).next().unwrap().to_owned()
            })
            .map(char_to_priority)
            .sum()
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        lines
            .into_iter()
            .chunks(3)
            .into_iter()
            .map(|mut triplet| {
                let s1 = triplet.next().unwrap();
                let intersection = HashSet::<_>::from_iter(s1.chars());
                let intersection = triplet.fold(intersection, |mut inter, s| {
                    inter.retain(|&v| s.contains(v));
                    inter
                });

                intersection.into_iter().next().unwrap().to_owned()
            })
            .map(char_to_priority)
            .sum()
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

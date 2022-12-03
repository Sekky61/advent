use crate::util::DaySolution;
use itertools::Itertools;

pub struct Solution {
    year: u64,
    day: u64,
}

fn get_carry_amounts(lines: Vec<String>) -> Vec<u64> {
    lines
        .into_iter()
        .map(|s| s.parse::<u64>()) // parse
        .group_by(|el| el.is_ok()) // groups per elv
        .into_iter()
        .filter(|&(is_ok, _)| is_ok)
        .map(|(_, group)| group.map(|e| e.unwrap()))
        .map(|g| g.sum::<u64>())
        .collect()
}

struct MaxN {
    pub buf: [u64; 4],
}

impl MaxN {
    pub fn new() -> MaxN {
        Self { buf: [0, 0, 0, 0] }
    }

    pub fn add(mut self, v: u64) -> MaxN {
        // get lowest of the three
        self.buf[3] = v;
        self.buf.sort_unstable_by(|v1, v2| v2.cmp(v1)); // descending order
        self
    }

    pub fn sum_top_three(self) -> usize {
        self.buf[0..3].iter().sum::<u64>() as usize
    }
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        get_carry_amounts(lines).into_iter().max().unwrap() as usize
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        get_carry_amounts(lines)
            .into_iter()
            .fold(MaxN::new(), |m, v| m.add(v))
            .sum_top_three()
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

use crate::util::DaySolution;

pub struct Solution {
    year: u64,
    day: u64,
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();
        let line = &lines[0];
        let slice = line.as_bytes();

        let ind = slice
            .windows(4)
            .position(|win| (0..win.len()).all(|i| !(win[(i + 1)..].contains(&win[i]))));

        ind.unwrap() + 4 // +-1 errors
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();
        0
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

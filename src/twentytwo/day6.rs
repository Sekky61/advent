use crate::util::DaySolution;

pub struct Solution {
    year: u64,
    day: u64,
}

// Find the index of the end of the first substring of length n that has all unique characters
fn find_first_unique_substring(slice: &[u8], n: usize) -> Option<usize> {
    slice
        .windows(n)
        .position(|win| (0..win.len()).all(|i| !(win[(i + 1)..].contains(&win[i]))))
        .map(|i| i + n)
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let line = self.get_input().unwrap();
        let slice = line.as_bytes();

        find_first_unique_substring(slice, 4).unwrap()
    }

    fn part2_solution(&self) -> usize {
        let line = self.get_input().unwrap();
        let slice = line.as_bytes();

        find_first_unique_substring(slice, 14).unwrap()
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

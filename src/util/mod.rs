use core::panic;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    Ok(lines.map(|l| l.expect("Could not parse line")).collect())
}

pub struct Dispatch {}

impl Dispatch {
    pub fn call(year: u64, day: u64, part: u64) -> usize {
        let sol = match year {
            2021 => Dispatch::twentyone(day),
            2022 => Dispatch::twentytwo(day),
            _ => panic!("Not valid year"),
        };

        match part {
            1 => sol.part1_solution(),
            2 => sol.part2_solution(),
            _ => panic!("Not valid solution number"),
        }
    }

    fn twentyone(day: u64) -> Box<dyn DaySolution> {
        todo!()
    }

    // todo not automatic
    fn twentytwo(day: u64) -> Box<dyn DaySolution> {
        let year = 2022;
        match day {
            1 => Box::new(crate::twentytwo::day1::Solution::new(year, day)),
            2 => Box::new(crate::twentytwo::day2::Solution::new(year, day)),
            3 => Box::new(crate::twentytwo::day3::Solution::new(year, day)),
            6 => Box::new(crate::twentytwo::day6::Solution::new(year, day)),
            7 => Box::new(crate::twentytwo::day7::Solution::new(year, day)),
            _ => panic!("Solution for day {day} does not exist"),
        }
    }
}

pub trait DaySolution {
    fn new(year: u64, day: u64) -> Self
    where
        Self: Sized;

    fn part1_solution(&self) -> usize {
        0
    }

    fn part2_solution(&self) -> usize {
        0
    }

    fn get_year(&self) -> u64;

    fn get_day(&self) -> u64;

    fn get_input(&self) -> io::Result<String> {
        let year_text = year_text(self.get_year());
        let filename = format!("inputs/{}/day{}", year_text, self.get_day());
        read_file(filename)
    }

    fn get_input_lines(&self) -> io::Result<Vec<String>> {
        let year_text = year_text(self.get_year());
        let filename = format!("inputs/{}/day{}", year_text, self.get_day());
        read_lines(filename)
    }
}

fn year_text(year: u64) -> &'static str {
    match year {
        2021 => "twentyone",
        2022 => "twentytwo",
        2023 => "twentythree",
        _ => panic!("Unknown year {year}"),
    }
}

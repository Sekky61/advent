#[allow(dead_code)]
pub mod day1;
#[allow(dead_code)]
pub mod day2;
#[allow(dead_code)]
pub mod day3;
#[allow(dead_code)]
pub mod day4;

pub struct Day1;
pub struct Day2;
pub struct Day3;
pub struct Day4;
pub struct Day5;
pub struct Day6;
pub struct Day7;
pub struct Day8;
pub struct Day9;
pub struct Day10;
pub struct Day11;
pub struct Day12;
pub struct Day13;
pub struct Day14;
pub struct Day15;
pub struct Day16;
pub struct Day17;
pub struct Day18;
pub struct Day19;
pub struct Day20;
pub struct Day21;
pub struct Day22;
pub struct Day23;
pub struct Day24;

impl Day for Day1 {
    fn part1(&self) -> Result<usize, &'static str> {
        Ok(45)
    }

    fn part2(&self) -> Result<usize, &'static str> {
        Ok(78)
    }
}

pub fn solve_challenge(day: usize, part: usize) -> Result<usize, &'static str> {
    let day_obj: Box<dyn Day> = match day {
        1 => Box::new(Day1),
        _ => return Err("Day outside of range 1..=24"),
    };

    let result = match part {
        1 => day_obj.part1()?,
        2 => day_obj.part2()?,
        _ => return Err("Part outside of range 1..=2"),
    };

    Ok(result)
}

pub trait Day {
    fn part1(&self) -> Result<usize, &'static str> {
        Err("Part 1 not implemented")
    }

    fn part2(&self) -> Result<usize, &'static str> {
        Err("Part 2 not implemented")
    }
}

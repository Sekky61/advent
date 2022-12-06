use crate::util::DaySolution;
use itertools::Itertools;
use nom::{bytes::complete::take, character::complete::anychar, sequence::tuple, IResult};

pub struct Solution {
    year: u64,
    day: u64,
}

enum Res {
    Win,
    Lose,
    Draw,
}

impl Res {
    pub fn parse(c: char) -> Self {
        match c {
            'X' => Res::Lose,
            'Y' => Res::Draw,
            'Z' => Res::Win,
            _ => panic!("Unknown letter {c}"),
        }
    }

    pub fn get_points(self) -> usize {
        match self {
            Res::Win => 6,
            Res::Lose => 0,
            Res::Draw => 3,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn parse(c: char) -> Self {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Unknown letter {c}"),
        }
    }

    pub fn fight(self, me: &Hand) -> Res {
        match (self, me) {
            (Hand::Rock, Hand::Paper) => Res::Win,
            (Hand::Rock, Hand::Scissors) => Res::Lose,
            (Hand::Paper, Hand::Rock) => Res::Lose,
            (Hand::Paper, Hand::Scissors) => Res::Win,
            (Hand::Scissors, Hand::Rock) => Res::Win,
            (Hand::Scissors, Hand::Paper) => Res::Lose,
            _ => Res::Draw,
        }
    }

    pub fn get_points(self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn need_to(&self, result: &Res) -> Hand {
        match result {
            Res::Win => match self {
                // Need to beat this
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            Res::Lose => match self {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            Res::Draw => *self,
        }
    }
}

fn take1(s: &str) -> IResult<&str, &str> {
    take(1usize)(s)
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        lines
            .into_iter()
            .map(|l| {
                let (_, (first, _, second)) = tuple((anychar, take1, anychar))(&l).unwrap();
                let (enemy, me) = (Hand::parse(first), Hand::parse(second));
                let p1 = enemy.fight(&me).get_points();
                let p2 = me.get_points();
                p1 + p2
            })
            .sum()
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();
        lines
            .into_iter()
            .map(|l| {
                let (_, (first, _, second)) = tuple((anychar, take1, anychar))(&l).unwrap();
                let (enemy, result) = (Hand::parse(first), Res::parse(second));
                let my_hand = enemy.need_to(&result);
                let p1 = my_hand.get_points();
                let p2 = result.get_points();
                p1 + p2
            })
            .sum()
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

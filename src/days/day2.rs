use crate::filereader;

pub enum Direction {
    Forward,
    Up,
    Down,
}

pub struct Command {
    direction: Direction,
    amount: usize,
}

impl Command {
    pub fn from_string(s: &str) -> Command {
        let mut split = s.split_whitespace();
        let command_string = split.next().expect("bad format 1");
        let amount: usize = split
            .next()
            .expect("bad format 2")
            .parse()
            .expect("Not a number");

        let direction = match command_string {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("not a direction"),
        };

        Command { direction, amount }
    }
}

// sum commands, return depth*horizontal
pub fn resulting_offset(commands: &[Command]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    commands.iter().for_each(|command| match command {
        Command {
            direction: Direction::Forward,
            amount,
        } => horizontal += amount,
        Command {
            direction: Direction::Up,
            amount,
        } => depth -= amount,
        Command {
            direction: Direction::Down,
            amount,
        } => depth += amount,
    });

    horizontal * depth
}

// sum commands, return depth*horizontal
pub fn resulting_offset_aim(commands: &[Command]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    commands.iter().for_each(|command| match command {
        Command {
            direction: Direction::Forward,
            amount,
        } => {
            horizontal += amount;
            depth += aim * amount
        }
        Command {
            direction: Direction::Up,
            amount,
        } => aim -= amount,
        Command {
            direction: Direction::Down,
            amount,
        } => aim += amount,
    });

    horizontal * depth
}

pub fn part1() -> usize {
    let lines = filereader::read_lines("inputs/day2").unwrap();
    let parsed_lines: Vec<_> = lines
        .iter()
        .map(|line| Command::from_string(line))
        .collect();

    resulting_offset(&parsed_lines)
}

pub fn part2() -> usize {
    let lines = filereader::read_lines("inputs/day2").unwrap();
    let parsed_lines: Vec<_> = lines
        .iter()
        .map(|line| Command::from_string(line))
        .collect();

    resulting_offset_aim(&parsed_lines)
}

use crate::util::DaySolution;
use itertools::Itertools;
use nom::combinator::map;
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::opt,
    multi::{many0, many1},
    sequence::tuple,
};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .copied()
    }

    pub fn to_vector(self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

/// Represents a position on the field, with the top left being (0, 0)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Field {
    lines: Vec<Vec<u64>>,
}

impl Field {
    pub fn from_input(input: &str) -> Result<Self, &'static str> {
        let (_, field) = Field::parse(input).map_err(|_| "Failed to parse input")?;

        Ok(field)
    }

    pub fn width(&self) -> usize {
        self.lines[0].len()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    fn parse_line(input: &str) -> nom::IResult<&str, Vec<u64>> {
        let x = tuple((many1(take_one_digit), opt(newline)));
        map(x, |(digits, _)| digits)(input)
    }

    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, lines) = many0(Field::parse_line)(input)?;

        Ok((input, Self { lines }))
    }

    pub fn iter(&self) -> FieldIterator {
        FieldIterator::new(self)
    }

    pub fn get_tree(&self, pos: Position) -> Option<u64> {
        self.lines
            .get(pos.y)
            .and_then(|line| line.get(pos.x).copied())
    }

    fn look_direction(&self, pos: Position, direction: Direction) -> TreeIterator {
        TreeIterator::new(self, pos, direction)
    }

    pub fn tree_visible(&self, pos: Position) -> bool {
        let tree = self.get_tree(pos).unwrap();

        Direction::all()
            .map(|dir| self.look_direction(pos, dir))
            .any(|mut trees| trees.all(|v| v < tree))
    }

    pub fn scenic_score(&self, pos: Position) -> usize {
        Direction::all()
            .map(|dir| self.look_direction(pos, dir))
            .map(|trees| trees.count_visibility())
            .product()
    }
}

pub struct FieldIterator<'a> {
    field: &'a Field,
    pos: Position,
}

impl<'a> FieldIterator<'a> {
    pub fn new(field: &'a Field) -> Self {
        Self {
            field,
            pos: Position::default(),
        }
    }
}

impl<'a> Iterator for FieldIterator<'a> {
    type Item = (Position, u64);

    fn next(&mut self) -> Option<Self::Item> {
        // Advance to the next position
        self.pos.x += 1;
        if self.pos.x >= self.field.width() {
            self.pos.x = 0;
            self.pos.y += 1;
        }

        // Check if we're done
        if self.pos.y >= self.field.height() {
            return None;
        }

        Some((self.pos, self.field.lines[self.pos.y][self.pos.x]))
    }
}

pub struct TreeIterator<'a> {
    field: &'a Field,
    direction: Direction,
    pos: Position,
    distance: i64,
}

impl<'a> TreeIterator<'a> {
    pub fn new(field: &'a Field, pos: Position, direction: Direction) -> Self {
        Self {
            field,
            direction,
            pos,
            distance: 0,
        }
    }

    fn get_pos(&self) -> Position {
        let change = self.direction.to_vector();
        Position::new(
            self.pos.x + (self.distance * change.0) as usize,
            self.pos.y + (self.distance * change.1) as usize,
        )
    }

    /// How many trees until we see a tree bigger than the first one?
    fn count_visibility(mut self) -> usize {
        let tree = self.field.get_tree(self.pos).unwrap();

        self.take_while_inclusive(|&v| v < tree).count()
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Advance to the next position
        self.distance += 1;

        let p = self.get_pos();
        self.field.get_tree(p)
    }
}

/// Takes exactly one character from the input and returns the numeric value 0-9
fn take_one_digit(input: &str) -> nom::IResult<&str, u64> {
    let (input, digit) = take(1_usize)(input)?;
    let digit = match digit.parse::<u64>() {
        Ok(digit) => digit,
        Err(_) => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )))
        }
    };
    Ok((input, digit))
}

pub struct Solution {
    year: u64,
    day: u64,
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let input = self.get_input().unwrap();
        let field = Field::from_input(&input).unwrap();

        field
            .iter()
            .filter(|(pos, _)| field.tree_visible(*pos))
            .count()
    }

    fn part2_solution(&self) -> usize {
        let input = self.get_input().unwrap();
        let field = Field::from_input(&input).unwrap();

        field
            .iter()
            .map(|(pos, _)| field.scenic_score(pos))
            .max()
            .unwrap()
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_empty() {
        let input = "";
        let res = Field::parse_line(input);

        assert!(res.is_err());
    }

    #[test]
    fn test_parse_line() {
        let input = "123\n4";
        let (input, line) = Field::parse_line(input).unwrap();

        assert_eq!(input, "4");
        assert_eq!(line, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_line2() {
        let input = "123";
        let (input, line) = Field::parse_line(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(line, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse() {
        let input = "123\n456\n789";

        let (_, field) = Field::parse(input).unwrap();

        assert_eq!(field.width(), 3);
        assert_eq!(field.height(), 3);

        assert_eq!(field.lines[0], vec![1, 2, 3]);
        assert_eq!(field.lines[1], vec![4, 5, 6]);
        assert_eq!(field.lines[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_parse_invalid() {
        let input = "123\n456\n78a";
        let field = Field::parse(input);

        assert!(field.is_err());
    }

    #[test]
    fn test_is_visible() {
        let input = "123\n456\n789";
        let (_, field) = Field::parse(input).unwrap();

        assert!(field.tree_visible(Position::new(1, 1)));
        assert!(field.tree_visible(Position::new(0, 0)));
    }

    #[test]
    fn test_is_not_visible() {
        let input = "123\n416\n789";
        let (_, field) = Field::parse(input).unwrap();

        assert!(!field.tree_visible(Position::new(1, 1)));
    }

    #[test]
    fn scenic_score() {
        let input = "123\n456\n789";
        let (_, field) = Field::parse(input).unwrap();

        assert_eq!(field.scenic_score(Position::new(0, 0)), 0);
        assert_eq!(field.scenic_score(Position::new(1, 0)), 0);
        assert_eq!(field.scenic_score(Position::new(2, 0)), 0);
        assert_eq!(field.scenic_score(Position::new(1, 1)), 1);
        assert_eq!(field.scenic_score(Position::new(2, 2)), 0);
    }

    #[test]
    fn scenic_score_2() {
        let input = r#"30373
25512
65332
33549
35390"#;

        let field = Field::from_input(input).unwrap();

        assert_eq!(field.scenic_score(Position::new(2, 3)), 8);
    }
}

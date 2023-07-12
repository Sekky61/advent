use std::{collections::HashSet, hash::Hash};

use crate::util::DaySolution;

const WALL: char = '#';
const GROUND: char = '.';

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position(i64, i64);

impl Position {
    pub fn move_in(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }

    /// Returns the new position if it is inside the map
    pub fn move_checked(&self, direction: Direction, world: &World) -> Option<Self> {
        let new_pos = self.move_in(direction);
        match world.is_ground(new_pos) {
            true => Some(new_pos),
            false => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Blizard {
    position: Position,
    direction: Direction,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl Blizard {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

/// Rectangular map with walls on the edges
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct World {
    /// Map size (width, height)
    map_size: (i64, i64),
    /// Entry point
    entry: Position,
    /// Exit point
    exit: Position,
    blizards: HashSet<Blizard>,
}

impl World {
    /// Returns the position of the hole in the wall if there is only one
    fn parse_horizontal_wall(line: &str) -> Result<i64, &'static str> {
        let width = line.len();
        line.chars()
            .enumerate()
            .try_fold(None, |position, (w, c)| match c {
                WALL => Ok(position),
                GROUND => match w {
                    0 => Err("Entry point in the corner"),
                    _ if w == width - 1 => Err("Entry point in the corner"),
                    _ => match position {
                        None => Ok(Some(w as i64)),
                        Some(_) => Err("Multiple entry points"),
                    },
                },
                _ => Err("Invalid character in the wall"),
            })?
            .ok_or("No entry point")
    }

    /// Parse the ASCII map into a World
    pub fn parse(lines: &[String]) -> Result<World, &'static str> {
        // Parse upper wall
        let upper_wall = lines.first().ok_or("Empty map")?;
        let lower_wall = lines.last().ok_or("Empty map")?;

        let width = upper_wall.len() as i64;
        let height = lines.len() as i64;
        if height < 3 || width < 3 {
            return Err("Map is too small");
        }

        // Check that all lines have the same length
        // Line must start and end with a wall
        if !lines.iter().all(|line| {
            line.len() == (width as usize) && line.starts_with(WALL) && line.ends_with(WALL)
        }) {
            return Err("Lines have different lengths or are not surrounded by walls");
        }

        // Check that the walls are made of '#' and there is one '.' somewhere
        let entry_position = World::parse_horizontal_wall(upper_wall).map(|x| Position(x, 0))?;

        let exit_position =
            World::parse_horizontal_wall(lower_wall).map(|x| Position(x, height - 1))?;

        // Collect all blizards
        let blizards = lines
            .iter()
            .enumerate()
            .flat_map(|(h, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(w, c)| match c.try_into() {
                        Ok(direction) => {
                            Some(Blizard::new(Position(w as i64, h as i64), direction))
                        }
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Ok(Self {
            map_size: (width, height),
            entry: entry_position,
            exit: exit_position,
            blizards,
        })
    }

    /// Get the position of the blizard at the given time
    fn blizard_at(&self, blizard: &Blizard, time: i64) -> Position {
        let Blizard {
            mut position,
            direction,
        } = blizard;
        match direction {
            Direction::Up => {
                let trajectory_length = self.map_size.1 - 2;
                position.1 = (position.1 - 1 - time).rem_euclid(trajectory_length) + 1;
            }
            Direction::Down => {
                let trajectory_length = self.map_size.1 - 2;
                position.1 = (position.1 - 1 + time).rem_euclid(trajectory_length) + 1;
            }
            Direction::Left => {
                let trajectory_length = self.map_size.0 - 2;
                position.0 = (position.0 - 1 - time).rem_euclid(trajectory_length) + 1;
            }
            Direction::Right => {
                let trajectory_length = self.map_size.0 - 2;
                position.0 = (position.0 - 1 + time).rem_euclid(trajectory_length) + 1;
            }
        };
        position
    }

    pub fn blizards_at(&self, time: u64) -> HashSet<Position> {
        self.blizards
            .iter()
            .map(|blizard| self.blizard_at(blizard, time as i64))
            .collect()
    }

    /// Returns true if the position is ground (not a wall). The walls are on the edges of the map except the entry and exit points.
    pub fn is_ground(&self, position: Position) -> bool {
        let (width, height) = self.map_size;
        let Position(x, y) = position;
        let is_inside = x > 0 && x < width - 1 && y > 0 && y < height - 1;
        is_inside || position == self.entry || position == self.exit
    }
}

pub struct Player {
    possible_positions: HashSet<Position>,
}

impl Player {
    pub fn new(stands_at: Position) -> Self {
        let mut possible_positions = HashSet::new();
        possible_positions.insert(stands_at);
        Self { possible_positions }
    }

    pub fn stands_at(&self, position: Position) -> bool {
        self.possible_positions.contains(&position)
    }

    /// Generate all possible positions for the next step. The player can move in any direction or stay at the same position.
    /// Do not care about blizards.
    pub fn next_possible_positions(&self, world: &World) -> HashSet<Position> {
        self.possible_positions
            .iter()
            .flat_map(|position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|direction| position.move_checked(*direction, world))
                    .collect::<HashSet<_>>()
            })
            .chain(self.possible_positions.iter().cloned())
            .collect()
    }
}

// Solution

pub struct Solution {
    year: u64,
    day: u64,
}

impl Solution {
    fn trip_from_to(from: Position, to: Position, time_start: u64, world: &World) -> u64 {
        let mut player = Player::new(from);
        let mut time = time_start;
        while !player.stands_at(to) {
            time += 1;
            let blizards = world.blizards_at(time);
            let next_possible_positions = player.next_possible_positions(world);
            let possible_positions = next_possible_positions
                .difference(&blizards)
                .cloned()
                .collect();
            player.possible_positions = possible_positions;
        }
        time
    }
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        let world = World::parse(&lines[..]).unwrap();
        let time = Solution::trip_from_to(world.entry, world.exit, 0, &world);

        time as usize
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();

        let world = World::parse(&lines[..]).unwrap();
        // Go there, go back, go there again
        let time1 = Solution::trip_from_to(world.entry, world.exit, 0, &world);
        let time2 = Solution::trip_from_to(world.exit, world.entry, time1, &world);
        let time3 = Solution::trip_from_to(world.entry, world.exit, time2, &world);

        time3 as usize
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
    fn test_world_parser() {
        let lines: Vec<_> = ["#.#", "#.#", "#.#"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let world = World::parse(&lines[..]).unwrap();

        assert_eq!(
            world,
            World {
                map_size: (3, 3),
                entry: Position(1, 0),
                exit: Position(1, 2),
                blizards: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_parsing_blizards() {
        let lines: Vec<_> = ["##.##", "#>.<#", "#...#", "#vvv#", "#.###"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let world = World::parse(&lines[..]).unwrap();

        assert_eq!(
            world,
            World {
                map_size: (5, 5),
                entry: Position(2, 0),
                exit: Position(1, 4),
                blizards: vec![
                    Blizard::new(Position(1, 1), Direction::Right),
                    Blizard::new(Position(3, 1), Direction::Left),
                    Blizard::new(Position(1, 3), Direction::Down),
                    Blizard::new(Position(2, 3), Direction::Down),
                    Blizard::new(Position(3, 3), Direction::Down),
                ]
                .into_iter()
                .collect(),
            }
        );
    }

    #[test]
    fn test_blizard_movement() {
        let lines: Vec<_> = ["##.##", "#>.<#", "#...#", "#v..#", "#.###"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let world = World::parse(&lines[..]).unwrap();

        let but = world
            .blizards
            .iter()
            .find(|b| b.position == Position(1, 1))
            .unwrap();

        let blizard_t1 = world.blizard_at(but, 1);
        let blizard_t2 = world.blizard_at(but, 2);
        let blizard_t3 = world.blizard_at(but, 3);

        let but2 = world
            .blizards
            .iter()
            .find(|b| b.position == Position(1, 3))
            .unwrap();

        let blizard2_t1 = world.blizard_at(but2, 1);
        let blizard2_t2 = world.blizard_at(but2, 2);
        let blizard2_t3 = world.blizard_at(but2, 3);

        assert_eq!(blizard_t1, Position(2, 1));
        assert_eq!(blizard_t2, Position(3, 1));
        assert_eq!(blizard_t3, but.position);

        assert_eq!(blizard2_t1, Position(1, 1));
        assert_eq!(blizard2_t2, Position(1, 2));
        assert_eq!(blizard2_t3, but2.position);
    }
}

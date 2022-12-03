use crate::util;
use itertools::Itertools;

// number, is crossed
pub struct BingoField(u8, bool);

impl BingoField {
    pub fn new(n: u8) -> BingoField {
        BingoField(n, false)
    }
}

pub enum MoveResult {
    Nothing,
    Win(u8, usize), // winning number, sum of unchecked
}

pub struct BingoBoard {
    board: Vec<BingoField>,
    winnable: bool,
    win_steps: usize,
    win_number: Option<u8>,
}

impl BingoBoard {
    pub fn new(board: Vec<u8>) -> BingoBoard {
        assert!(board.len() == 5 * 5);
        let board = board.iter().map(|&n| BingoField::new(n)).collect();
        BingoBoard {
            board,
            winnable: false,
            win_steps: 0,
            win_number: None,
        }
    }

    fn cross_number(&mut self, number: u8) -> bool {
        self.win_steps += 1;
        for field in self.board.iter_mut() {
            if field.0 == number {
                field.1 = true;
                return true;
            }
        }

        false
    }

    fn win_condition(&mut self) -> bool {
        let mut col_flags = [true; 5];
        for row in self.board.chunks(5) {
            if row.iter().all(|c| c.1) {
                self.winnable = true;
                return true;
            }
            for (i, cell) in row.iter().enumerate() {
                if !cell.1 {
                    col_flags[i] = false;
                }
            }
        }

        // no winning rows
        let won = col_flags.iter().any(|&flag| flag);
        if won {
            self.winnable = true;
        }
        won
    }

    fn play_round(&mut self, number: u8) -> MoveResult {
        let crossed = self.cross_number(number);
        if crossed {
            let won = self.win_condition();
            if won {
                self.win_number = Some(number);
                let uncrossed_count = self.get_uncrossed_count();
                return MoveResult::Win(number, uncrossed_count);
            }
        }
        MoveResult::Nothing
    }

    fn play_multiple_rounds(&mut self, numbers: &[u8]) {
        for &number in numbers {
            let crossed = self.cross_number(number);
            if crossed {
                let won = self.win_condition();
                if won {
                    self.win_number = Some(number);
                    return;
                }
            }
        }
    }

    fn get_uncrossed_count(&self) -> usize {
        self.board
            .iter()
            .fold(0, |sum, c| if !c.1 { sum + c.0 as usize } else { sum })
    }
}

pub fn parse_input(lines: Vec<String>) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut line_iter = lines.iter();
    let number_line = line_iter.next().expect("wrong input");
    let numbers: Vec<u8> = number_line
        .split(',')
        .map(|c| c.parse().expect("not a number"))
        .collect();

    let mut boards = vec![];

    let chunks = line_iter.chunks(6);
    let boards_input_iter = chunks.into_iter();

    for board_input in boards_input_iter {
        let board: Vec<u8> = board_input
            .skip(1)
            .map(|s| s.split_whitespace())
            .flatten()
            .map(|c| c.parse::<u8>().expect("Not a number"))
            .collect();

        let board = BingoBoard::new(board);
        boards.push(board);
    }

    (numbers, boards)
}

fn first_winner(numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> (usize, usize) {
    for number in numbers {
        for board in &mut boards {
            match board.play_round(number) {
                MoveResult::Nothing => (),
                MoveResult::Win(win_number, sum) => return (win_number as usize, sum),
            }
        }
    }

    (0, 0)
}

fn last_winner(numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> (usize, usize) {
    for board in &mut boards {
        board.play_multiple_rounds(&numbers);
    }

    let last_winner = boards
        .iter()
        .max_by_key(|board| board.win_steps)
        .expect("No max");

    let unchecked_sum = last_winner.get_uncrossed_count();

    (
        last_winner.win_number.expect("No win number") as usize,
        unchecked_sum,
    )
}

pub fn part1() -> usize {
    let lines = util::read_lines("inputs/day4").unwrap();

    let (numbers, boards) = parse_input(lines);

    let (winning_number, winning_board_sum) = first_winner(numbers, boards);

    println!("Ans: {} {}", winning_number, winning_board_sum);

    winning_number * winning_board_sum
}

pub fn part2() -> usize {
    let lines = util::read_lines("inputs/day4").unwrap();

    let (numbers, boards) = parse_input(lines);

    let (winning_number, winning_board_sum) = last_winner(numbers, boards);

    println!("Ans: {} {}", winning_number, winning_board_sum);

    winning_number * winning_board_sum
}

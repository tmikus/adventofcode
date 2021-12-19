use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

const BOARD_WIDTH: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

#[derive(Clone, Copy)]
struct BingoBoardNumber {
    checked: bool,
    number: i32,
}

struct BingoBoard {
    numbers: [BingoBoardNumber; BOARD_SIZE],
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            numbers: [
                BingoBoardNumber {
                    checked: false,
                    number: 0,
                };
                BOARD_SIZE
            ],
        }
    }

    fn check_number(&mut self, number: i32) -> bool {
        for mut item in self.numbers.iter_mut() {
            if item.number == number {
                item.checked = true;
            }
        }
        self.is_bingo()
    }

    fn is_bingo(&self) -> bool {
        for i in 0..BOARD_WIDTH {
            if self.is_column_bingo(i) {
                return true;
            }
            if self.is_row_bingo(i) {
                return true;
            }
        }
        false
    }

    fn is_column_bingo(&self, column_index: usize) -> bool {
        self.numbers[0 * BOARD_WIDTH + column_index].checked
        && self.numbers[1 * BOARD_WIDTH + column_index].checked
        && self.numbers[2 * BOARD_WIDTH + column_index].checked
        && self.numbers[3 * BOARD_WIDTH + column_index].checked
        && self.numbers[4 * BOARD_WIDTH + column_index].checked
    }

    fn is_row_bingo(&self, row_index: usize) -> bool {
        self.numbers[row_index * BOARD_WIDTH + 0].checked
        && self.numbers[row_index * BOARD_WIDTH + 1].checked
        && self.numbers[row_index * BOARD_WIDTH + 2].checked
        && self.numbers[row_index * BOARD_WIDTH + 3].checked
        && self.numbers[row_index * BOARD_WIDTH + 4].checked
    }

    fn set_number(&mut self, row: usize, column: usize, number: i32) {
        self.numbers[BOARD_WIDTH * row + column].number = number;
    }

    fn sum_unmarked_numbers(&self) -> i32 {
        let mut sum = 0;
        for number in self.numbers.iter() {
            if !number.checked {
                sum += number.number;
            }
        }
        sum
    }
}

fn read_checked_numbers(lines: &mut Lines<StdinLock>) -> Vec<i32> {
    let line = lines.next().unwrap().unwrap();
    lines.next(); // Clear next empty line
    let numbers = line.split(',');
    let mut result = Vec::new();
    for number in numbers {
        result.push(number.parse::<i32>().unwrap());
    }
    result
}

fn simple_solution() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let checked_numbers = read_checked_numbers(&mut lines);
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut current_board = BingoBoard::new();
    let mut row_index = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        if line.is_empty() {
            row_index = 0;
            boards.push(current_board);
            current_board = BingoBoard::new();
            continue;
        }
        let line_numbers = line.split_whitespace();
        for (column_index, number) in line_numbers.enumerate() {
            let n = number.parse().unwrap();
            current_board.set_number(
                row_index,
                column_index,
                n
            );
        }
        row_index += 1;
    }
    for number in checked_numbers {
        for mut board in boards.iter_mut() {
            if board.check_number(number) {
                println!("Result: {}", board.sum_unmarked_numbers() * number);
                return;
            }
        }
    }
}


fn complex_solution() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let checked_numbers = read_checked_numbers(&mut lines);
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut current_board = BingoBoard::new();
    let mut row_index = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        if line.is_empty() {
            row_index = 0;
            boards.push(current_board);
            current_board = BingoBoard::new();
            continue;
        }
        let line_numbers = line.split_whitespace();
        for (column_index, number) in line_numbers.enumerate() {
            let n = number.parse().unwrap();
            current_board.set_number(
                row_index,
                column_index,
                n
            );
        }
        row_index += 1;
    }
    for number in checked_numbers {
        println!("Processing number: {}", number);
        let mut updated_boards = Vec::new();
        let mut last_winning_board: Option<BingoBoard> = None;
        for mut board in boards {
            if board.check_number(number) {
                last_winning_board = Some(board);
            } else {
                updated_boards.push(board);
            }
        }
        if updated_boards.len() == 0 {
            println!("Result: {}", last_winning_board.unwrap().sum_unmarked_numbers() * number);
            last_winning_board = None;
            return;
        }
        boards = updated_boards;
    }
}

fn main() {
    // simple_solution();
    complex_solution();
}

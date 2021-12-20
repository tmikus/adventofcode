use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct Board {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Board {
    fn new() -> Board {
        Board {
            data: vec![],
            height: 0,
            width: 0,
        }
    }

    fn count_local_minima(&self) -> u32 {
        let mut result = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_local_minimum(x as usize, y as usize) {
                    result += 1 + self.get(x, y) as u32;
                }
            }
        }
        result
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]
    }

    fn is_local_minimum(&self, x: usize, y: usize) -> bool {
        let value = self.get(x, y);
        if y > 0 {
            if value >= self.get(x, y - 1) {
                return false;
            }
        }
        if (x > 0 && value >= self.get(x - 1, y))
        || (x < (self.width - 1) && value >= self.get(x + 1, y)) {
            return false
        }
        if y < (self.height - 1) {
            if value >= self.get(x, y + 1) {
                return false;
            }
        }
        true
    }

    fn parse(lines: Lines<StdinLock>) -> Board {
        let mut board = Board::new();
        for line in lines {
            let line = line.unwrap();
            for char in line.chars() {
                let number = match char {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => panic!("not an umber, brah!"),
                };
                board.data.push(number);
            };
            board.width = line.len();
            board.height += 1;
        }
        board
    }
}

fn main() {
    let stdin = io::stdin();
    let board = Board::parse(stdin.lock().lines());
    println!("Result: {}", board.count_local_minima());
}

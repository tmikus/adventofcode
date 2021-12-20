use std::collections::VecDeque;
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
                if self.is_local_minimum(x, y) {
                    result += 1 + self.get(x, y) as u32;
                }
            }
        }
        result
    }

    fn compute_complex_result(&self) -> u64 {
        let mut sizes = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_local_minimum(x, y) {
                    sizes.push(self.compute_basin_size(x, y));
                }
            }
        }
        sizes.sort();
        println!("{:?}", sizes);
        let len = sizes.len();
        sizes[len - 1]
          * sizes[len - 2]
          * sizes[len - 3]
    }

    fn compute_basin_size(&self, start_x: usize, start_y: usize) -> u64 {
        let mut visit_map = vec![false; self.data.len()];
        let mut size = 0;
        let mut point_queue = VecDeque::from([(start_x, start_y)]);
        while !point_queue.is_empty() {
            let (x, y) = point_queue.pop_front().unwrap();
            if visit_map[y * self.width + x] {
                continue;
            }
            visit_map[y * self.width + x] = true;
            if self.get(x, y) == 9 {
                continue;
            }
            size += 1;
            // Try move left
            if x > 0 && !visit_map[y * self.width + x - 1] {
                point_queue.push_back((x - 1, y));
            }
            // Try move right
            if x < (self.width - 1) && !visit_map[y * self.width + x + 1] {
                point_queue.push_back((x + 1, y));
            }
            // Try move up
            if y > 0 && !visit_map[(y - 1) * self.width + x] {
                point_queue.push_back((x, y - 1));
            }
            // Try move down
            if y < (self.height - 1) && !visit_map[(y + 1) * self.width + x] {
                point_queue.push_back((x, y + 1));
            }
        }
        size
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
    // println!("Result: {}", board.count_local_minima()); // Simple Result
    println!("Result: {}", board.compute_complex_result());
}

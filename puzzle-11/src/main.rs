use std::cmp::min;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct Board {
    energy_levels: Vec<i8>,
    height: usize,
    width: usize,
}

impl Board {
    fn get(&self, x: usize, y: usize) -> i8 {
        self.energy_levels[y * self.width + x]
    }

    fn increment(&mut self, x: usize, y: usize) -> i8 {
        let value = self.get(x, y);
        if value == -1 {
            return value;
        }
        self.set(x, y, min(value + 1, 10))
    }

    fn parse(lines: Lines<StdinLock>) -> Board {
        let mut energy_levels = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            for c in line.as_bytes() {
                energy_levels.push(match c {
                    b'0' => 0,
                    b'1' => 1,
                    b'2' => 2,
                    b'3' => 3,
                    b'4' => 4,
                    b'5' => 5,
                    b'6' => 6,
                    b'7' => 7,
                    b'8' => 8,
                    b'9' => 9,
                    _ => 0,
                });
            }
            width = line.len();
            height += 1;
        }
        Board {
            energy_levels,
            height,
            width,
        }
    }

    fn run_step(&mut self) -> u32 {
        let mut queue = VecDeque::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.get(x, y) + 1;
                if value > 9 {
                    queue.push_back((x, y));
                }
                self.set(x, y, value);
            }
        }
        let mut flashes = 0;
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if self.get(x, y) <= 9 {
                continue;
            }
            flashes += 1;
            self.set(x, y, -1);
            if y > 0 {
                if x > 0 {
                    if self.increment(x - 1, y - 1) > 9 {
                        queue.push_back((x - 1, y - 1));
                    }
                }
                if self.increment(x, y - 1) > 9 {
                    queue.push_back((x, y - 1));
                }
                if x < (self.width - 1) {
                    if self.increment(x + 1, y - 1) > 9 {
                        queue.push_back((x + 1, y - 1));
                    }
                }
            }
            if x > 0 {
                if self.increment(x - 1, y) > 9 {
                    queue.push_back((x - 1, y));
                }
            }
            if x < (self.width - 1) {
                if self.increment(x + 1, y) > 9 {
                    queue.push_back((x + 1, y));
                }
            }
            if y < (self.height - 1) {
                if x > 0 {
                    if self.increment(x - 1, y + 1) > 9 {
                        queue.push_back((x - 1, y + 1));
                    }
                }
                if self.increment(x, y + 1) > 9 {
                    queue.push_back((x, y + 1));
                }
                if x < (self.width - 1) {
                    if self.increment(x + 1, y + 1) > 9 {
                        queue.push_back((x + 1, y + 1));
                    }
                }
            }
        }
        self.zero_flashed_items();
        flashes
    }

    fn set(&mut self, x: usize, y: usize, value: i8) -> i8 {
        self.energy_levels[y * self.width + x] = value;
        value
    }

    fn zero_flashed_items(&mut self) {
        for index in 0..self.energy_levels.len() {
            if self.energy_levels[index] == -1 {
                self.energy_levels[index] = 0;
            }
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}\t", self.get(x, y));
            }
            print!("\n");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut board = Board::parse(stdin.lock().lines());
    // Simple solution
    // let mut total_flashes = 0;
    // for _ in 0..100 {
    //     total_flashes += board.run_step();
    // }
    // println!("Result: {}", total_flashes);

    // Complex solution
    let mut step = 1;
    loop {
        let flashes = board.run_step();
        if flashes == board.energy_levels.len() as u32 {
            println!("Step: {}", step);
            break;
        }
        step += 1;
    }
}

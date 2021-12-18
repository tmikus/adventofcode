use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn line_to_i32(line: Result<String, Error>) -> i32 {
    line.unwrap().parse::<i32>().unwrap()
}

fn read_next_i32(lines: &mut Lines<StdinLock>) -> i32 {
    line_to_i32(lines.next().unwrap())
}

fn simple_solution() {
    let mut increased_count = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut last_value = read_next_i32(&mut lines);
    for line in lines {
        let value = line_to_i32(line);
        if last_value < value {
            increased_count += 1;
        }
        last_value = value;
    }
    println!("Increased count: {}", increased_count);
}

fn sliding_window_solution() {
    let mut increased_count = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut a1 = read_next_i32(&mut lines);
    let mut a2 = read_next_i32(&mut lines);
    let mut a3 = read_next_i32(&mut lines);
    let mut b1 = a2;
    let mut b2 = a3;
    let mut b3 = read_next_i32(&mut lines);
    loop {
        let a_count = a1 + a2 + a3;
        let b_count = b1 + b2 + b3;
        if b_count > a_count {
            increased_count += 1;
        }
        if let Some(line) = lines.next() {
            a1 = a2;
            a2 = a3;
            a3 = b3;
            b1 = b2;
            b2 = b3;
            b3 = line_to_i32(line);
        } else {
            break;
        }
    }
    println!("Increased count: {}", increased_count);
}

fn main() {
   // simple_solution();
    sliding_window_solution();
}

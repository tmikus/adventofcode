use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

enum Operation {
    Down(i32),
    Up(i32),
    Forward(i32),
    Backward(i32),
}

fn parse_operation(line: String) -> Operation {
    let mut split = line.split_whitespace();
    let key = split.next().unwrap();
    let value = split.next().unwrap().parse::<i32>().unwrap();
    if key.starts_with("f") {
        Operation::Forward(value)
    }
    else if key.starts_with("b") {
        Operation::Backward(value)
    }
    else if key.starts_with("d") {
        Operation::Down(value)
    }
    else {
        Operation::Up(value)
    }
}

fn simple_solution() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    for line in lines {
        let operation = parse_operation(line.unwrap());
        match operation {
            Operation::Down(value) => depth += value,
            Operation::Up(value) => depth -= value,
            Operation::Backward(value) => horizontal_position -= value,
            Operation::Forward(value) => horizontal_position += value,
        }
    }
    println!("position * depth = {}", horizontal_position * depth);
}

fn complex_solution() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    for line in lines {
        let operation = parse_operation(line.unwrap());
        match operation {
            Operation::Down(value) => aim += value,
            Operation::Up(value) => aim -= value,
            Operation::Backward(value) => {
                horizontal_position -= value;
                depth -= aim * value;
            },
            Operation::Forward(value) => {
                horizontal_position += value;
                depth += aim * value;
            },
        }
    }
    println!("position * depth = {}", horizontal_position * depth);
}

fn main() {
    complex_solution();
}

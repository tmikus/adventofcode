use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct SimpleResult {
    epsilon: i64,
    gamma: i64,
}

fn init_bit_counts(line: &str) -> Vec<i32> {
    let length = line.len();
    let mut bit_counts = Vec::with_capacity(length);
    for _ in 0..length {
        bit_counts.push(0);
    }
    bit_counts
}

fn update_counts(bit_counts: &mut Vec<i32>, line: String) {
    for (index, char) in line.chars().enumerate() {
        bit_counts[index] += if char == '1' {
            1
        } else {
            -1
        };
    }
}

fn calculate_simple_result(bit_counts: &Vec<i32>) -> SimpleResult {
    let mut epsilon = 0;
    let mut gamma = 0;
    let count = bit_counts.len();
    for index in 0..count {
        let bit_value = 1 << (count - 1 - index);
        if bit_counts[index] > 0 {
            gamma += bit_value;
        } else {
            epsilon += bit_value;
        }
    }
    SimpleResult {
        epsilon,
        gamma,
    }
}

fn simple_solution() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap();
    let mut bit_counts = init_bit_counts(&line);
    loop {
        update_counts(&mut bit_counts, line);
        if let Some(next_line) = lines.next() {
            line = next_line.unwrap();
        } else {
            break;
        }
    }
    let result = calculate_simple_result(&bit_counts);
    println!("Result: {}", result.gamma * result.epsilon);
}

fn main() {
    simple_solution()
}

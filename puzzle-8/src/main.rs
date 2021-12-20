use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct Line {
    signal_patterns: [String; 10],
    output_values: [String; 4],
}

impl Line {
    fn new() -> Line {
        Line {
            signal_patterns: Default::default(),
            output_values: Default::default(),
        }
    }

    fn parse(value: String) -> Line {
        let mut parts = value.split(" | ");
        let mut line = Line::new();
        let mut signal_patterns = parts.next().unwrap().split(' ');
        let mut output_values = parts.next().unwrap().split(' ');
        for (index, pattern) in signal_patterns.enumerate() {
            line.signal_patterns[index] = pattern.to_owned();
        }
        for (index, pattern) in output_values.enumerate() {
            line.output_values[index] = pattern.to_owned();
        }
        line
    }
}

fn count_simple_digits(line: &Line) -> u32 {
    let mut count = 0;
    for pattern in line.output_values.iter() {
        let len = pattern.len();
        if len == 2 || len == 3 || len == 4 || len == 7 {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut count = 0;
    for input in stdin.lock().lines() {
        let line = Line::parse(input.unwrap());
        count += count_simple_digits(&line);
    }
    println!("Count: {}", count);
}

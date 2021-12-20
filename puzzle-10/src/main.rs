use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn simple_solution(line: String) -> u64 {
    let mut valid_closing_braces = VecDeque::new();
    let mut result = 0;
    for brace in line.chars() {
        if brace == '(' {
            valid_closing_braces.push_back(')');
            continue;
        }
        if brace == '[' {
            valid_closing_braces.push_back(']');
            continue;
        }
        if brace == '{' {
            valid_closing_braces.push_back('}');
            continue;
        }
        if brace == '<' {
            valid_closing_braces.push_back('>');
            continue;
        }
        match valid_closing_braces.pop_back() {
            Some(expected_closing_brace) => {
                if brace != expected_closing_brace {
                    result = match brace {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    break;
                }
            },
            None => break,
        }
    }
    result
}

fn complex_solution(line: String) -> u64 {
    let mut valid_closing_braces = VecDeque::new();
    for brace in line.chars() {
        if brace == '(' {
            valid_closing_braces.push_back(')');
            continue;
        }
        if brace == '[' {
            valid_closing_braces.push_back(']');
            continue;
        }
        if brace == '{' {
            valid_closing_braces.push_back('}');
            continue;
        }
        if brace == '<' {
            valid_closing_braces.push_back('>');
            continue;
        }
        match valid_closing_braces.pop_back() {
            Some(expected_closing_brace) => {
                if brace != expected_closing_brace {
                    return 0;
                }
            },
            None => break,
        }
    }
    let mut score = 0;
    loop {
        match valid_closing_braces.pop_back() {
            Some(brace) => {
                score *= 5;
                score += match brace {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                };
            },
            None => break,
        }
    }
    score
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Simple solution
    // let mut count = 0;
    // for line in lines {
    //     count += simple_solution(line.unwrap());
    // }
    let mut scores = vec![];
    for line in lines {
        let result = complex_solution(line.unwrap());
        if result == 0 {
            continue;
        }
        scores.push(result);
    }
    scores.sort();
    println!("{:?}", scores);
    println!("{}", scores[scores.len() / 2]);
}

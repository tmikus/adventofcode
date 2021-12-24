use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct InsertionRule {
    left: u8,
    right: u8,
    value: u8,
}

impl InsertionRule {
    fn parse(value: &str) -> InsertionRule {
        let mut parts = value.split(" -> ");
        let from = parts.next().unwrap().as_bytes();
        let to = parts.next().unwrap().as_bytes();
        InsertionRule {
            left: from[0],
            right: from[1],
            value: to[0],
        }
    }
}

fn find_rule(left: u8, right: u8, rules: &Vec<InsertionRule>) -> Option<&InsertionRule> {
    for rule in rules {
        if rule.left == left && rule.right == right {
            return Some(rule);
        }
    }
    None
}

fn get_item_index(items: &mut Vec<u8>, item_to_find: u8) -> usize {
    for (index, item) in items.iter().enumerate() {
        if *item == item_to_find {
            return index;
        }
    }
    let index = items.len();
    items.push(item_to_find);
    index
}

fn get_result(pattern: Vec<u8>) -> u64 {
    let mut items = vec![];
    let mut counts = vec![];
    for item in pattern {
        let index = get_item_index(&mut items, item);
        if index == counts.len() {
            counts.push(1)
        } else {
            counts[index] += 1;
        }
    }
    let largest = counts.iter().max().unwrap();
    let smallest = counts.iter().min().unwrap();
    largest - smallest
}

fn run_iteration(pattern: Vec<u8>, rules: &Vec<InsertionRule>) -> Vec<u8> {
    let mut result = vec![pattern[0]];
    for i in 1..pattern.len() {
        let left = pattern[i - 1];
        let right = pattern[i];
        let rule = find_rule(left, right, rules);
        if let Some(rule) = rule {
            result.push(rule.value);
        }
        result.push(right);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut pattern = lines.next().unwrap().unwrap().as_bytes().to_vec();
    lines.next();
    let mut rules = vec![];
    for line in lines {
        let line_str = line.unwrap();
        rules.push(InsertionRule::parse(&line_str));
    }
    for iteration in 0..40 {
        println!("Running iteration {}", iteration);
        pattern = run_iteration(pattern, &rules);
    }
    println!("Result: {}", get_result(pattern));
}

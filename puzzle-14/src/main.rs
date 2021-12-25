use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct InsertionRule {
    from: String,
    to_left: String,
    to_right: String,
}

impl InsertionRule {
    fn parse(value: &str) -> InsertionRule {
        let mut parts = value.split(" -> ");
        let from: Vec<char> = parts.next().unwrap().chars().collect();
        let to: Vec<char> = parts.next().unwrap().chars().collect();
        InsertionRule {
            from: format!("{}{}", from[0], from[1]),
            to_left: format!("{}{}", from[0], to[0]),
            to_right: format!("{}{}", to[0], from[1]),
        }
    }
}

fn find_rule<'a>(pattern: &str, rules: &'a Vec<InsertionRule>) -> &'a InsertionRule {
    for rule in rules {
        if rule.from == pattern {
            return rule;
        }
    }
    &rules[0]
}

fn get_result(patterns: HashMap<String, u64>, last_char: char) -> u64 {
    let mut letter_counts = HashMap::new();
    for (pattern, count) in &patterns {
        let pattern: Vec<char> = pattern.chars().collect();
        let left = pattern[0];
        if let Some(existing_count) = letter_counts.get_mut(&left) {
            *existing_count += *count;
        } else {
            letter_counts.insert(left, *count);
        }
    }
    if let Some(existing_count) = letter_counts.get_mut(&last_char) {
        *existing_count += 1;
    } else {
        letter_counts.insert(last_char, 1);
    }
    let highest = letter_counts.values().max().unwrap();
    let smallest = letter_counts.values().min().unwrap();
    highest - smallest
}

fn parse_patterns(raw_pattern: String) -> HashMap<String, u64> {
    let mut patterns = HashMap::new();
    let chars: Vec<char> = raw_pattern.chars().collect();
    for i in 1..chars.len() {
        let pattern_key = format!("{}{}", chars[i - 1], chars[i]);
        if let Some(pattern) = patterns.get_mut(&pattern_key) {
           *pattern += 1;
        } else {
            patterns.insert(pattern_key, 1);
        }
    }
    patterns
}

fn run_iteration(
    patterns: HashMap<String, u64>,
    rules: &Vec<InsertionRule>
) -> HashMap<String, u64> {
    let mut new_patterns = HashMap::new();
    for (pattern, count) in patterns {
        let rule = find_rule(&pattern, rules);
        if let Some(existing_count) = new_patterns.get_mut(&rule.to_left) {
            *existing_count += count;
        } else {
            new_patterns.insert(rule.to_left.to_owned(), count);
        }
        if let Some(existing_count) = new_patterns.get_mut(&rule.to_right) {
            *existing_count += count;
        } else {
            new_patterns.insert(rule.to_right.to_owned(), count);
        }
    }
    new_patterns
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let pattern = lines.next().unwrap().unwrap();
    let last_char = pattern.chars().last().unwrap();
    let mut patterns = parse_patterns(pattern);
    lines.next();
    let mut rules = vec![];
    for line in lines {
        let line_str = line.unwrap();
        rules.push(InsertionRule::parse(&line_str));
    }
    for iteration in 0..40 {
        println!("Running iteration {}", iteration);
        patterns = run_iteration(patterns, &rules);
    }
    println!("Result: {}", get_result(patterns, last_char));
}

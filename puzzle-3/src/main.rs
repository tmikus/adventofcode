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

fn update_counts(bit_counts: &mut Vec<i32>, line: &str) {
    let line_bytes = line.as_bytes();
    for index in 0..line.len() {
        bit_counts[index] += if line_bytes[index] == b'1' {
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
        update_counts(&mut bit_counts, &line);
        if let Some(next_line) = lines.next() {
            line = next_line.unwrap();
        } else {
            break;
        }
    }
    let result = calculate_simple_result(&bit_counts);
    println!("Result: {}", result.gamma * result.epsilon);
}

fn copy_vec<'a>(original: &'a Vec<String>) -> Vec<&'a str> {
    let mut copy = Vec::with_capacity(original.len());
    for value in original {
        copy.push(value.as_ref());
    }
    copy
}

fn clean_ratings(ratings: Vec<&str>, index: usize, expected_value: u8) -> Vec<&str> {
    let mut result = Vec::new();
    for rating in ratings {
        if rating.as_bytes()[index] != expected_value {
            continue;
        }
        result.push(rating);
    }
    result
}

fn count_positive_bits(numbers: &Vec<&str>, bit_index: usize) -> i32 {
    let mut positive_bits = 0;
    for number in numbers {
        positive_bits += if number.as_bytes()[bit_index] == b'1' { 1 } else { -1 };
    }
    positive_bits
}

fn find_oxygen_rating(numbers: &Vec<String>, bit_count: usize) -> i32 {
    let mut ratings = copy_vec(numbers);
    for bit_index in 0..bit_count {
        if ratings.len() == 1 {
            break;
        }
        let positive_bits = count_positive_bits(&ratings, bit_index);
        let expected_bit = if positive_bits >= 0 { b'1' } else { b'0' };
        ratings = clean_ratings(ratings, bit_index, expected_bit);
    }
    println!("Oxygen ratings: {:?}", ratings);
    let oxygen_rating = ratings.first().unwrap();
    println!("Oxygen rating: {}", oxygen_rating);
    str_to_i32(oxygen_rating)
}

fn find_scrubber_rating(numbers: &Vec<String>, bit_count: usize) -> i32 {
    let mut ratings = copy_vec(numbers);
    for bit_index in 0..bit_count {
        if ratings.len() == 1 {
            break;
        }
        let positive_bits = count_positive_bits(&ratings, bit_index);
        let expected_bit = if positive_bits >= 0 { b'0' } else { b'1' };
        ratings = clean_ratings(ratings, bit_index, expected_bit);
    }
    println!("Oxygen ratings: {:?}", ratings);
    let oxygen_rating = ratings.first().unwrap();
    println!("Oxygen rating: {}", oxygen_rating);
    str_to_i32(oxygen_rating)
}

fn str_to_i32(value: &str) -> i32 {
    let value_bytes = value.as_bytes();
    let mut number = 0;
    let count = value.len();
    for index in 0..count {
        let bit_value = 1 << (count - 1 - index);
        if value_bytes[index] == b'1' {
            number += bit_value;
        }
    }
    number
}

fn complex_solution() {
    let stdin = io::stdin();
    let mut numbers: Vec<String> = Vec::new();
    let mut lines = stdin.lock().lines();
    let mut bit_count = 0;
    for line in lines {
        let line_str = line.unwrap();
        bit_count = line_str.len();
        numbers.push(line_str);
    }
    let oxygen_rating = find_oxygen_rating(&numbers, bit_count);
    let scrubber_rating = find_scrubber_rating(&numbers, bit_count);
    println!("Result: {}", oxygen_rating * scrubber_rating);
}

fn main() {
    complex_solution();
}

use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct Line {
    digits: Vec<Vec<u8>>,
    output_values: Vec<Vec<u8>>,
}

impl Line {
    fn new() -> Line {
        Line {
            digits: vec![],
            output_values: vec![],
        }
    }

    fn parse(value: String) -> Line {
        let mut parts = value.split(" | ");
        let mut line = Line::new();
        let mut digits = parts.next().unwrap().split(' ');
        let mut output_values = parts.next().unwrap().split(' ');
        for pattern in digits {
            line.digits.push(pattern.as_bytes().to_owned());
        }
        for pattern in output_values {
            line.output_values.push(pattern.as_bytes().to_owned());
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

fn a_equals_b(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    a.len() == b.len() && a.iter().all(|number| b.contains(number))
}

fn a_includes_b(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    b.iter().all(|number| a.contains(number))
}

fn find_one_four_seven_eight(line: &mut Line) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut one = None;
    let mut four = None;
    let mut seven = None;
    let mut eight = None;
    let mut remaining_digits = vec![];
    for digit in &line.digits {
        if digit.len() == 2 {
            one = Some(digit.to_owned());
        } else if digit.len() == 3 {
            seven = Some(digit.to_owned());
        } else if digit.len() == 4 {
            four = Some(digit.to_owned());
        } else if digit.len() == 7 {
            eight = Some(digit.to_owned());
        } else {
            remaining_digits.push(digit.to_owned());
        }
    }
    line.digits = remaining_digits;
    (one.unwrap(), four.unwrap(), seven.unwrap(), eight.unwrap())
}

fn find_zero_three_nine(line: &mut Line, four: &Vec<u8>, seven: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut zero = None;
    let mut three = None;
    let mut nine = None;
    let mut remaining_digits = vec![];
    for digit in &line.digits {
        if digit.len() == (four.len() + 2) && a_includes_b(digit, four) {
            nine = Some(digit.to_owned());
        } else if digit.len() == (seven.len() + 3) && a_includes_b(digit, seven) {
            zero = Some(digit.to_owned());
        } else if digit.len() == (seven.len() + 2) && a_includes_b(digit, seven) {
            three = Some(digit.to_owned());
        } else {
            remaining_digits.push(digit.to_owned());
        }
    }
    line.digits = remaining_digits;
    (zero.unwrap(), three.unwrap(), nine.unwrap())
}

fn find_six(line: &mut Line) -> Vec<u8> {
    let mut six = None;
    let mut remaining_digits = vec![];
    for digit in &line.digits {
        if digit.len() == 6 {
            six = Some(digit.to_owned());
        } else {
            remaining_digits.push(digit.to_owned());
        }
    }
    line.digits = remaining_digits;
    six.unwrap()
}

fn find_five(line: &mut Line, six: &Vec<u8>) -> Vec<u8> {
    let mut five = None;
    let mut remaining_digits = vec![];
    for digit in &line.digits {
        if digit.len() == 5 && a_includes_b(six, digit) {
            five = Some(digit.to_owned());
        } else {
            remaining_digits.push(digit.to_owned());
        }
    }
    line.digits = remaining_digits;
    five.unwrap()
}

fn find_digit(value: &Vec<u8>, digits: &[Vec<u8>; 10]) -> u32 {
    let mut result = None;
    for (digit, bytes) in digits.iter().enumerate() {
        if a_equals_b(bytes, value) {
            result = Some(digit as u32);
            break;
        }
    }
    if result.is_none() {
        println!("Value: {}", std::str::from_utf8(value.as_slice()).unwrap());
        println!("Digits:");
        for (digit, value) in digits.iter().enumerate() {
            println!("{}: {}", digit, std::str::from_utf8(value.as_slice()).unwrap());
        }
    }
    result.unwrap()
}

fn value_to_u32(line: &Line, digits: [Vec<u8>; 10]) -> u32 {
    let mut str = String::new();
    for value in &line.output_values {
        let digit = find_digit(value, &digits).to_string();
        str += &digit;
    }
    str.parse::<u32>().unwrap()
}

fn get_output_value(mut line: Line) -> u32 {
    let (one, four, seven, eight) = find_one_four_seven_eight(&mut line);
    let (zero, three, nine) = find_zero_three_nine(&mut line, &four, &seven);
    let six = find_six(&mut line);
    let five = find_five(&mut line, &six);
    let two = line.digits.first().unwrap().to_owned();
    let digits = [zero, one, two, three, four, five, six, seven, eight, nine];
    // println!("Digits:");
    // for (digit, value) in digits.iter().enumerate() {
    //     println!("{}: {}", digit, std::str::from_utf8(value.as_slice()).unwrap());
    // }
    value_to_u32(&line, digits)
}

fn main() {
    let stdin = io::stdin();
    let mut count = 0;
    for input in stdin.lock().lines() {
        let line = Line::parse(input.unwrap());
        // count += count_simple_digits(&line); // Simple solution
        count += get_output_value(line);
    }
    println!("Count: {}", count);
}

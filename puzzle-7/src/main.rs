use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn compute_fuel_expenditure(positions: Vec<u16>, median: u16) -> u32 {
    let mut fuel_used = 0;
    for position in positions {
        fuel_used += (position as i32 - median as i32).abs() as u32;
    }
    fuel_used
}

fn compute_median(positions: &mut Vec<u16>) -> u16 {
    positions.sort();
    let middle = positions.len() / 2;
    positions[middle]
}

fn parse_positions(line: String) -> Vec<u16> {
    line.split(',').map(|value| value.parse().unwrap()).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut positions = parse_positions(lines.next().unwrap().unwrap());
    let median = compute_median(&mut positions);
    let fuel_expenditure = compute_fuel_expenditure(positions, median);
    println!("Result: {}", fuel_expenditure);
}

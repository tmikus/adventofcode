use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn compute_complex_fuel_expenditure(positions: &Vec<u32>, ideal_position: u32) -> u32 {
    let mut fuel_used = 0;
    for position in positions {
        let diff = (*position as i32 - ideal_position as i32).abs() as u32;
        for num in 1..=diff {
            fuel_used += num;
        }
    }
    fuel_used
}

fn compute_simple_fuel_expenditure(positions: Vec<u32>, ideal_position: u32) -> u32 {
    let mut fuel_used = 0;
    for position in positions {
        fuel_used += (position as i32 - ideal_position as i32).abs() as u32;
    }
    fuel_used
}

fn compute_median(positions: &mut Vec<u32>) -> u32 {
    positions.sort();
    let middle = positions.len() / 2;
    positions[middle]
}

fn optimise_position(positions: Vec<u32>) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut min_fuel_used = u32::MAX;
    for position in min..=max {
        println!("{}/{}", position, max);
        let fuel_used = compute_complex_fuel_expenditure(&positions, position);
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
        }
    }
    min_fuel_used
}

fn parse_positions(line: String) -> Vec<u32> {
    line.split(',').map(|value| value.parse().unwrap()).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut positions = parse_positions(lines.next().unwrap().unwrap());
    // let ideal_position = compute_median(&mut positions); // Simple Solution
    // let fuel_expenditure = compute_simple_fuel_expenditure(positions, ideal_position);
    let fuel_expenditure = optimise_position(positions);
    println!("Result: {}", fuel_expenditure);
}

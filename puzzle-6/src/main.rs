use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn parse_fish_age(line: String) -> Vec<u8> {
    line.split(',').map(|value| value.parse::<u8>().unwrap()).collect()
}

fn update_fish(fish: Vec<u8>) -> Vec<u8> {
    let mut next_gen_fish = vec![];
    for current in fish {
        if current == 0 {
            next_gen_fish.push(6); // Current
            next_gen_fish.push(8); // New
        } else {
            next_gen_fish.push(current - 1);
        }
    }
    next_gen_fish
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fish_age = parse_fish_age(lines.next().unwrap().unwrap());
    for _ in 0..80 {
        fish_age = update_fish(fish_age);
    }
    println!("Fish count: {}", fish_age.len());
}

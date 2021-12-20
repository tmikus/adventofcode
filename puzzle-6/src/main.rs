use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

fn count_fish(generations: [u64; 9]) -> u64 {
    let mut total_count = 0;
    for count in generations {
        total_count += count;
    }
    total_count
}

fn parse_generations(line: String) -> [u64; 9] {
    let mut generations = [0; 9];
    let ages: Vec<usize> = line.split(',').map(|value| value.parse().unwrap()).collect();
    for age in ages {
        generations[age] += 1;
    }
    generations
}

fn update_fish(generations: [u64; 9]) -> [u64; 9] {
    let mut next_generations = [0; 9];
    for (age, count) in generations.iter().enumerate() {
        if age == 0 {
            next_generations[6] = *count;
            next_generations[8] = *count;
        } else {
            next_generations[age - 1] += count;
        }
    }
    next_generations
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fish_age = parse_generations(lines.next().unwrap().unwrap());
    for day in 0..256 {
        println!("Computing day {}...", day);
        fish_age = update_fish(fish_age);
    }
    println!("Fish count: {}", count_fish(fish_age));
}

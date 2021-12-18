use std::io;
use std::io::prelude::*;

fn simple_solution() {
    let mut increased_count = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut last_value = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    for line in lines {
        let value = line.unwrap().parse::<i32>().unwrap();
        if last_value < value {
            increased_count += 1;
        }
        last_value = value;
    }
    println!("Increased count: {}", increased_count);
}

fn main() {
   simple_solution();
}

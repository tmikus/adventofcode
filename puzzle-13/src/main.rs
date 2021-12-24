use std::cmp::max;
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(value: String) -> Point {
        let mut numbers = value.split(',');
        let x = numbers.next().unwrap().parse().unwrap();
        let y = numbers.next().unwrap().parse().unwrap();
        Point { x, y }
    }
}

fn parse_points(lines: &mut Lines<StdinLock>) -> Vec<Point> {
    let mut points = vec![];
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        points.push(Point::parse(line));
    }
    points
}

struct Fold {
    along_x: bool,
    along_y: bool,
    value: usize,
}

impl Fold {
    fn parse(value: String) -> Fold {
        let mut words = value.split(' ');
        words.next();
        words.next();
        let mut coords = words.next().unwrap().split('=');
        let axis = coords.next().unwrap();
        let value = coords.next().unwrap().parse().unwrap();
        Fold {
            along_x: axis == "x",
            along_y: axis == "y",
            value,
        }
    }
}

fn parse_folds(lines: &mut Lines<StdinLock>) -> Vec<Fold> {
    let mut folds = vec![];
    for line in lines {
        folds.push(Fold::parse(line.unwrap()));
    }
    folds
}

struct Board {
    data: Vec<bool>,
    height: usize,
    width: usize,
}

impl Board {
    fn count_dots(&self) -> u32 {
        let mut count = 0;
        for value in &self.data {
            if *value {
                count += 1;
            }
        }
        count
    }

    fn fold_on(&self, fold: Fold) -> Board {
        self.print_with_fold(&fold);
        if fold.along_x {
            self.fold_along_x(fold.value)
        } else {
            self.fold_along_y(fold.value)
        }
    }

    fn fold_along_y(&self, fold_index: usize) -> Board {
        let bottom_fold_height = self.height - fold_index - 1;
        let new_height = max(fold_index, bottom_fold_height);
        let mut data = vec![false; self.width * new_height];
        // Copy top fold
        let offset = new_height - fold_index;
        for y in 0..fold_index {
            for x in 0..self.width {
                data[(y + offset) * self.width + x] = self.data[y * self.width + x];
            }
        }
        // Copy bottom fold
        let offset = new_height - bottom_fold_height;
        for y in 0..bottom_fold_height {
            for x in 0..self.width {
                data[(y + offset) * self.width + x] |= self.data[(self.height - 1 - y) * self.width + x];
            }
        }
        Board {
            data,
            height: new_height,
            width: self.width,
        }
    }

    fn fold_along_x(&self, fold_index: usize) -> Board {
        let right_fold_width = self.width - fold_index - 1;
        let new_width = max(fold_index, right_fold_width);
        let mut data = vec![false; new_width * self.height];
        for y in 0..self.height {
            // Copy left fold
            let offset = new_width - fold_index;
            for x in 0..fold_index {
                data[y * new_width + offset + x] = self.data[y * self.width + x];
            }
            // Copy right fold
            let offset = new_width - right_fold_width;
            for x in 0..right_fold_width {
                data[y * new_width + offset + x] |= self.data[y * self.width + self.width - x - 1];
            }
        }
        Board {
            data,
            height: self.height,
            width: new_width,
        }
    }

    fn from_points(points: Vec<Point>) -> Board {
        let (width, height) = Self::get_dimensions(&points);
        let mut data = vec![false; width * height];
        for point in points {
            data[point.y * width + point.x] = true;
        }
        Board {
            data,
            height,
            width,
        }
    }

    fn get_dimensions(points: &Vec<Point>) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for point in points {
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
        (max_x + 1, max_y + 1)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = match self.data[y * self.width + x] {
                    false => " ",
                    true => "#",
                };
                print!("{}", value);
            }
            print!("\n");
        }
    }

    fn print_with_fold(&self, fold: &Fold) {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = if fold.along_y && fold.value == y {
                    "-"
                } else if fold.along_x && fold.value == x {
                    "|"
                } else {
                    match self.data[y * self.width + x] {
                        false => " ",
                        true => "#",
                    }
                };
                print!("{}", value);
            }
            print!("\n");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let points = parse_points(&mut lines);
    let folds = parse_folds(&mut lines);
    let mut board = Board::from_points(points);
    for fold in folds {
        println!("\nFolding");
        board = board.fold_on(fold);
        println!("\nAfter fold:");
        board.print();
    }
    println!("Remaining dots: {}", board.count_dots());
}

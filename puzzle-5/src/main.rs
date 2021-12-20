use std::cmp::{max, min};
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};

#[derive(Debug)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point {
      x,
      y,
    }
  }

  fn parse(value: &str) -> Point {
    let mut numbers = value.split(',');
    let x = numbers.next().unwrap().parse().unwrap();
    let y = numbers.next().unwrap().parse().unwrap();
    Point::new(x, y)
  }
}

struct Line {
  from: Point,
  to: Point,
}

impl Line {
  fn new(from: Point, to: Point) -> Line {
    Line {
      from,
      to,
    }
  }

  fn parse(value: String) -> Line {
    let mut points_data = value.split(" -> ");
    let from = Point::parse(points_data.next().unwrap());
    let to = Point::parse(points_data.next().unwrap());
    Self::new(from, to)
  }
}

#[derive(Debug)]
struct Dimensions {
  height: usize,
  width: usize,
}

impl Dimensions {
  fn new(width: usize, height: usize) -> Dimensions {
    Dimensions {
      height,
      width,
    }
  }

  fn find_dimensions(lines: &Vec<Line>) -> Dimensions {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
      if line.from.x > max_x {
        max_x = line.from.x;
      }
      if line.from.y > max_y {
        max_y = line.from.y;
      }
      if line.to.x > max_x {
        max_x = line.to.x;
      }
      if line.to.y > max_y {
        max_y = line.to.y;
      }
    }
    Dimensions::new(max_x + 1, max_y + 1)
  }
}

struct Board {
  dimensions: Dimensions,
  data: Vec<u32>,
}

impl Board {
  fn new(dimensions: Dimensions) -> Board {
    let data = vec![0; dimensions.width * dimensions.height];
    Board {
      dimensions,
      data,
    }
  }

  fn draw_line(&mut self, line: Line) {
    let delta_x = (
      if line.from.x > line.to.x { -1 }
      else if line.from.x < line.to.x { 1 }
      else { 0 }
    );
    let delta_y = (
      if line.from.y > line.to.y { -1 }
      else if line.from.y < line.to.y { 1 }
      else { 0 }
    );
    let mut x = line.from.x;
    let mut y = line.from.y;
    loop {
      self.data[self.dimensions.width * y + x] += 1;
      if delta_x != 0 && x == line.to.x {
        break;
      }
      if delta_y != 0 && y == line.to.y {
        break;
      }
      x = (x as i32 + delta_x) as usize;
      y = (y as i32 + delta_y) as usize;
    }
  }

  fn get_result(&self) -> u32 {
    let mut count = 0;
    for number in &self.data {
      if *number >= 2 {
        count += 1;
      }
    }
    count
  }

  fn print_board(&self) {
    for height in 0..=self.dimensions.height {
      for width in 0..=self.dimensions.width {
        let point = self.data[height * self.dimensions.width + width];
        if point > 0 {
          print!("{}", point);
        } else {
          print!(" ");
        }
      }
      print!("\n");
    }
  }
}

fn main() {
  let stdin = io::stdin();
  let mut lines = Vec::new();
  for line in stdin.lock().lines() {
    lines.push(Line::parse(line.unwrap()));
  }
  let dimensions = Dimensions::find_dimensions(&lines);
  let mut board = Board::new(dimensions);
  for line in lines {
    board.draw_line(line);
  }
  // board.print_board();
  println!("Result: {}", board.get_result());
}

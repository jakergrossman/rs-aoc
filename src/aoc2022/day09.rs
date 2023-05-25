use crate::{aoclib::day::*, run_day};
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i128,
    y: i128
}

impl Point {
    const MOVE_THRESHOLD: i128 = 2;

    fn from_str(dir: &str, distance: &str) -> Self {
        match dir {
            "U" => Self { x: 0, y: i128::from_str_radix(distance, 10).unwrap() },
            "D" => Self { x: 0, y: -i128::from_str_radix(distance, 10).unwrap() },
            "L" => Self { x: -i128::from_str_radix(distance, 10).unwrap(), y: 0 },
            "R" => Self { x: i128::from_str_radix(distance, 10).unwrap(), y: 0 },
            _ => panic!("Invalid input")
        }
    }

    fn normal(&self) -> Self {
        Self {
            x: match self.x.signum() {
                0 => 0,
                s => self.x / self.x * s
            },
            y: match self.y.signum() {
                0 => 0,
                s => self.y / self.y * s
            },
        }
    }

    fn mag(&self) -> i128 {
        self.x.abs().max(self.y.abs())
    }

    fn distance_sq(&self, other: &Self) -> i128 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).abs()
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Mul<i128> for Point {
    type Output = Point;

    fn mul(self, rhs: i128) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

fn serialize<'a>(s: String) -> Vec<Point> {
    s.lines().map(|line| {
        let mut tokens = line.split_ascii_whitespace();
        Point::from_str(tokens.next().unwrap(), tokens.next().unwrap())
    }).collect()
}

pub fn run(is_sample: bool) {
    run_day!(2022, 9, is_sample, serialize, (part1));
}

fn part1(directions: Vec<Point>) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut tail_visited = HashSet::new();
    tail_visited.insert(tail);

    for d in directions {
        let norm = d.normal();
        let mag = d.mag();

        for _ in 0..mag {
            let new_head = head + norm;
            let distance = new_head.distance_sq(&tail);

            if  distance > Point::MOVE_THRESHOLD {
                // need to move tail to where head just was
                tail = head;
                tail_visited.insert(tail);
            }
            head = new_head;
        }
    }

    tail_visited.len()
}

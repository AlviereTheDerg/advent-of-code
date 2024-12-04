
use std::{env, fs};

mod day01;
mod day02;
mod day03;
mod day04;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    pub x: isize,
    pub y: isize,
}
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Sub for Coord {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Mul<isize> for Coord {
    type Output = Self;
    fn mul(self, other: isize) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn grab_input(input_name: &str) -> String {
    fs::read_to_string(format!("./inputs/{input_name}.txt")).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).expect("Missing day!")
        .parse::<isize>().expect("Unable to parse day!");

    match day {
        1 => { day01::main(); },
        2 => { day02::main(); },
        3 => { day03::main(); },
        4 => { day04::main(); },
        _ => { println!("Day not recognized!"); },
    }
}

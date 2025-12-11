
use std::{env, fs, hash::Hash};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
// mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).expect("Missing day!")
        .parse::<isize>().expect("Unable to parse day!");

    match day {
        1 => { day01::main(); },
        2 => { day02::main(); },
        3 => { day03::main(); },
        4 => { day04::main(); },
        5 => { day05::main(); },
        6 => { day06::main(); },
        7 => { day07::main(); },
        8 => { day08::main(); },
        9 => { day09::main(); },
        10 => { day10::main(); },
        11 => { day11::main(); },
        // 12 => { day12::main(); },
        _ => { println!("Day not recognized!"); },
    }
}


#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
#[allow(dead_code)]
impl Coord {
    fn within_bounds(&self, outer_bounds: &Coord) -> bool {
        self.x >= 0 &&
        self.x < outer_bounds.x &&
        self.y >= 0 &&
        self.y < outer_bounds.y
    }
}
#[allow(dead_code)]
trait New<X,Y> {
    fn new(x:X, y:Y) -> Self;
}
impl New<isize, isize> for Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord{x, y}
    }
}
impl New<isize, usize> for Coord {
    fn new(x: isize, y: usize) -> Coord {
        Coord{x, y:y as isize}
    }
}
impl New<usize, isize> for Coord {
    fn new(x: usize, y: isize) -> Coord {
        Coord{x:x as isize, y}
    }
}
impl New<usize, usize> for Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord{x:x as isize, y:y as isize}
    }
}

fn grab_input(input_name: &str) -> String {
    fs::read_to_string(format!("./inputs/{input_name}.txt")).unwrap()
}
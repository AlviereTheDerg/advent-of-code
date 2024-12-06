
use std::{collections::{HashMap, HashSet}, env, fs, hash::Hash};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

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
impl Coord {
    fn within_bounds(&self, outer_bounds: &Coord) -> bool {
        self.x >= 0 &&
        self.x < outer_bounds.x &&
        self.y >= 0 &&
        self.y < outer_bounds.y
    }
}

struct Graph<NodeType>
where 
    NodeType: Hash + Eq + Clone,
{
    data: HashMap<NodeType, HashSet<NodeType>>,
}
impl<NodeType> Graph<NodeType>
where 
    NodeType: Hash + Eq + Clone,
{
    pub fn add_node(&mut self, node: NodeType) {
        self.data.entry(node).or_insert(HashSet::new());
    }

    pub fn has_node(&self, node: &NodeType) -> bool {
        self.data.contains_key(node)
    }

    pub fn add_edge(&mut self, start: NodeType, end: NodeType) {
        self.add_node(start.clone()); self.add_node(end.clone());
        self.data.get_mut(&start).unwrap().insert(end);
    }

    pub fn has_edge(&self, start: &NodeType, end: &NodeType) -> bool {
        self.has_node(start) &&
        self.has_node(end) &&
        self.data.get(start).unwrap().contains(end)
    }

    pub fn remove_edge(&mut self, start: &NodeType, end: &NodeType) {
        self.data.get_mut(start).unwrap().remove(end);
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
        5 => { day05::main(); },
        6 => { day06::main(); },
        _ => { println!("Day not recognized!"); },
    }
}

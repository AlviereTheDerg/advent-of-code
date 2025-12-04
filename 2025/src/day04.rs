
use std::collections::HashSet;
use crate::{Coord, New};

fn part1(rolls: &HashSet<Coord>) {
    let mut accessible_rolls = 0;
    let neighbours = vec![
        Coord{x:-1,y:-1}, Coord{x: 0,y:-1}, Coord{x: 1,y:-1}, 
        Coord{x:-1,y: 0},                   Coord{x: 1,y: 0}, 
        Coord{x:-1,y: 1}, Coord{x: 0,y: 1}, Coord{x: 1,y: 1}
    ];
    for &roll in rolls {
        if neighbours.iter().map(|&neighbour| roll + neighbour).filter(|neighbour| rolls.contains(neighbour)).count() < 4 {
            accessible_rolls += 1;
        }
    }
    println!("{accessible_rolls}");
}

pub fn main () {
    let input = crate::grab_input("day04");

    let rolls: HashSet<Coord> = input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(row_index, row_chars)| {
            row_chars.chars().enumerate().map(move |(column_index, char)| (Coord::new(row_index, column_index), char))
        })
        .flatten()
        .filter_map(|(coord, char)| if char == '@' {Some(coord)} else {None})
        .collect();

    part1(&rolls);
}
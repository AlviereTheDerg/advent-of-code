
use std::{collections::{HashMap, HashSet}, convert::identity};
use crate::Coord;

fn part1(input: &HashMap<char, Vec<Coord>>, bounds: &Coord) {
    let mut anti_nodes = HashSet::<Coord>::new();

    for (_, antennae) in input.iter() {
        for &antennA in antennae.iter() {
            for &antennB in antennae.iter() {
                if antennA == antennB {continue;}
                let diff = antennA - antennB;

                if (antennA + diff).within_bounds(bounds) {
                    anti_nodes.insert(antennA + diff);
                }
                if (antennB + diff * -1).within_bounds(bounds) {
                    anti_nodes.insert(antennB + diff * -1);
                }
            }
        }
    }
    println!("{}", anti_nodes.len())
}

pub fn main() {
    let input = crate::grab_input("day08");

    let bounds = Coord{x:input.split_whitespace().next().unwrap().len() as isize, y:input.split_whitespace().filter(|s| !s.is_empty()).count() as isize};
    
    let filtered_data = input.split_whitespace().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .filter_map(move |(column, char)|
                if char != '.' {
                    Some((char, Coord{x:column as isize, y:row as isize}))
                } else {None}
            )
        ).flat_map(identity).collect::<Vec<(char, Coord)>>();
    
    let mapped_data = filtered_data.iter()
        .map(|(char, _)| *char).collect::<HashSet<char>>().iter()
        .map(|map_char| 
            (
                *map_char,
                filtered_data.iter().filter_map(|(tuple_char, tuple_coord)| {
                    if map_char == tuple_char {
                        Some(*tuple_coord)
                    } else {None}
                }).collect::<Vec<Coord>>()
            )
        ).collect::<HashMap<char, Vec<Coord>>>();

    part1(&mapped_data, &bounds);
}

use std::{collections::{HashMap, HashSet}, convert::identity};
use crate::{Coord, New};

#[allow(non_snake_case)]
fn get_antinodes_count(
    antenna_clusters: &HashMap<char, Vec<Coord>>, 
    bounds: &Coord, 
    anti_node_generator: &dyn Fn(Coord, Coord) -> Vec<Coord>)
    -> usize
{
    let mut anti_nodes: HashSet<Coord> = HashSet::new();
    for (_, antennae) in antenna_clusters.iter() {
        for &antennA in antennae.iter() {
            for &antennB in antennae.iter() {
                if antennA == antennB {continue;}
                anti_nodes.extend(
                    anti_node_generator(antennA, antennB).iter()
                        .filter(|coord| coord.within_bounds(bounds))
                );
            }
        }
    }
    anti_nodes.len()
}

#[allow(non_snake_case)]
fn part1(input: &HashMap<char, Vec<Coord>>, bounds: &Coord) {
    let result = get_antinodes_count(
        input, 
        bounds, 
        &|antennA, antennB| {
            vec![antennA * 2 - antennB] // antennA + (antennA - antennB)
        }
    );
    println!("{result}");
}

#[allow(non_snake_case)]
fn part2(input: &HashMap<char, Vec<Coord>>, bounds: &Coord) {
    let result = get_antinodes_count(
        input, 
        bounds, 
        &|antennA, antennB| {
            let mut output = Vec::new();
            let diff = antennA - antennB;
            let mut i = 0;
            while (antennA + diff * i).within_bounds(bounds) {
                output.push(antennA + diff * i);
                i += 1;
            }
            output
        }
    );
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day08");

    let bounds = Coord::new(
        input.split_whitespace().next().unwrap().len(), 
        input.split_whitespace().filter(|s| !s.is_empty()).count()
    );
    
    let filtered_data = input.split_whitespace().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .filter_map(move |(column, char)|
                if char != '.' {
                    Some((char, Coord::new(column, row)))
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
    part2(&mapped_data, &bounds);
}
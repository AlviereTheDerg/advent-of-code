
use std::collections::{HashMap, HashSet};
use crate::{Coord, New};

fn part1(input: &str) {
    let map: HashMap<Coord, usize> = input.split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate()
                .map(move |(column, value)| {
                    (
                        Coord::new(column, row),
                        value.to_digit(10).unwrap() as usize
                    )
                })
        })
        .flat_map(std::convert::identity)
        .collect();

    let trail_heads = map.iter()
        .filter_map(|(coord, value)| 
            if value == &0 {Some(*coord)} else {None}
        ).collect::<HashSet<Coord>>();

    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    
    let mut result = 0;
    for head in trail_heads {
        let mut travel = vec![head];
        let mut visited = HashSet::<Coord>::new();
        while let Some(head) = travel.pop() {
            visited.insert(head);
            for neighbour in neighbours.iter() {
                if let Some(value) = map.get(&(head + *neighbour)) {
                    if *value == map.get(&head).unwrap() + 1 {
                        travel.push(head + *neighbour);
                    }
                }
            }
        }
        result += visited.iter().filter_map(|coord| map.get(coord)).filter(|value| **value == 9).count();
    }
    println!("{result}");
}

fn part2(input: &str) {
    let map: HashMap<Coord, usize> = input.split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate()
                .map(move |(column, value)| {
                    (
                        Coord::new(column, row),
                        value.to_digit(10).unwrap() as usize
                    )
                })
        })
        .flat_map(std::convert::identity)
        .collect();
    let mut trails = map.iter()
        .filter_map(|(coord, value)| 
            if value == &0 {Some((*coord, 1usize))} else {None}
        ).collect::<HashMap<Coord, usize>>();

    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    
    for value in 1..10usize {
        let mut next_trails = HashMap::new();
        for (trail_end, convergences) in trails.iter() {
            for neighbour in neighbours.iter() {
                let checked_spot = *trail_end + *neighbour;
                if map.get(&checked_spot) == Some(&value) {
                    next_trails.entry(checked_spot)
                        .and_modify(|v| {*v += convergences})
                        .or_insert(*convergences);
                }
            }
        }
        trails = next_trails;
    }
    let result: usize = trails.iter().map(|(_, convergences)| convergences).sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day10");
    part1(&input);
    part2(&input);
}
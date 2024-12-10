
use std::collections::{HashMap, HashSet};
use crate::{Coord, New};

fn part1(map: &HashMap<Coord, usize>, trail_heads: &HashSet<Coord>) {
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    
    let mut result = 0;
    for head in trail_heads.iter() {
        let mut travel = vec![*head];
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
        result += visited.iter()
            .filter_map(|coord| map.get(coord))
            .filter(|value| **value == 9)
            .count();
    }
    println!("{result}");
}

fn part2(map: &HashMap<Coord, usize>, trail_heads: &HashSet<Coord>) {
    let mut trails = trail_heads.iter()
        .map(|coord| (*coord, 1))
        .collect::<HashMap<Coord, usize>>();

    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    
    for value in 1..10usize {
        let mut next_trails = HashMap::new();
        for (trail_end, convergences) in trails.iter() {
            for checked_spot in neighbours.iter()
                .map(|neighbour| *trail_end + *neighbour)
                .filter(|neighbour| map.get(&neighbour) == Some(&value))
            {
                next_trails.entry(checked_spot)
                    .and_modify(|v| {*v += convergences})
                    .or_insert(*convergences);
            }
        }
        trails = next_trails;
    }
    let result: usize = trails.iter()
        .map(|(_, convergences)| convergences)
        .sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day10");

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

    part1(&map, &trail_heads);
    part2(&map, &trail_heads);
}
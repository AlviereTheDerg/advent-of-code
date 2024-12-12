
use std::collections::{HashMap, HashSet};
use crate::{Coord, New};

fn group(map: &HashMap<Coord, char>, nucleation: Coord) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    let group_id = map.get(&nucleation);

    let mut travel_stack = vec![nucleation];
    while !travel_stack.is_empty() {
        let here = travel_stack.pop().unwrap();
        visited.insert(here);
        for neighbour in neighbours.iter().map(|n| here + *n) {
            if !visited.contains(&neighbour) && map.get(&neighbour) == group_id {
                travel_stack.push(neighbour);
            }
        }
    }

    visited
}

fn get_fencing(patch: &HashSet<Coord>) -> usize {
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    let mut result = 0;
    for here in patch.iter() {
        for neighbour in neighbours.iter().map(|n| *here + *n) {
            if !patch.contains(&neighbour) {result += 1}
        }
    }
    result
}

fn part1(map: &HashMap<Coord, char>, bounds: &Coord) {
    let mut groups = Vec::<HashSet<Coord>>::new();
    let mut visited = HashSet::<Coord>::new();

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            if !visited.contains(&Coord::new(x,y)) {
                let new_group = group(map, Coord::new(x,y));
                visited.extend(new_group.iter());
                groups.push(new_group);
            }
        }
    }

    let result: usize = groups.iter().map(|s| s.len() * get_fencing(s)).sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day12");

    let bounds = Coord::new(input.split_whitespace().next().unwrap().len(), input.split_whitespace().filter(|s| !s.is_empty()).count());
    let map: HashMap<Coord, char> = input.split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate()
                .map(move |(column, value)| {
                    (
                        Coord::new(column, row),
                        value
                    )
                })
        })
        .flat_map(std::convert::identity)
        .collect();

    part1(&map, &bounds);
}
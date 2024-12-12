
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

fn get_fencing(patch: &HashSet<Coord>) -> Vec<(Coord, usize)> {
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:0,y:1}, Coord{x:-1,y:0}, Coord{x:0,y:-1}];
    let mut result = Vec::<(Coord, usize)>::new();
    for here in patch.iter() {
        for (dir, neighbour) in neighbours.iter().map(|n| *here + *n).enumerate() {
            if !patch.contains(&neighbour) {result.push((neighbour, dir))}
        }
    }
    result
}

fn part1(groups: &Vec<HashSet<Coord>>) {
    let result: usize = groups.iter().map(|s| s.len() * get_fencing(s).len()).sum();
    println!("{result}");
}

fn get_fencelines(patch: &HashSet<Coord>) -> usize {
    let fence_units = get_fencing(patch);
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:0,y:1}, Coord{x:-1,y:0}, Coord{x:0,y:-1}];

    let mut lines = Vec::<HashSet<Coord>>::new();
    for direction in 0..4usize {
        let mut fence_units = fence_units.iter()
            .filter_map(|(coord, dir)| 
                if direction == *dir {Some(*coord)} 
                else {None}
            ).collect::<HashSet<Coord>>();
        
        // if 1,0 or -1,0 then dir%2 == 0, if 0,1 or 0,-1 then dir%2 == 1
        let neighbours = neighbours.iter().enumerate()
            .filter_map(|(dir, coord)| 
                if direction % 2 != dir % 2 {Some(*coord)} else {None})
            .collect::<Vec<Coord>>();

        while !fence_units.is_empty() {
            let start = *fence_units.iter().next().unwrap();
            fence_units.remove(&start);

            let mut visited = HashSet::<Coord>::new();
            let mut travel_stack = vec![start];
            while !travel_stack.is_empty() {
                let here = travel_stack.pop().unwrap();
                visited.insert(here);
                for neighbour in neighbours.iter().map(|n| here + *n) {
                    if fence_units.contains(&neighbour) {
                        fence_units.remove(&neighbour);
                        travel_stack.push(neighbour);
                    }
                }
            }
            lines.push(visited);
        }
    }

    lines.len()
}

fn part2(groups: &Vec<HashSet<Coord>>) {
    let result: usize = groups.iter().map(|s| s.len() * get_fencelines(s)).sum();
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

    
    let mut groups = Vec::<HashSet<Coord>>::new();
    let mut visited = HashSet::<Coord>::new();

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            if !visited.contains(&Coord::new(x,y)) {
                let new_group = group(&map, Coord::new(x,y));
                visited.extend(new_group.iter());
                groups.push(new_group);
            }
        }
    }

    part1(&groups);
    part2(&groups);
}

use std::collections::{HashMap, HashSet, VecDeque};
use crate::{Coord, New};

fn moving_options(position: Coord) -> Vec<Coord> {
    vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}]
        .into_iter()
        .map(|c| c + position)
        .collect()
}

fn backtrace(direction_map: HashMap<Coord, Coord>, end: Coord) -> Vec<Coord> {
    let mut stack_trace = vec![end];
    while let Some(thing) = direction_map.get(&stack_trace.last().unwrap()) {
        if thing == stack_trace.last().unwrap() {break}
        stack_trace.push(*thing);
    }
    stack_trace.reverse();
    stack_trace
}

fn search(
    walls: &HashSet<Coord>, 
    start: Coord, 
    goal: Coord, 
    bounds: Coord,
) -> HashSet<Coord>
{
    // score -> Vec<(position, direction, visitation_stack)>
    let mut exploration: VecDeque<Coord> = VecDeque::new();
    // (position, direction) -> score
    let mut visited: HashMap<Coord, Coord> = HashMap::new();

    exploration.push_front(start);
    visited.insert(start, start);
    while let Some(here) = exploration.pop_back() {
        for neighbour in moving_options(here) {
            if !neighbour.within_bounds(&bounds) || visited.contains_key(&neighbour) || walls.contains(&neighbour) {continue;}

            visited.insert(neighbour, here);
            exploration.push_front(neighbour);

            if neighbour == goal {
                return backtrace(visited, goal).into_iter().collect();
            }
        }
    }

    HashSet::new()
}

fn part1(obstacles: &Vec<Coord>, start: Coord, goal: Coord, bounds: Coord) -> (HashSet<Coord>, HashSet<Coord>) {
    let first_1024: HashSet<Coord> = (obstacles[..1024]).iter().map(Coord::clone).collect();
    let result = search(&first_1024, start, goal, bounds);
    println!("{}", result.len() - 1);
    (result, first_1024)
}

fn part2(mut path: HashSet<Coord>, obstacles: &Vec<Coord>, mut blockages: HashSet<Coord>, start: Coord, goal: Coord, bounds: Coord) {
    let mut last_blockage = start;
    while let Some(blockage) = obstacles.iter().filter(|c| path.contains(c)).next() {
        last_blockage = *blockage;
        blockages.insert(*blockage);
        path = search(&blockages, start, goal, bounds);
    }
    println!("{},{}", last_blockage.x, last_blockage.y);
}

pub fn main() {
    let input = crate::grab_input("day18");
    let blocks = input.split_whitespace()
        .filter_map(|s|
            if !s.is_empty() {
                let mut splits = s.split(",");
                let x: isize = splits.next().unwrap().parse().unwrap(); 
                let y: isize = splits.next().unwrap().parse().unwrap();
                Some(Coord::new(x, y))
            } else {None}
        )
        .collect::<Vec<Coord>>();
    let start = Coord::new(0isize, 0isize);
    let width = 70isize;
    let goal = Coord::new(width, width);
    let bounds = Coord::new(width+1, width+1);
    let (path, blockages) = part1(&blocks, start, goal, bounds);
    part2(path, &blocks, blockages, start, goal, bounds);
}
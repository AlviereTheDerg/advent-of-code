
use std::collections::{HashMap, HashSet, VecDeque};
use crate::{Coord, New};

fn get_path(walls: &HashSet<Coord>, start: Coord, end: Coord) -> HashMap<Coord, usize> {
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];
    let mut travel_path = vec![start];
    let mut visited: HashSet<Coord> = HashSet::new();
    while let Some(&thing) = travel_path.last() {
        visited.insert(thing);
        let next = neighbours.iter()
            .map(|c| thing + *c)
            .filter(|c| !walls.contains(c) && !visited.contains(c))
            .next().unwrap();
        travel_path.push(next);
        if next == end {break;}
    }

    travel_path.into_iter().enumerate().map(|(index, coord)| (coord, index)).collect()
}

fn part1(walls: &HashSet<Coord>, start: Coord, end: Coord) {
    let travel_path = get_path(walls, start, end);
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];

    let mut cheats: HashMap<(Coord, Coord), usize> = HashMap::new();
    for (&start_point, &start_score) in travel_path.iter() {
        for &neighbour in neighbours.iter() {
            let destination_point = start_point + neighbour * 2;
            if let Some(&destination_score) = travel_path.get(&destination_point) {
                if destination_score > start_score + 2 {
                    cheats.insert((start_point, destination_point), destination_score - start_score - 2);
                }
            }
        }
    }

    /*let cheat_skips = cheats.values().collect::<HashSet<_>>();
    let mut cheat_skips = cheat_skips.iter().collect::<Vec<_>>();
    cheat_skips.sort();
    for skip_length in cheat_skips {
        let skips = cheats.values().filter(|cheat_skip| cheat_skip == skip_length).count();
        println!("There are {skips} cheats that save {skip_length} picoseconds.");
    }*/
    let result = cheats.values().filter(|&&skip_length| skip_length >= 100).count();
    println!("{result}");
}

fn all_reachable_within_20(coord: Coord) -> HashMap<Coord, usize> {
    let neighbours = vec![Coord{x:1,y:0}, Coord{x:-1,y:0}, Coord{x:0,y:1}, Coord{x:0,y:-1}];

    let mut exploration = VecDeque::new();
    exploration.push_front((coord, 0usize));
    let mut explored = HashMap::new();

    while let Some((here, dist)) = exploration.pop_back() {
        if explored.contains_key(&here) || dist > 20 {continue;}
        explored.insert(here, dist);

        for &neighbour in neighbours.iter() {
            exploration.push_front((here+neighbour, dist+1));
        }
    }

    explored
}

fn part2(walls: &HashSet<Coord>, start: Coord, end: Coord) {
    let travel_path = get_path(walls, start, end);

    let mut cheats: HashMap<(Coord, Coord), usize> = HashMap::new();
    for (&start_point, &start_score) in travel_path.iter() {
        for (destination_point, skip_dist) in all_reachable_within_20(start_point) {
            if let Some(&destination_score) = travel_path.get(&destination_point) {
                if destination_score > start_score + skip_dist {
                    cheats.insert((start_point, destination_point), destination_score - start_score - skip_dist);
                }
            }
        }
    }

    let result = cheats.values().filter(|&&skip_length| skip_length >= 100).count();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day20");

    // strip all the '.'
    let mut maze = input.split_whitespace()
        .enumerate()
        .map(|(row, line)|
            line.chars()
                .enumerate()
                .map(move |(column, char)|
                (
                    Coord::new(column, row),
                    char
                )
            )
        )
        .flat_map(std::convert::identity)
        .filter(|(_, char)| *char != '.')
        .collect::<HashMap<Coord, char>>();

    // identify start/end and remove from maze
    let goal = *maze.iter()
        .filter_map(|(coord, char)| 
            if *char == 'E' {
                Some(coord)
            } else {None}
        )
        .next().unwrap();
    maze.remove(&goal);

    let start = *maze.iter()
        .filter_map(|(coord, char)| 
            if *char == 'S' {
                Some(coord)
            } else {None}
        )
        .next().unwrap();
    maze.remove(&start);

    // convert remainder of maze to set of walls
    let walls = maze.into_keys().collect::<HashSet<Coord>>();

    part1(&walls, start, goal);
    part2(&walls, start, goal);
}

use std::{collections::HashSet, convert::identity};
use crate::Coord;

fn get_guard_next_position(guard_data: (Coord, isize), obstacles: &HashSet<Coord>) -> (Coord, isize) {
    let offset = match guard_data.1 {
        0 => Some(Coord{x:0, y:-1}),
        1 => Some(Coord{x:1, y:0}),
        2 => Some(Coord{x:0, y:1}),
        3 => Some(Coord{x:-1, y:0}),
        _ => None
    }.unwrap();
    
    if !obstacles.contains(&(guard_data.0 + offset)) {
        (guard_data.0 + offset, guard_data.1)
    } else {
        (guard_data.0, (guard_data.1 + 1) % 4)
    }
}

fn get_walk_positions(bounds: &Coord, start_position: &Coord, obstacles: &HashSet<Coord>) -> HashSet<Coord> {
    let mut guard_position = start_position.clone();

    let mut guard_dir = 0;
    let mut guard_track = HashSet::<(Coord, isize)>::new();

    while guard_position.x < bounds.x && guard_position.x >= 0 && guard_position.y < bounds.y && guard_position.y >= 0 {
        guard_track.insert((guard_position, guard_dir));
        let hold = get_guard_next_position((guard_position, guard_dir), obstacles);
        guard_position = hold.0;
        guard_dir = hold.1;
    }

    guard_track.into_iter().map(|(pos, _)| pos).collect()
}

fn part1(bounds: &Coord, guard_position: &Coord, obstacles: &HashSet<Coord>) {
    let result = get_walk_positions(bounds, guard_position, obstacles).len();
    println!("{result}")
}

fn loops(bounds: &Coord, start_position: &Coord, obstacles: &HashSet<Coord>) -> bool {
    let mut guard_position = start_position.clone();
    let mut guard_dir = 0;
    let mut guard_track = HashSet::<(Coord, isize)>::new();

    while guard_position.x < bounds.x && guard_position.x >= 0 && guard_position.y < bounds.y && guard_position.y >= 0 {
        if guard_track.contains(&(guard_position, guard_dir)) {return true;}
        guard_track.insert((guard_position, guard_dir));
        let hold = get_guard_next_position((guard_position, guard_dir), obstacles);
        guard_position = hold.0;
        guard_dir = hold.1;
    }
    false
}

fn part2(bounds: &Coord, guard_position: &Coord, obstacles: &HashSet<Coord>) {
    let mut result = 0;
    for possible_obstacle in get_walk_positions(bounds, guard_position, obstacles) {
        if guard_position == &possible_obstacle {continue;}
        let mut this_obstacles = obstacles.clone();
        this_obstacles.insert(possible_obstacle);

        if loops(bounds, guard_position, &this_obstacles) {result += 1;}
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day06");
    
    let filtered_data = input.split_whitespace().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .map(move |(column, char)|
                (Coord{x:column as isize, y:row as isize}, char)
            )
        ).flat_map(identity).collect::<Vec<(Coord, char)>>();
    
    let bounds: Coord = Coord{x:input.split_whitespace().next().unwrap().len() as isize,y:input.split_whitespace().count() as isize};
    let guard_position = filtered_data.iter().filter_map(|(pos, char)| if char == &'^' {Some(pos)} else {None}).next().unwrap().clone();
    let obstacles: HashSet<Coord> = filtered_data.into_iter().filter_map(|(pos, char)| if char == '#' {Some(pos)} else {None}).collect();

    part1(&bounds, &guard_position, &obstacles);
    part2(&bounds, &guard_position, &obstacles);
}
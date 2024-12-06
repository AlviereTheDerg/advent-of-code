
use std::{collections::HashSet, convert::identity};

use crate::Coord;

fn part1(input: &str) {
    let filtered_data = input.split_whitespace().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .map(move |(column, char)|
                (Coord{x:column as isize, y:row as isize}, char)
            )
        ).flat_map(identity).collect::<Vec<(Coord, char)>>();
    
    let bounds: Coord = Coord{x:input.split_whitespace().next().unwrap().len() as isize,y:input.split_whitespace().count() as isize};
    let mut guard_position = filtered_data.iter().filter_map(|(pos, char)| if char == &'^' {Some(pos)} else {None}).next().unwrap().clone();
    let obstacles: HashSet<Coord> = filtered_data.into_iter().filter_map(|(pos, char)| if char == '#' {Some(pos)} else {None}).collect();

    let mut guard_dir = 0;
    let get_guard_move = |dir| {
        match dir {
            0 => Some(Coord{x:0, y:-1}),
            1 => Some(Coord{x:1, y:0}),
            2 => Some(Coord{x:0, y:1}),
            3 => Some(Coord{x:-1, y:0}),
            _ => None
        }
    };
    let mut guard_track = HashSet::<(Coord, isize)>::new();

    while !guard_track.contains(&(guard_position, guard_dir)) {
        guard_track.insert((guard_position, guard_dir));
        if !obstacles.contains(&(guard_position + get_guard_move(guard_dir).unwrap())) {
            guard_position = guard_position + get_guard_move(guard_dir).unwrap();
        } else {
            guard_dir = (guard_dir + 1) % 4;
        }
        // if guard goes out of bounds
        if guard_position.x >= bounds.x || guard_position.x < 0 || guard_position.y >= bounds.y || guard_position.y < 0 {break;}
    }

    let result = guard_track.into_iter().map(|(pos, _)| pos).collect::<HashSet<Coord>>().len();
    println!("{result}")
}

pub fn main() {
    let input = crate::grab_input("day06");
    part1(&input);
}
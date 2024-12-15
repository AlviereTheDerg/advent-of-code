
use std::collections::HashMap;
use crate::{Coord, New};

#[derive(Debug, Clone)]
struct StorageMap {
    pub data: HashMap::<Coord, char>,
    pub robot: Coord,
    pub bounds: Coord,
}
impl StorageMap {
    pub fn from(input: &str) -> Self {
        let bounds = Coord::new(input.split_whitespace().next().unwrap().len(), input.split_whitespace().count());
        let mut data: HashMap::<Coord, char> = input.split_whitespace()
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
            .filter(|(_, char)|
                *char != '.'
            )
            .collect();
        let robot = *data.iter().filter_map(|(coord, char)| if *char == '@' {Some(coord)} else {None}).next().unwrap();
        data.remove(&robot);

        StorageMap{data, robot, bounds}
    }

    pub fn grade(&self) -> isize {
        self.data.iter()
            .filter_map(|(coord, char)|
                if *char == 'O' {
                    Some(coord.x + 100*coord.y)
                } else {None}
            )
            .sum()
    }

    pub fn robot_move(&mut self, direction: Coord) {
        let mut hold = self.robot;
        // empty space check
        while true {
            hold = hold + direction;
            if self.data.get(&hold) == Some(&'#') {return;} // check hit a wall (can't move here)
            if !hold.within_bounds(&self.bounds) {return;} // check went OOB (can't move here)
            if !self.data.contains_key(&hold) {break;} // check found empty space (continue)
        }
        let destination = self.robot + direction;
        // if the space right in front of the robot isn't open
        if hold != destination {
            self.data.insert(hold, *self.data.get(&destination).unwrap());
            self.data.remove(&destination);
        }

        self.robot = destination;
    }
}

fn part1(mut input: StorageMap, directions: &Vec<Coord>) {
    for direction in directions {
        input.robot_move(*direction);
    }
    println!("{}", input.grade());
}

pub fn main() {
    let input = crate::grab_input("day15");
    let mut sections = input.split("\n\n");
    let map = StorageMap::from(sections.next().unwrap());
    let directions: Vec<Coord> = sections.next().unwrap()
        .chars()
        .filter_map(|char|
            match char {
                '^' => Some(Coord::new(0isize, -1isize)),
                '>' => Some(Coord::new(1isize, 0isize)),
                'v' => Some(Coord::new(0isize, 1isize)),
                '<' => Some(Coord::new(-1isize, 0isize)),
                _ => None,
            }
        )
        .collect();

    part1(map.clone(), &directions);
}

use std::collections::{HashMap, HashSet};
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
                if *char == 'O' || *char == '[' {
                    Some(coord.x + 100*coord.y)
                } else {None}
            )
            .sum()
    }

    fn can_robot_move(&self, direction: Coord) -> Option<HashSet<Coord>> {
        let mut all_hold = HashSet::new();
        let mut expansion_stack = vec![self.robot + direction];
        // empty space check
        while !expansion_stack.is_empty() {
            let hold = expansion_stack.pop().unwrap();
            if all_hold.contains(&hold) {continue;}
            all_hold.insert(hold);
            if self.data.get(&hold) == Some(&'#') {return None;} // check hit a wall (can't move here)
            if !hold.within_bounds(&self.bounds) {return None;} // check went OOB (can't move here)
            if let Some(char) = self.data.get(&hold) {
                match char {
                    'O' => expansion_stack.push(hold + direction),
                    '[' => {
                        all_hold.insert(hold + Coord::new(1isize, 0isize));
                        expansion_stack.push(hold + direction); 
                        expansion_stack.push(hold + direction + Coord::new(1isize,0isize))
                    },
                    ']' => {
                        all_hold.insert(hold + Coord::new(-1isize, 0isize));
                        expansion_stack.push(hold + direction); 
                        expansion_stack.push(hold + direction + Coord::new(-1isize,0isize))
                    },
                    _ => {},
                }
            }
        }
        Some(all_hold)
    }

    pub fn robot_move(&mut self, direction: Coord) {
        if let Some(mut hold) = self.can_robot_move(direction) {
            hold.retain(|coord| self.data.contains_key(coord));
            while !hold.is_empty() {
                let moving = hold.iter()
                    .filter_map(|&coord|
                        if !self.data.contains_key(&(coord + direction)) {
                            Some(coord)
                        } else {None}
                    )
                    .collect::<HashSet<Coord>>();
                for mover in moving.iter() {
                    let moved = self.data.remove(&mover).unwrap();
                    self.data.insert(*mover + direction, moved);
                }
                hold.retain(|coord| !moving.contains(coord));
            }
            self.robot = self.robot + direction;
        }
    }

    pub fn enwiden(mut self) -> Self {
        let x_doubler = |coord: Coord| Coord::new(2*coord.x, coord.y);
        self.robot = x_doubler(self.robot);
        self.bounds = x_doubler(self.bounds);
        self.data = self.data.into_iter()
                .map(|(coord, char)| {
                    let base = x_doubler(coord);
                    match char {
                        '#' => vec![(base, '#'), (base + Coord::new(1isize, 0isize), '#')].into_iter(),
                        'O' => vec![(base, '['), (base + Coord::new(1isize, 0isize), ']')].into_iter(),
                        _ => vec![].into_iter(),
                    }
                })
                .flat_map(std::convert::identity)
                .collect::<HashMap<Coord, char>>();

        self
    }
}

fn print_final_grade(mut input: StorageMap, directions: &Vec<Coord>) {
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

    print_final_grade(map.clone(), &directions);
    print_final_grade(map.enwiden(), &directions);
}
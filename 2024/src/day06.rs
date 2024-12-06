
use std::{collections::HashSet, convert::identity};
use crate::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct GuardData {
    pub pos: Coord,
    pub dir: isize,
}
impl GuardData {
    pub fn get_next_position(&self, obstacles: &HashSet<Coord>, possible_obstacle: Option<Coord>) -> GuardData {
        let offset = match self.dir {
            0 => Some(Coord{x:0, y:-1}),
            1 => Some(Coord{x:1, y:0}),
            2 => Some(Coord{x:0, y:1}),
            3 => Some(Coord{x:-1, y:0}),
            _ => None
        }.unwrap();
    
        if !obstacles.contains(&(self.pos + offset)) && Some(self.pos + offset) != possible_obstacle {
            GuardData{pos: self.pos + offset, dir: self.dir}
        } else {
            GuardData{pos: self.pos, dir: (self.dir + 1) % 4}
        }
    }

    pub fn step(&mut self, obstacles: &HashSet<Coord>, possible_obstacle: Option<Coord>) {
        let next = self.get_next_position(obstacles, possible_obstacle);
        self.pos = next.pos;
        self.dir = next.dir;
    }
}

fn get_walk_positions(bounds: &Coord, start_position: &Coord, obstacles: &HashSet<Coord>) -> HashSet<GuardData> {
    let mut guard = GuardData{pos: *start_position, dir: 0};
    let mut guard_track = HashSet::<GuardData>::new();

    while guard.pos.within_bounds(bounds) {
        guard_track.insert(guard);
        guard.step(obstacles, None);
    }

    guard_track
}

fn part1(bounds: &Coord, guard_position: &Coord, obstacles: &HashSet<Coord>) {
    let result = get_walk_positions(bounds, guard_position, obstacles)
        .into_iter()
        .map(|guard| guard.pos)
        .collect::<HashSet<Coord>>()
        .len();
    println!("{result}")
}

fn loops(bounds: &Coord, start_position: &GuardData, obstacles: &HashSet<Coord>, added_obstacle: Coord, previous_steps: &HashSet<GuardData>) -> bool {
    let mut guard = start_position.clone();
    let mut guard_track = previous_steps.clone();
    if !added_obstacle.within_bounds(bounds) {return false;}

    while guard.pos.within_bounds(bounds) {
        if guard_track.contains(&guard) {return true;}
        guard_track.insert(guard);
        guard.step(&obstacles, Some(added_obstacle));
    }
    false
}

fn part2(bounds: &Coord, guard_position: &Coord, obstacles: &HashSet<Coord>) {
    let mut guard = GuardData{pos:*guard_position, dir:0};
    let mut visited = HashSet::new();
    let mut added_obstacles = HashSet::new();
    let mut previous_steps = HashSet::new();

    while guard.pos.within_bounds(bounds) {
        visited.insert(guard.pos);

        let next = guard.get_next_position(obstacles, None).pos;

        // paradox avoidance, don't put an obstacle where we've already been
        // also redundancy checks: don't try and place an obstacle if there's already one there OR we've already tried putting one there
        if visited.contains(&next) || obstacles.contains(&next) || added_obstacles.contains(&next) { 
            guard.step(obstacles, None); 
            continue;
        }

        if loops(bounds, &guard, obstacles, next, &previous_steps) {
            added_obstacles.insert(next);
        }

        previous_steps.insert(guard);
        guard.step(obstacles, None);
    }

    println!("{}", added_obstacles.len());
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
    
    let timer = std::time::Instant::now();
    part1(&bounds, &guard_position, &obstacles);
    println!("{}", timer.elapsed().as_millis());

    let timer = std::time::Instant::now();
    part2(&bounds, &guard_position, &obstacles);
    println!("{}", timer.elapsed().as_millis());
}
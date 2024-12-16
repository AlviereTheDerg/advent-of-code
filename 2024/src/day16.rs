
use std::collections::{HashMap, HashSet, BTreeMap};
use crate::{Coord, New};

fn search(
    walls: &HashSet<Coord>, 
    start: &Coord, 
    goal: &Coord, 
    // input (position, direction), return list of possible (position, direction, points to move) to move to
    moving_options: &dyn Fn(Coord, Coord) -> Vec<(Coord, Coord, isize)> 
) -> isize
{
    // score -> Vec<(position, direction)>
    let mut exploration: BTreeMap<isize, Vec<(Coord, Coord)>> = BTreeMap::new();
    // (position, direction) -> score
    let mut visited: HashMap<(Coord, Coord), isize> = HashMap::new();

    exploration.insert(0, vec![(*start, Coord::new(1isize, 0isize))]);

    while !exploration.is_empty() {
        let (score, mut spots) = exploration.pop_first().unwrap();
        let (current_position, current_direction) = match spots.pop() {
            Some(stuff) => stuff,
            None => {continue;},
        };
        exploration.insert(score, spots);
        visited.insert((current_position, current_direction), score);

        for (next_position, next_direction, points_to_move) in moving_options(current_position, current_direction) {
            // if we've found the end, it's de facto the shortest path, otherwise we wouldn't be checking here
            if next_position == *goal {return score+points_to_move;}

            // if that next position is invalid, don't try and go there
            if walls.contains(&next_position) {continue;}

            // if we've already gotten to this position with a smaller value, don't try and go there again
            if **visited.get(&(next_position, next_direction)).get_or_insert(&isize::MAX) <= score+points_to_move {continue;}

            exploration.entry(score+points_to_move)
                .and_modify(|v| 
                    v.push((next_position, next_direction))
                )
                .or_insert(
                    vec![(next_position, next_direction)]
                );
        }
    }

    -1
}

fn part1(walls: &HashSet<Coord>, start: &Coord, goal: &Coord) {
    let turns: HashMap<Coord, Vec<Coord>> = vec![
        (Coord::new(1isize,  0isize), vec![Coord::new(0isize, -1isize), Coord::new(0isize, 1isize)]),
        (Coord::new(-1isize, 0isize), vec![Coord::new(0isize, -1isize), Coord::new(0isize, 1isize)]),
        (Coord::new(0isize,  1isize), vec![Coord::new(-1isize, 0isize), Coord::new(1isize, 0isize)]),
        (Coord::new(0isize, -1isize), vec![Coord::new(-1isize, 0isize), Coord::new(1isize, 0isize)])
    ].into_iter().collect();

    let moving_options = |position: Coord, direction: Coord| {
        vec![
            (position+direction, direction, 1isize),
            (position, *turns.get(&direction).unwrap().get(0).unwrap(), 1000isize),
            (position, *turns.get(&direction).unwrap().get(1).unwrap(), 1000isize)
        ]
    };
    let result = search(walls, start, goal, &moving_options);
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day16");
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

    let walls = maze.into_keys().collect::<HashSet<Coord>>();

    part1(&walls, &start, &goal);
}
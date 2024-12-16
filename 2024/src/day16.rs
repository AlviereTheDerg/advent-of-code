
use std::collections::{HashMap, HashSet, BTreeMap};
use crate::{Coord, New};

fn search(
    walls: &HashSet<Coord>, 
    start: &Coord, 
    goal: &Coord, 
    // input (position, direction), return list of possible (position, direction, points to move) to move to
    moving_options: &dyn Fn(Coord, Coord) -> Vec<(Coord, Coord, isize)> 
) -> (HashSet<Coord>, isize)
{
    // score -> Vec<(position, direction, visitation_stack)>
    let mut exploration: BTreeMap<isize, Vec<(Coord, Coord, Vec<Coord>)>> = BTreeMap::new();
    // (position, direction) -> score
    let mut visited: HashMap<(Coord, Coord), isize> = HashMap::new();

    exploration.insert(0, vec![(*start, Coord::new(1isize, 0isize), vec![])]);
    let mut smallest_path = isize::MAX;
    let mut smallest_path_visits = HashSet::<Coord>::new();
    while !exploration.is_empty() && *exploration.first_entry().unwrap().key() < smallest_path {
        let (score, mut spots) = exploration.pop_first().unwrap();
        let (current_position, current_direction, mut visitation_stack) = match spots.pop() {
            Some(stuff) => stuff,
            None => {continue;},
        };
        visitation_stack.push(current_position);
        exploration.insert(score, spots);
        visited.insert((current_position, current_direction), score);

        for (next_position, next_direction, points_to_move) in moving_options(current_position, current_direction) {
            let score = score + points_to_move;
            // if we've found the end, it's de facto the shortest path, otherwise we wouldn't be checking here
            if next_position == *goal {
                if score < smallest_path {
                    smallest_path_visits = HashSet::new();
                    smallest_path_visits.insert(*goal);
                    smallest_path = score;
                }
                smallest_path_visits.extend(visitation_stack.iter());
                continue;
            }

            // if that next position is invalid, don't try and go there
            if walls.contains(&next_position) {continue;}

            // if we've already gotten to this position with a smaller value, don't try and go there again
            if **visited.get(&(next_position, next_direction)).get_or_insert(&isize::MAX) <= score {continue;}

            exploration.entry(score)
                .and_modify(|v| 
                    v.push((next_position, next_direction, visitation_stack.clone()))
                )
                .or_insert(
                    vec![(next_position, next_direction, visitation_stack.clone())]
                );
        }
    }

    (smallest_path_visits, smallest_path)
}

fn part1(
    walls: &HashSet<Coord>, 
    start: &Coord, 
    goal: &Coord,
    moving_options: &dyn Fn(Coord, Coord) -> Vec<(Coord, Coord, isize)> 
) {
    let (_, result) = search(walls, start, goal, &moving_options);
    println!("{result}");
}

fn part2(
    walls: &HashSet<Coord>, 
    start: &Coord, 
    goal: &Coord,
    moving_options: &dyn Fn(Coord, Coord) -> Vec<(Coord, Coord, isize)> 
) {
    let (result, _) = search(walls, start, goal, &moving_options);
    println!("{}", result.len());
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

    part1(&walls, &start, &goal, &moving_options);
    part2(&walls, &start, &goal, &moving_options);
}
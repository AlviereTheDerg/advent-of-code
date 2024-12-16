
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
        // get score and spots with that score
        let (score, mut spots) = exploration.pop_first().unwrap();

        // if there are no spots with that score, just go to the next, otherwise we propagate
        let (current_position, current_direction, mut visitation_stack) = match spots.pop() {
            Some(stuff) => stuff,
            None => {continue;},
        };
        exploration.insert(score, spots); // re-insert remaining spots (if the spots with that score are empty it gets sorted next loop)
        visitation_stack.push(current_position); // for ease, visitation stack doesn't get added to until it's popped
        visited.insert((current_position, current_direction), score); // flag this position+direction with the score to reach it

        for (next_position, next_direction, points_to_move) in moving_options(current_position, current_direction) {
            let score = score + points_to_move;

            // if this position has no hope of making it in under the smallest score found, no need to check
            if score > smallest_path {continue;}

            // if we've found the end, it's de facto the shortest path so far, otherwise we wouldn't be checking here
            if next_position == *goal {
                if score < smallest_path {
                    smallest_path_visits = HashSet::new();
                    smallest_path_visits.insert(*goal);
                    smallest_path = score;
                }
                smallest_path_visits.extend(visitation_stack.iter());
                continue;
            }

            // if the next position is invalid, don't try and go there
            if walls.contains(&next_position) {continue;}

            // if we've already gotten to that position with a smaller score, don't try and go there again
            if **visited.get(&(next_position, next_direction)).get_or_insert(&isize::MAX) <= score {continue;}

            // if we've made it this far, propagate
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

pub fn main() {
    let input = crate::grab_input("day16");

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

    // helper closure to list off potential 'move from'
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

    // search does both part 1 and part 2, get the result, separate, print real pretty
    let (visiteds, path_score) = search(&walls, &start, &goal, &moving_options);
    println!("{}\n{}", path_score, visiteds.len());
}
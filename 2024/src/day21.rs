
use std::collections::HashMap;

use crate::{Coord, New};

// code -> list of lists of chars, chars are buttons, 
// each inner list is a unique shortest path s.t. all paths go from the same start to the same end
fn get_numpad_robot_inputs(numpad: &HashMap<char, Coord>, code: &str) -> Vec<String> {
    let mut result = vec![];
    let mut current_key = 'A'; // always start on A
    for button in code.chars() {
        let start_coord = *numpad.get(&current_key).unwrap();
        let end_coord = *numpad.get(&button).unwrap();

        // find all valid shortest-length paths from start to end
        // zigzags BAD so it's gotta be just straight lines
        let diff = end_coord - start_coord;
        let vert_move = (0..diff.y.abs()).map(|_| if diff.y > 0 {'v'} else {'^'}).collect::<String>();
        let hori_move = (0..diff.x.abs()).map(|_| if diff.x > 0 {'>'} else {'<'}).collect::<String>();
        
        // hand-found best paths
        result.push(
            // if we go right but the down-first corner crossed is present, we go down first then horizontal
            if diff.x > 0 && numpad.values().any(|coord| coord == &Coord::new(start_coord.x, end_coord.y)) {
                vert_move + &hori_move
            }
            // otherwise try horizontal first then vertical
            else if numpad.values().any(|coord| coord == &Coord::new(end_coord.x, start_coord.y)) {
                hori_move + &vert_move
            }
            // have to go vertical first then horizontal
            else {
                vert_move + &hori_move
            } 
            + &"A"
        );
        current_key = button;
    }
    result
}

fn sum_of_complexities_through_n_robots(codes: &Vec<&str>, mediator_bots: usize) {
    let numpad: HashMap<char, Coord> = vec![
        (Coord::new(2isize, 0isize), '9'),
        (Coord::new(1isize, 0isize), '8'),
        (Coord::new(0isize, 0isize), '7'),
        (Coord::new(2isize, 1isize), '6'),
        (Coord::new(1isize, 1isize), '5'),
        (Coord::new(0isize, 1isize), '4'),
        (Coord::new(2isize, 2isize), '3'),
        (Coord::new(1isize, 2isize), '2'),
        (Coord::new(0isize, 2isize), '1'),
        (Coord::new(2isize, 3isize), 'A'),
        (Coord::new(1isize, 3isize), '0'),
    ].into_iter().map(|(co, ch)| (ch, co)).collect();

    let directional_pad: HashMap<char, Coord> = vec![
        (Coord::new(2isize, 0isize), 'A'),
        (Coord::new(1isize, 0isize), '^'),
        (Coord::new(2isize, 1isize), '>'),
        (Coord::new(1isize, 1isize), 'v'),
        (Coord::new(0isize, 1isize), '<'),
    ].into_iter().map(|(co, ch)| (ch, co)).collect();

    let mut result = 0;
    for &code in codes.iter() {
        let mut pebble_time: HashMap<String, usize> = vec![
            (get_numpad_robot_inputs(&numpad, code).join(""), 1)
        ].into_iter().collect();

        for _ in 0..mediator_bots {
            let mut next_pebbles: HashMap<String, usize> = HashMap::new();

            for (sequence, count) in pebble_time.into_iter() {
                for subsequence in get_numpad_robot_inputs(&directional_pad, &sequence) {
                    next_pebbles.entry(subsequence)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }

            pebble_time = next_pebbles;
        }
        let length = pebble_time.iter().map(|(sequence, count)| sequence.len() * count).sum::<usize>();
        result += length * &code[..3].parse::<usize>().unwrap();
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day21");
    let input = input.split_whitespace().filter(|s| !s.is_empty()).collect::<Vec<_>>();
    sum_of_complexities_through_n_robots(&input, 2);
    sum_of_complexities_through_n_robots(&input, 25);
}
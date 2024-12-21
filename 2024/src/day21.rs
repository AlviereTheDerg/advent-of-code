
use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

use crate::{Coord, New};

// code -> list of lists of chars, chars are buttons, 
// each inner list is a unique shortest path s.t. all paths go from the same start to the same end
fn get_numpad_robot_inputs(numpad: &HashMap<char, Coord>, code: &str) -> String {
    let neighbours: HashMap<Coord, char> = vec![
        (Coord{x:1,y:0}, '>'), 
        (Coord{x:-1,y:0}, '<'), 
        (Coord{x:0,y:1}, 'v'), 
        (Coord{x:0,y:-1}, '^'),
    ].into_iter().collect();

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
        // if we go down and the corner crossed is present, we can go down first then horizontal
        if diff.y > 0 && numpad.values().any(|coord| coord == &Coord::new(start_coord.x, end_coord.y)) {
            result.push(vert_move + &hori_move + &"A");
        }
        // otherwise try horizontal first then vertical
        else if numpad.values().any(|coord| coord == &Coord::new(end_coord.x, start_coord.y)) {
            result.push(hori_move + &vert_move + &"A");
        }
        // have to go vertical first then horizontal
        else {
            result.push(vert_move + &hori_move + &"A");
        }

        current_key = numpad.iter().filter_map(|(&ch, &co)| if co == end_coord {Some(ch)} else {None}).next().unwrap();
    }

    result.join("")
}

fn part1(codes: &Vec<&str>) {
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
        let flag = false;//code == "379A";
        let pressure_bot = get_numpad_robot_inputs(&numpad, code);
        if flag {println!("{pressure_bot:?}")};
        let radiation_bot = get_numpad_robot_inputs(&directional_pad, &pressure_bot);
        if flag {println!("{radiation_bot}")};
        let cold_bot = get_numpad_robot_inputs(&directional_pad, &radiation_bot);
        if flag {println!("{cold_bot}")};

        let final_code = cold_bot;
        println!("{} * {}", final_code.len(), &code[..3]);
        result += final_code.len() * &code[..3].parse::<usize>().unwrap();
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day21");
    let input = input.split_whitespace().filter(|s| !s.is_empty()).collect::<Vec<_>>();
    part1(&input);
}
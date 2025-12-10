
use regex::Regex;
use std::collections::VecDeque;

fn bfs_step_count(start: i16, goal: i16, buttons: &Vec<i16>) -> i16 {
    let mut search_queue = VecDeque::new();
    search_queue.push_back((start, 0i16, buttons.clone())); // current place, number of steps, buttons that can be pressed
    while let Some((here, steps, buttons)) = search_queue.pop_front() {
        for (index, &available_button) in buttons.iter().enumerate() {
            if here^available_button == goal {return steps + 1;}

            search_queue.push_back((
                here^available_button, 
                steps + 1, 
                buttons.iter().enumerate()
                    .filter_map(|(i, v)| if i != index {Some(*v)} else {None})
                    .collect()
            ))
        }
    }
    0
}

fn part1(machine_data: &Vec<(i16, Vec<i16>)>) {
    // BFS TIME BAYBEE
    let mut result = 0;
    for (start, buttons) in machine_data {
        result += bfs_step_count(0, *start, buttons);
    }
    println!("{}", result);
}

pub fn main() {
    let input = crate::grab_input("day10");
    let desired_result_grabber = Regex::new(r#"^\[([\.#]+)\]"#).unwrap();
    let button_grabber = Regex::new(r#"\(([\d,]+)\)"#).unwrap();
    //let misc_grabber = Regex::new(r#"\{([\d,]+)\}$"#).unwrap();
    
    let mut machine_data: Vec<(i16, Vec<i16>)> = vec![];
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let desired_result_str = desired_result_grabber.captures(line).unwrap().get(1).unwrap().as_str();
        let buttons_str: Vec<_> = button_grabber.captures_iter(line).map(|capture| capture.get(1).unwrap().as_str()).collect();
        //let misc = misc_grabber.captures(line).unwrap().get(1).unwrap().as_str();

        // length of bitstring = length of desired_result
        let bits_string: String = desired_result_str.chars().rev().map(|ch| if ch == '#' {'1'} else {'0'}).collect();
        let desired_result_bits = i16::from_str_radix(&bits_string, 2).unwrap();

        let mut buttons_bits: Vec<i16> = Vec::new();
        for &button in buttons_str.iter() {
            let mut this_button = 0i16;
            for val in button.split(',') {
                this_button |= 2i16.pow(val.parse().unwrap()) as i16;
            }
            buttons_bits.push(this_button);
        }
        
        machine_data.push((desired_result_bits, buttons_bits));
    }

    part1(&machine_data);
}
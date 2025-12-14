
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};

fn bfs_step_count(start: u16, goal: u16, buttons: &Vec<u16>) -> u16 {
    let mut search_queue = VecDeque::new();
    search_queue.push_back((start, 0u16, buttons.clone())); // current place, number of steps, buttons that can be pressed
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

fn part1(lights_data: &Vec<(u16, Vec<u16>)>) {
    // BFS TIME BAYBEE
    let mut result = 0;
    for (start, buttons) in lights_data {
        result += bfs_step_count(0, *start, buttons);
    }
    println!("{}", result);
}

fn all_possible_presses(start: u16, goal: u16, buttons: &Vec<u16>) -> Vec<BTreeSet<usize>> {
    let mut search_queue = VecDeque::new();
    let mut searched_space = HashSet::new();
    let mut finds = Vec::new();
    if start == goal {finds.push(BTreeSet::new());}
    search_queue.push_back((start, BTreeSet::new())); // current place, buttons (by index) already pressed
    while let Some((here, buttons_pressed)) = search_queue.pop_front() {
        for (index, &button) in buttons.iter().enumerate() {
            if buttons_pressed.contains(&index) {continue;}

            let mut this_buttons_pressed = buttons_pressed.clone();
            this_buttons_pressed.insert(index);

            if searched_space.contains(&this_buttons_pressed) {continue;}
            searched_space.insert(this_buttons_pressed.clone());

            if here^button == goal {
                finds.push(this_buttons_pressed.clone());
            }

            search_queue.push_back((
                here^button,
                this_buttons_pressed
            ))
        }
    }
    finds
}

fn minimum_joltage_presses(
    buttons_bits: &Vec<u16>, 
    buttons_ints: &Vec<Vec<u16>>, 
    joltages: Vec<i16>, 
    precalculated_presses: &mut HashMap<u16, Vec<BTreeSet<usize>>>
) -> Option<u16> {
    if joltages.iter().all(|val| *val == 0) {return Some(0);}
    let target: u16 = joltages.iter().enumerate().map(|(index, val)| ((val % 2) as u16) << index).sum();
    let mut lowest = None;

    let possible_presses = 
        if let Some(value) = precalculated_presses.get(&target) {
            value.to_owned()
        } else {
            let buff = all_possible_presses(0, target, buttons_bits);
            println!("Miss. {}. {:?}", target, buff);
            precalculated_presses.insert(target, buff.clone());
            buff
        };
    
    //println!("{:?} -> {:?}", joltages, possible_presses);
    for presses in possible_presses {
        let mut resultant_joltages: Vec<i16> = joltages.clone();
        for press in presses.iter() {
            for index in buttons_ints.get(*press).unwrap() {
                resultant_joltages[*index as usize] -= 1;
            }
        }
        if resultant_joltages.iter().any(|v| v%2 != 0) {println!("uh oh");}
        if resultant_joltages.iter().any(|v| *v < 0) {continue;}
        resultant_joltages = resultant_joltages.iter().map(|v| v / 2).collect();
        if let Some(result) = minimum_joltage_presses(buttons_bits, buttons_ints, resultant_joltages, precalculated_presses) {
            let result = result * 2 + presses.len() as u16;
            if let Some(current_lowest) = lowest {
                lowest = Some(result.min(current_lowest));
            } else {
                lowest = Some(result);
            }
        }
    }

    lowest
}

fn part2(data: &Vec<(Vec<i16>, Vec<u16>, Vec<Vec<u16>>)>) {
    let mut result = 0;
    for (index, (joltages, button_bits, button_ints)) in data.iter().enumerate() {
        let mut precalculated_presses: HashMap<u16, Vec<BTreeSet<usize>>> = HashMap::new();
        println!("Prepopulating up to {}", 2u16.pow(button_bits.len() as u32));
        for sequence in 0..(2u16.pow(button_bits.len() as u32)) {
            let mut sequence_indexes = BTreeSet::new();
            let mut sequence_result = 0;
            for (index, bits) in button_bits.iter().enumerate() {
                if sequence & (1 << index) == 0 {continue;}
                sequence_indexes.insert(index);
                sequence_result ^= bits;
            }
            precalculated_presses.entry(sequence_result).or_insert(Vec::new()).push(sequence_indexes);
        }

        let buff = minimum_joltage_presses(button_bits, button_ints, joltages.clone(), &mut precalculated_presses).unwrap();
        result += buff;
        println!("Machine {index} solved!");
    }
    println!("{}", result);
}

pub fn main() {
    let input = crate::grab_input("day10");
    let lights = Regex::new(r#"^\[([\.#]+)\]"#).unwrap();
    let button_grabber = Regex::new(r#"\(([\d,]+)\)"#).unwrap();
    let joltage_grabber = Regex::new(r#"\{([\d,]+)\}$"#).unwrap();
    
    let mut lights_data: Vec<(u16, Vec<u16>)> = Vec::new(); // light target, button bits
    let mut joltage_data: Vec<(Vec<i16>, Vec<Vec<u16>>)> = Vec::new(); // joltage target, button ints
    let mut combined_data: Vec<(Vec<i16>, Vec<u16>, Vec<Vec<u16>>)> = Vec::new(); // joltage target, button bits, button ints
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let lights_str = lights.captures(line).unwrap().get(1).unwrap().as_str();
        let buttons_str: Vec<_> = button_grabber.captures_iter(line).map(|capture| capture.get(1).unwrap().as_str()).collect();
        let joltages_str = joltage_grabber.captures(line).unwrap().get(1).unwrap().as_str();

        // length of bitstring = length of desired_result
        let lights_bits = lights_str.chars().enumerate()
            .filter_map(|(index, ch)| if ch == '#' {Some(1 << index)} else {None})
            .sum();

        let mut buttons_bits: Vec<u16> = Vec::new();
        let mut button_ints = Vec::new();
        for &button in buttons_str.iter() {
            let mut this_button_bits = 0u16;
            let mut this_button_ints = Vec::new();
            for val in button.split(',') {
                let this_button_int = val.parse().unwrap();
                this_button_ints.push(this_button_int);
                this_button_bits |= 2i16.pow(this_button_int as u32) as u16;
            }
            button_ints.push(this_button_ints);
            buttons_bits.push(this_button_bits);
        }

        let mut joltages = Vec::new();
        for joltage in joltages_str.split(',') {
            joltages.push(joltage.parse().unwrap());
        }
        
        lights_data.push((lights_bits, buttons_bits.clone()));
        joltage_data.push((joltages.clone(), button_ints.clone()));
        combined_data.push((joltages, buttons_bits, button_ints));
    }

    use std::time::Instant;
    let mut now = Instant::now();
    part1(&lights_data);
    let mut elapsed = now.elapsed();
    println!("{:2?}", elapsed);

    now = Instant::now();
    part2(&combined_data);
    elapsed = now.elapsed();
    println!("{:2?}", elapsed);
}
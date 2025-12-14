
use regex::Regex;
use std::collections::{HashMap, BTreeSet};

fn minimum_joltage_presses(
    buttons_bits: &Vec<u16>, 
    buttons_ints: &Vec<Vec<u16>>, 
    joltages: Vec<i16>, 
    precalculated_presses: &HashMap<u16, Vec<BTreeSet<usize>>>
) -> Option<u16> {
    if joltages.iter().all(|val| *val == 0) {return Some(0);}
    let target: u16 = joltages.iter().enumerate().map(|(index, val)| ((val % 2) as u16) << index).sum();
    let mut lowest = None;

    let possible_presses = 
        if let Some(value) = precalculated_presses.get(&target) {
            value
        } else {
            &vec![]
        };
    
    for presses in possible_presses {
        let mut resultant_joltages: Vec<i16> = joltages.clone();
        for press in presses.iter() {
            for index in buttons_ints.get(*press).unwrap() {
                resultant_joltages[*index as usize] -= 1;
            }
        }
        if resultant_joltages.iter().any(|v| *v < 0) {continue;}

        resultant_joltages = resultant_joltages.iter().map(|v| v / 2).collect();
        if let Some(result) = minimum_joltage_presses(buttons_bits, buttons_ints, resultant_joltages, precalculated_presses) {
            let result = result * 2 + presses.len() as u16;
            lowest = if let Some(current_lowest) = lowest {
                Some(result.min(current_lowest))
            } else {
                Some(result)
            };
        }
    }

    lowest
}

fn combined_processing(data: &Vec<(Vec<i16>, Vec<u16>, Vec<Vec<u16>>, u16)>) {
    let mut result_a = 0;
    let mut result_b = 0;
    for (joltages, button_bits, button_ints, lights) in data {
        let mut precalculated_presses: HashMap<u16, Vec<BTreeSet<usize>>> = HashMap::new();
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

        result_a += precalculated_presses.get(lights).unwrap().iter().map(|seq| seq.len()).min().unwrap();
        result_b += minimum_joltage_presses(button_bits, button_ints, joltages.clone(), &precalculated_presses).unwrap();
    }
    println!("{}", result_a);
    println!("{}", result_b);
}

pub fn main() {
    let input = crate::grab_input("day10");
    let lights = Regex::new(r#"^\[([\.#]+)\]"#).unwrap();
    let button_grabber = Regex::new(r#"\(([\d,]+)\)"#).unwrap();
    let joltage_grabber = Regex::new(r#"\{([\d,]+)\}$"#).unwrap();
    
    let mut combined_data: Vec<(Vec<i16>, Vec<u16>, Vec<Vec<u16>>, u16)> = Vec::new(); // joltage target, button bits, button ints, lights
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

        combined_data.push((joltages, buttons_bits, button_ints, lights_bits));
    }

    use std::time::Instant;
    let now = Instant::now();
    combined_processing(&combined_data);
    let elapsed = now.elapsed();
    println!("{:2?}", elapsed);
}
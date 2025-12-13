
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

fn bfs_joltages(start: &Vec<i16>, buttons: &Vec<Vec<u16>>) -> u16 {
    let mut search_queue = VecDeque::new();
    search_queue.push_back((start.clone(), 0u16)); // current 'place' (joltage ticks remaining), number of steps
    let mut searched = HashMap::new();
    while let Some((here, steps)) = search_queue.pop_front() {
        if let Some(&already) = searched.get(&here) && already <= steps {continue;}
        searched.insert(here.clone(), steps);
        for button in buttons {
            let mut modified_here = here.clone();
            for &button_sub in button {
                modified_here[button_sub as usize] -= 1;
            }
            if modified_here.iter().any(|&v| v < 0) {continue;} // if a joltage-ticks-remaining goes negative
            if modified_here.iter().all(|&v| v == 0) {return steps + 1;} // if we met the goal

            search_queue.push_back((modified_here, steps + 1));
        }
    }
    0
}

fn gcd(a: isize, b: isize) -> isize {
    let (high, low) = (a.max(b), a.min(b));
    if low == 0 {
        high
    } else {
        gcd(low, high % low)
    }
}

fn part2_linalg(joltage_data: &Vec<(Vec<i16>, Vec<Vec<u16>>)>) {
    let mut result = 0;
    for (start, buttons) in joltage_data {
        // construct matrix to be reduced
        let rows = start.len();
        let columns = buttons.len();
        let mut matrix_data = vec![vec![0i16; columns + 1]; rows];
        for (column, button) in buttons.iter().enumerate() {
            for &row in button {
                matrix_data[row as usize][column] = 1;
            }
        }
        for (row, &val) in start.iter().enumerate() {
            matrix_data[row][columns] = val;
        }

        // reduce the matrix, track dependents and independents
        let mut pivot_count = 0;
        let mut column = 0;
        let mut dependents: Vec<usize> = Vec::new();
        let mut independents: Vec<usize> = Vec::new();
        while pivot_count < rows && column < columns {
            // if this column is all 0s, independent, and try to pivot on the next column
            if matrix_data.iter()
                .skip(pivot_count)
                .map(|row| row.get(column).unwrap())
                .all(|val| *val == 0) {
                independents.push(column);
                column += 1;
                continue;
            }

            // pivot around the smallest (absolute value) non-zero value to pivot on
            let (pivot_row, mut pivot_value) = matrix_data.iter().enumerate()
                .skip(pivot_count)
                .map(|(row_index, row_content)| 
                    (row_index, row_content.get(column).unwrap().abs())
                )
                .filter(|(_, val)| *val != 0)
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();
            
            // attempt to normalize row
            let mut divisor = matrix_data.get(pivot_row).unwrap()
                .iter()
                .map(|v| v.abs())
                .fold(pivot_value, |a, b| gcd(a as isize, b as isize) as i16);
            pivot_value /= divisor;
            divisor *= if *matrix_data.get(pivot_row).unwrap().get(column).unwrap() > 0 {1} else {-1};
            for col in 0..(columns+1) {
                *matrix_data.get_mut(pivot_row).unwrap().get_mut(col).unwrap() /= divisor;
            }

            // attempt to remove this value from all other rows
            for row in 0..rows {
                if row == pivot_row {continue;} // don't mul(0) a row

                // avoid floating point bs => factor = current value in pivot row's column / pivot value (integer division -> rounds down)
                let factor = *matrix_data.get(row).unwrap().get(column).unwrap() / pivot_value;

                if factor == 0 {continue;} // no need to do anything to a row with 0 in this entry
                for col in 0..(columns+1) {
                    // this row's val -= factor * pivot row's val
                    *matrix_data.get_mut(row).unwrap().get_mut(col).unwrap() -= factor * matrix_data.get(pivot_row).unwrap().get(col).unwrap()
                }
            }

            // if this row isn't fully reduced BUT a different possible pivot row does exist at a lower value
            if pivot_value != 1 && pivot_value > matrix_data.iter().enumerate()
                .skip(pivot_count)
                .map(|(row_index, row_content)| 
                    (row_index, row_content.get(column).unwrap().abs())
                )
                .filter(|(row, val)| pivot_count < *row && *val != 0)
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(_, b)| b)
                .unwrap()
            {
                continue;
            }

            // swap the pivot row up and mark this column as a dependent
            matrix_data.swap(pivot_count, pivot_row);
            dependents.push(column);

            pivot_count += 1;
            column += 1;
        }

        // all remaining columns are independents
        independents.extend(column..columns);

        println!("{:?}\n{:?}\n{:?}", matrix_data, dependents, independents);
        result += bfs_joltages(start, buttons);
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
    finds.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
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

fn part2_bifurcate(data: &Vec<(Vec<i16>, Vec<u16>, Vec<Vec<u16>>)>) {
    let mut result = 0;
    for (index, (joltages, button_bits, button_ints)) in data.iter().enumerate() {
        //let goal = joltages.iter().enumerate().map(|(index, val)| ((val % 2) as u16) << index).sum();
        //println!("{:?} -> {:?}", joltages, all_possible_presses(0, goal, button_bits));
        let buff = minimum_joltage_presses(button_bits, button_ints, joltages.clone(), &mut HashMap::new()).unwrap();
        result += buff;
        //println!("{buff}");
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
    part2_bifurcate(&combined_data);
    //part2_linalg(&joltage_data);
    elapsed = now.elapsed();
    println!("{:2?}", elapsed);
}
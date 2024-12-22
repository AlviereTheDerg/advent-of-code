
use std::collections::HashMap;

fn next_secret_number(mut secret: isize) -> isize {
    secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    secret = (secret ^ (secret * 2048)) % 16777216;
    secret
}

fn part1(monkey_numbers: &Vec<isize>) {
    let mut result = 0;
    for monkey_number in monkey_numbers {
        let mut monkey_number = *monkey_number;
        for _ in 0..2000 {
            monkey_number = next_secret_number(monkey_number);
        }
        result += monkey_number;
    }
    println!("{result}");
}

fn part2(monkey_numbers: &Vec<isize>) {
    let mut monkey_number_maps: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for monkey in monkey_numbers {
        let mut monkey_numbers = vec![*monkey];
        for _ in 0..2000 {
            monkey_numbers.push(next_secret_number(*monkey_numbers.last().unwrap()));
        }
        monkey_numbers = monkey_numbers.into_iter().map(|v| v % 10).collect();
        let mut this_monkey_map: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
        for window_spot in 0..(monkey_numbers.len()-4) {
            let window = (
                monkey_numbers[window_spot+1] - monkey_numbers[window_spot], 
                monkey_numbers[window_spot+2] - monkey_numbers[window_spot+1], 
                monkey_numbers[window_spot+3] - monkey_numbers[window_spot+2], 
                monkey_numbers[window_spot+4] - monkey_numbers[window_spot+3]
            );
            if !this_monkey_map.contains_key(&window) {
                this_monkey_map.insert(window, monkey_numbers[window_spot+4]);
            }
        }
        for (window, earning) in this_monkey_map.into_iter() {
            monkey_number_maps.entry(window)
                .and_modify(|v| *v += earning)
                .or_insert(earning);
        }
    }
    
    let result = monkey_number_maps.values().max();
    println!("{result:?}");
}

pub fn main() {
    let input = crate::grab_input("day22");
    let starting_numbers: Vec<isize> = input.split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    part1(&starting_numbers);
    part2(&starting_numbers);
}

use std::collections::HashMap;

fn next_secret_number(mut secret: isize) -> isize {
    secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    secret = (secret ^ (secret * 2048)) % 16777216;
    secret
}

fn part1(monkey_numbers: &Vec<Vec<isize>>) {
    let result = monkey_numbers.iter().map(|num| num.last().unwrap()).sum::<isize>();
    println!("{result}");
}

fn part2(monkey_numbers: &Vec<Vec<isize>>) {
    let mut monkey_number_maps: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for secret_numbers in monkey_numbers {
        let monkey_numbers: Vec<isize> = secret_numbers.into_iter().map(|v| v % 10).collect();
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
    
    let result = monkey_number_maps.values().max().unwrap();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day22");

    let monkey_numbers = input.split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut monkey_number = vec![line.parse::<isize>().unwrap()];
            for _ in 0..2000 {
                monkey_number.push(next_secret_number(*monkey_number.last().unwrap()));
            }
            monkey_number
        })
        .collect::<Vec<_>>();

    part1(&monkey_numbers);
    part2(&monkey_numbers);
}
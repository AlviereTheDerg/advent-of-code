
use std::collections::{VecDeque, HashSet};

fn can_towel_be_made(wanted_towel: &str, available_towels: &Vec<String>) -> bool {
    let mut exploration: VecDeque<String> = VecDeque::new();
    let mut explored: HashSet<String> = HashSet::new();
    exploration.push_front(String::from(""));

    while let Some(current_towel) = exploration.pop_front() {
        if explored.contains(&current_towel) {continue;}
        explored.insert(current_towel.clone());
        for available_towel in available_towels.iter() {
            let current_towel = current_towel.clone() + available_towel;
            if current_towel == wanted_towel {return true;}

            if current_towel.len() <= wanted_towel.len() && wanted_towel.starts_with(&current_towel) {
                exploration.push_front(current_towel);
            }
        }
    }

    false
}

fn part1(available_towels: &Vec<String>, wanted_towels: &Vec<String>) {
    let result = wanted_towels.iter()
        .filter(|wanted_towel| can_towel_be_made(wanted_towel, available_towels))
        .count();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day19");
    let mut spliterator = input.split("\n\n");
    let available_towels = spliterator.next().unwrap()
        .split(", ")
        .map(str::to_string)
        .collect::<Vec<_>>();
    let wanted_towels = spliterator.next().unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    part1(&available_towels, &wanted_towels);
}
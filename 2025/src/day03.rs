use std::cmp::max;


fn determine_max_joltage(input: &str) -> u32 {
    let batteries: Vec<_> = input.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

    let mut max_battery = 0;
    for (index, battery_a) in batteries.iter().enumerate() {
        for battery_b in batteries[(index+1)..batteries.len()].iter() {
            max_battery = max(max_battery, battery_a * 10 + battery_b)
        }
    }
    max_battery
}

fn part1(input: &Vec<&str>) {
    let result: u32 = input.iter().map(|row| determine_max_joltage(row)).sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day03");

    let rows: Vec<&str> = input.split_whitespace().collect();

    part1(&rows);
}
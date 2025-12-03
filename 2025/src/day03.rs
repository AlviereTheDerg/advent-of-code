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

fn determine_max_joltage_recur(input: &str, digits_remaining: u32) -> Option<u64> {
    if digits_remaining == 0 {return Some(0)}
    if input.len() < digits_remaining as usize {return None}
    for target in "9876543210".chars() { // uh oh greedy!!
        let this_step = 10u64.pow(digits_remaining - 1) * u64::from(target.to_digit(10).unwrap());
        if let Some((_, split_rem)) = input.split_once(target) {
            if let Some(remainder_value) = determine_max_joltage_recur(split_rem, digits_remaining - 1) {
                return Some(this_step + remainder_value)
            }
        }
    }
    None
}

fn determine_max_joltage_2(input: &str) -> u64 {
    if let Some(val) = determine_max_joltage_recur(input, 12) {
        return val;
    }
    println!("uh oh {input}");
    0
}

fn part2(input: &Vec<&str>) {
    let result: u64 = input.iter().map(|row| determine_max_joltage_2(row)).sum();
    println!("{result}");
}


pub fn main() {
    let input = crate::grab_input("day03");

    let rows: Vec<&str> = input.split_whitespace().collect();

    part1(&rows);
    part2(&rows);
}
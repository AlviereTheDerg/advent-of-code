
use std::collections::HashSet;
use std::ops::{Add, Mul};

fn part1(lines: &Vec<(isize, Vec<isize>)>) {
    let mut result = 0;
    for (goal, operands) in lines {
        let mut found_numbers = HashSet::new();
        for &operand in operands.iter() {
            let mut next_found_numbers = HashSet::new();
            if found_numbers.is_empty() {found_numbers.insert(operand); continue;}

            for number in found_numbers {
                for operator in vec![Add::add, Mul::mul] {
                    next_found_numbers.insert(operator(number, operand));
                }
            }
            found_numbers = next_found_numbers;
        }
        if found_numbers.contains(&goal) {result += goal;}
    }
    println!("{result}");
}

fn concatenate(left: isize, right:isize) -> isize {
    (left.to_string() + &right.to_string()).parse::<isize>().unwrap()
}

fn part2(lines: &Vec<(isize, Vec<isize>)>) {
    let mut result = 0;
    for (goal, operands) in lines {
        let mut found_numbers = HashSet::new();
        for &operand in operands.iter() {
            let mut next_found_numbers = HashSet::new();
            if found_numbers.is_empty() {found_numbers.insert(operand); continue;}

            for number in found_numbers {
                for operator in vec![Add::add, Mul::mul, concatenate] {
                    next_found_numbers.insert(operator(number, operand));
                }
            }
            found_numbers = next_found_numbers;
        }
        if found_numbers.contains(&goal) {result += goal;}
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day07");
    let input = input.split("\n").filter(|s| !s.is_empty()).map(|line| {
        let mut hold = line.split(": ");
        let goal = hold.next().unwrap().parse::<isize>().unwrap();
        let operands = hold.next().unwrap().split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        (
            goal,
            operands,
        )
    }).collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

use std::collections::HashSet;
use std::ops::{Add, Mul};

fn part1(input: &str) {
    let lines = input.split("\n").filter(|s| !s.is_empty()).map(|line| {
        let mut hold = line.split(": ");
        let goal = hold.next().unwrap().parse::<isize>().unwrap();
        let operands = hold.next().unwrap().split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        (
            goal,
            operands,
        )
    }).collect::<Vec<_>>();

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
        if found_numbers.contains(&goal) {result += goal; println!("{goal}: {operands:?}");}
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day07");
    part1(&input);
}
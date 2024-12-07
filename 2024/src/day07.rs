
use std::collections::HashSet;
use std::ops::{Add, Mul};

fn valid_sequence(goal: isize, operands: &Vec<isize>, operators: &Vec<&dyn Fn(isize, isize) -> isize>) -> bool {
    let mut operands = operands.iter();
    let mut found_numbers = HashSet::new(); found_numbers.insert(*operands.next().unwrap());
    for &operand in operands {
        let mut next_found_numbers = HashSet::new();

        for number in found_numbers {
            for operator in operators.iter() {
                // optimization! numbers can only increase, if it's overshot then skip it, do not pass go, do not collect $100
                if operator(number, operand) > goal {continue;}

                next_found_numbers.insert(operator(number, operand));
            }
        }
        found_numbers = next_found_numbers;
    }
    found_numbers.contains(&goal)
}

fn process_valid_sequences(lines: &Vec<(isize, Vec<isize>)>, operators: Vec<&dyn Fn(isize, isize) -> isize>) -> isize {
    lines.iter().filter_map(|(goal, operands)| {
        if valid_sequence(*goal, operands, &operators) {
            Some(goal)
        } else {None}
    }).sum()
}

fn part1(lines: &Vec<(isize, Vec<isize>)>) {
    let result = process_valid_sequences(lines, vec![&Add::add, &Mul::mul]);
    println!("{result}");
}

fn concatenate(left: isize, right:isize) -> isize {
    left * 10isize.pow(right.ilog10() + 1) + right
}

fn part2(lines: &Vec<(isize, Vec<isize>)>) {
    let result = process_valid_sequences(lines, vec![&Add::add, &Mul::mul, &concatenate]);
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

    let timer = std::time::Instant::now();
    part1(&input);
    println!("{}", timer.elapsed().as_millis());

    let timer = std::time::Instant::now();
    part2(&input);
    println!("{}", timer.elapsed().as_millis());
}
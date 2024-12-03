
use itertools::Itertools;

fn is_safe_report(line: &Vec<isize>) -> bool {
    let line: Vec<isize> = line.iter()
        .tuple_windows()
        .map(|(a,b)| (a-b))
        .collect();

    !line.iter().any(|&v| v == 0 || v.abs() > 3)
        &&
    line.iter().map(|v| v.signum()).all_equal()
}

fn part1(input: &Vec<Vec<isize>>) {
    let result = input.iter()
        .filter(|&line| {
            is_safe_report(line)
        })
        .count();
    println!("{result}");
}

fn part2(input: &Vec<Vec<isize>>) {
    let result = input.iter()
        .filter(|&line| {
            is_safe_report(line)
                ||
            line.iter().enumerate()
                .map(|(index, _)| {
                    line.iter().enumerate()
                        .filter_map(|(i, &item)| 
                            if i != index {Some(item)} else {None}
                        ).collect::<Vec<_>>()
                })
                .any(|subline| is_safe_report(&subline))
        })
        .count();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day02");
    let input: Vec<Vec<isize>> = input.split("\n").filter_map(|line| {
            if !line.is_empty() {
                Some(line.split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect())
            } else {
                None
            }
        })
        .collect();

    part1(&input);
    part2(&input);
}

use itertools::Itertools;

fn part1(input: &str) {
    let result = input.split("\n").filter(|&s| !s.is_empty())
        .filter(|&line| {
            let line: Vec<isize> = line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .tuple_windows()
                .map(|(a,b)| (a-b))
                .collect();

            !line.iter().any(|&v| v == 0 || v.abs() > 3)
            &&
            line.iter().map(|v| v.signum()).all_equal()
        })
        .count();
    println!("{result}");
} 

pub fn main() {
    let input = crate::grab_input("day02");
    part1(&input);
}
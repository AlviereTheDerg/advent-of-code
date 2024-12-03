
use regex::Regex;

fn part1(input: &str) {
    let capturer = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).expect("Failure to construct regex");
    
    let mut result = 0;
    for capture in capturer.captures_iter(input) {
        let (_, [b,c]) = capture.extract();
        result += b.parse::<isize>().unwrap() * c.parse::<isize>().unwrap();
    }

    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day03");
    part1(&input);
}
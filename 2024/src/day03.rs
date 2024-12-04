
use regex::Regex;

fn part1(input: &str) {
    let capturer = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).expect("Failure to construct regex");

    let result: isize = capturer.captures_iter(input).map(|capture| {
        let (_, [b, c]) = capture.extract();
        b.parse::<isize>().unwrap() * c.parse::<isize>().unwrap()
    }).sum();

    println!("{result}");
}

fn part2(input: &str) {
    let capturer = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)"#).expect("Failure to construct regex");
    
    let mut result = 0;
    let mut flag = true;
    for capture in capturer.captures_iter(input) {
        if let Some(content) = capture.get(0) {
            match content.as_str() {
                "do()" => flag = true,
                "don't()" => flag = false,
                _ => {
                    if flag {
                        result += capture.get(1).unwrap().as_str().parse::<isize>().unwrap() * capture.get(2).unwrap().as_str().parse::<isize>().unwrap()
                    }
                },
            }
        }
    }

    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day03");
    part1(&input);
    part2(&input);
}
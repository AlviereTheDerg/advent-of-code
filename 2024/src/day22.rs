


fn next_secret_number(mut secret: usize) -> usize {
    secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    secret = (secret ^ (secret * 2048)) % 16777216;
    secret
}

fn part1(monkey_numbers: &Vec<usize>) {
    let mut result = 0;
    for monkey_number in monkey_numbers {
        let mut monkey_number = *monkey_number;
        for _ in 0..2000 {
            monkey_number = next_secret_number(monkey_number);
        }
        result += monkey_number;
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day22");
    let starting_numbers: Vec<usize> = input.split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    part1(&starting_numbers);
}
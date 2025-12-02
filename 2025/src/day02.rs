use itertools::Itertools;

fn check_validity(input: isize) -> bool {
    let length = input.checked_ilog10().unwrap_or(0) + 1;
    if length % 2 == 1 {return true}

    input % 10_isize.pow(length / 2) != input / 10_isize.pow(length / 2)
}

fn part1(input: &Vec<(isize, isize)>) {
    let mut invalid_sum: isize = 0;
    for (a,b) in input {
        for c in *a..=*b {
            if !check_validity(c) {invalid_sum += c};
        }
    }
    println!("{invalid_sum}");
}

fn check_validity_2(input: isize) -> bool {
    let length = input.checked_ilog10().unwrap_or(0) + 1;
    for prime in vec![2, 3, 5, 7, 11] {
        if length % prime != 0 {continue}

        let block_length = length / prime;
        let base = input % 10_isize.pow(block_length);
        let mut flag = true;
        for block in 1..prime {
            if base != (input / 10_isize.pow(block_length * block)) % 10_isize.pow(block_length) {flag = false; break;};
        }
        if flag {return false};
    }
    true
}

fn part2(input: &Vec<(isize, isize)>) {
    let mut invalid_sum: isize = 0;
    for (a,b) in input {
        for c in *a..=*b {
            if !check_validity_2(c) {invalid_sum += c};
        }
    }
    println!("{invalid_sum}");
}


pub fn main() {
    let input = crate::grab_input("day02");
    
    let ranges: Vec<(isize, isize)> = input.split_whitespace().collect::<String>()
        .split(",")
        .map(|range_full| {
            let (a,b) = range_full.split("-").collect_tuple().unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    

    part1(&ranges);
    part2(&ranges);
}
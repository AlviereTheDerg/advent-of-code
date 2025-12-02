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
}
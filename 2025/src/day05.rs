

fn part1(fresh_ranges: &Vec<(usize, usize)>, ingredient_ids: &Vec<usize>) {
    let mut valid_ids = 0;

    for ingredient_id in ingredient_ids {
        for (range_start, range_end) in fresh_ranges {
            if range_start <= ingredient_id && ingredient_id <= range_end {
                valid_ids += 1;
                break;
            }
        }
    }

    println!("{}", valid_ids);
}

pub fn main() {
    let input = crate::grab_input("day05");
    let (fresh_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<(usize, usize)> = fresh_ranges.split_whitespace()
        .map(|row| {
            let (a,b) = row.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }).collect();

    let ingredient_ids: Vec<usize> = ingredient_ids.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    part1(&fresh_ranges, &ingredient_ids);
}
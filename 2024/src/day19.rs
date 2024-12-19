
use std::collections::HashMap;

fn towel_permutations(wanted_towel: &str, available_towels: &Vec<&str>, memoizer: &mut HashMap<String, usize>) -> usize {
    if wanted_towel == "" {return 1;}

    let mut founds = 0;
    for &available_towel in available_towels.iter() {
        if !wanted_towel.starts_with(available_towel) {continue;}

        let next_towel = &wanted_towel[available_towel.len()..];
        if let Some(amount) = memoizer.get(next_towel) {founds += amount;}
        else {
            let this_find = towel_permutations(next_towel, available_towels, memoizer);
            memoizer.insert(next_towel.to_string(), this_find);
            founds += this_find;
        }
    }

    founds
}

/*
fn part1(available_towels: &Vec<&str>, wanted_towels: &Vec<&str>) {
    let result = wanted_towels.iter()
        .filter(|wanted_towel| towel_permutations(wanted_towel, available_towels) > 0)
        .count();
    println!("{result}");
}

fn part2(available_towels: &Vec<&str>, wanted_towels: &Vec<&str>) {
    let result: usize = wanted_towels.iter()
        .map(|wanted_towel| towel_permutations(wanted_towel, available_towels))
        .sum();
    println!("{result}");
}
 */

pub fn main() {
    let input = crate::grab_input("day19");
    let mut spliterator = input.split("\n\n");
    let available_towels = spliterator.next().unwrap()
        .split(", ")
        .collect::<Vec<_>>();
    let wanted_towels = spliterator.next().unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut memoizer = HashMap::new();
    let search_results = wanted_towels.iter()
        .map(|wanted_towel| towel_permutations(wanted_towel, &available_towels, &mut memoizer))
        .collect::<Vec<usize>>();

    println!("{}", search_results.iter().filter(|&&v| v > 0usize).count());
    println!("{}", search_results.iter().sum::<usize>());
    //part1(&available_towels, &wanted_towels);
    //part2(&available_towels, &wanted_towels);
}
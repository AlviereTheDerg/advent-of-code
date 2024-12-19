
use std::collections::HashMap;

fn towel_permutations(wanted_towel: &str, available_towels: &Vec<&str>, memoizer: &mut HashMap<String, usize>) -> usize {
    if wanted_towel == "" {return 1;}

    available_towels.iter()
        .map(|&available_towel| 
            if !wanted_towel.starts_with(available_towel) {0}
            else {
                let next_towel = &wanted_towel[available_towel.len()..];
                if let Some(&amount) = memoizer.get(next_towel) {amount}
                else {
                    let this_find = towel_permutations(next_towel, available_towels, memoizer);
                    memoizer.insert(next_towel.to_string(), this_find);
                    this_find
                }
            }
        )
        .sum()
}

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
}

use std::collections::HashMap;

fn part1(list1: &Vec<isize>, list2: &Vec<isize>) {
    let result: isize = list1.iter().zip(list2.iter()).map(|(a,b)| (a-b).abs()).sum();
    println!("{result}");
}

fn count_occurrences(list: &Vec<isize>) -> HashMap<isize, isize> {
    let mut result = HashMap::new();

    for item in list {
        if !result.contains_key(item) {
            result.insert(*item, list.iter().filter(|&i| i == item).count() as isize);
        }
    }

    result
}

fn part2(list1: &Vec<isize>, list2: &Vec<isize>) {
    let right = count_occurrences(list2);

    let mut result: isize = 0;
    for (id, count) in count_occurrences(list1).iter() {
        result += id * count * right.get(id).unwrap_or(&0);
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day01");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for (index, entry) in input.split_whitespace().enumerate() {
        if index % 2 == 0 {&mut list1} else {&mut list2}
            .push(entry.parse::<isize>().unwrap());
    }
    list1.sort(); list2.sort();

    part1(&list1, &list2);
    part2(&list1, &list2);
}
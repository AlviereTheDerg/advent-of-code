
use std::collections::HashSet;

fn order(order_rules: &Vec<Vec<isize>>, print: &Vec<isize>) -> bool {
    let mut pre_printed: HashSet<&isize> = HashSet::new();
    for page in print.iter() {
        pre_printed.insert(page);

        for pre_page in order_rules.iter().filter(|rule| rule.get(0).unwrap() == page) {
            if pre_printed.contains(pre_page.get(1).unwrap()) {return false;};
        }
    }

    true
}

fn part1(input: &str) {
    let mut spliterator = input.split("\n\n");
    let order_rules = spliterator.next().unwrap();
    let prints = spliterator.next().unwrap();

    let order_rules: Vec<Vec<isize>> = order_rules.split_whitespace().filter_map(|line|
        if !line.is_empty() {
            Some(line.split("|").map(|v| v.parse().unwrap()).collect())
        } else {None}
    ).collect();

    let prints: Vec<Vec<isize>> = prints.split_whitespace().filter_map(|line| 
        if !line.is_empty() {
            Some(line.split(",").map(|v| v.parse().unwrap()).collect())
        } else {None}
    ).collect();

    let mut result = 0;
    for print in prints.iter() {
        if order(&order_rules, print) {result += print.get(print.len() / 2).unwrap();}
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day05");
    part1(&input);
}
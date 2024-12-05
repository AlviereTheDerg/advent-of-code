
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

fn get_middle(print: &Vec<isize>) -> &isize {
    print.get(print.len() / 2).unwrap()
}

fn part1(order_rules: &Vec<Vec<isize>>, prints: &Vec<Vec<isize>>) {
    let mut result = 0;
    for print in prints.iter() {
        if order(&order_rules, print) {result += get_middle(print);}
    }
    println!("{result}");
}

fn force_order(order_rules: &Vec<Vec<isize>>, print: &Vec<isize>) -> Vec<isize> {
    let mut result = Vec::new();

    // edges :D
    let mut order_rules: Vec<&Vec<isize>> = order_rules.iter().filter(|v| print.contains(v.get(0).unwrap()) && print.contains(v.get(1).unwrap())).collect();

    // nodes with no incoming edges
    let mut S: Vec<&isize> = print.iter().filter(|&n| 
            order_rules.iter().filter(|&v| 
                v.get(1).unwrap() == n)
                .count() == 0
        ).collect();
    while !S.is_empty() {
        let n = *S.pop().unwrap();
        result.push(n);
        order_rules = order_rules.iter().filter_map(|&v| if n != *v.get(0).unwrap() {Some(v)} else {None}).collect::<Vec<&Vec<isize>>>();
        S.extend(print.iter().filter(|i| !result.contains(i)).filter(|&n| order_rules.iter().filter(|&v| 
            v.get(1).unwrap() == n)
            .count() == 0));
    }

    result
}

fn part2(order_rules: &Vec<Vec<isize>>, prints: &Vec<Vec<isize>>) {
    let mut result = 0;
    for print in prints.iter() {
        if !order(order_rules, print) {
            result += get_middle(&force_order(order_rules, print));
        }
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day05");
    
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

    part1(&order_rules, &prints);
    part2(&order_rules, &prints);
}
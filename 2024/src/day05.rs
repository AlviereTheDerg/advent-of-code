
use std::collections::HashSet;

fn order(order_rules: &Vec<(isize, isize)>, print: &Vec<isize>) -> bool {
    let mut pre_printed: HashSet<&isize> = HashSet::new();
    for page in print.iter() {
        pre_printed.insert(page);

        for pre_page in order_rules.iter().filter(|rule| rule.0 == *page) {
            if pre_printed.contains(&pre_page.1) {return false;};
        }
    }
    true
}

fn get_middle(print: &Vec<isize>) -> &isize {
    print.get(print.len() / 2).unwrap()
}

fn part1(order_rules: &Vec<(isize, isize)>, prints: &Vec<Vec<isize>>) {
    let result: isize = prints.iter().filter(|print| order(&order_rules, print)).map(get_middle).sum();
    println!("{result}");
}

fn topological_sort(order_rules: &Vec<(isize, isize)>, print: &Vec<isize>) -> Vec<isize> {
    let mut result = Vec::new();

    // edges :D
    let mut order_rules: Vec<&(isize, isize)> = order_rules.iter().filter(|v| print.contains(&v.0) && print.contains(&v.1)).collect();

    // nodes with no incoming edges
    let mut free_branches: Vec<&isize> = print.iter().filter(|&n| 
            order_rules.iter().filter(|&v| 
                v.1 == *n)
                .count() == 0
        ).collect();

    while !free_branches.is_empty() {
        let n = *free_branches.pop().unwrap();
        result.push(n);

        order_rules.retain(|&v| n != v.0);

        free_branches.extend(print.iter()
                .filter(|i| !result.contains(i))
                .filter(|&n| 
                    order_rules.iter().filter(|&v| v.1 == *n).count() == 0
                )
            );
    }

    result
}

fn part2(order_rules: &Vec<(isize, isize)>, prints: &Vec<Vec<isize>>) {
    let mut result = 0;
    for print in prints.iter() {
        if !order(order_rules, print) {
            result += get_middle(&topological_sort(order_rules, print));
        }
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day05");
    
    let mut spliterator = input.split("\n\n");
    let order_rules = spliterator.next().unwrap();
    let prints = spliterator.next().unwrap();

    let order_rules: Vec<(isize, isize)> = order_rules.split_whitespace().filter_map(|line|
        if !line.is_empty() {
            let mut hold = line.split("|").map(|v| v.parse::<isize>().unwrap());
            Some((hold.next().unwrap(), hold.next().unwrap()))
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